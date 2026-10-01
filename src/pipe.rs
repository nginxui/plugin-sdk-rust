//! Named pipes, where both transports listen on Windows.

use crate::transport::Listener;

/// Starts the name of every named pipe the SDK opens.
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) const PIPE_PREFIX: &str = r"\\.\pipe\nginx-ui-plugin-";

/// Returns a pipe name nobody can guess, so no other process can create it
/// first.
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) fn new_pipe_name() -> std::io::Result<String> {
    Ok(format!("{PIPE_PREFIX}{}", crate::util::random_hex(16)?))
}

/// Opens a named pipe under a fresh name and returns it with that name.
#[cfg(windows)]
pub(crate) fn listen() -> Result<(Listener, String), String> {
    let listener =
        windows::PipeListener::bind().map_err(|e| format!("listen on a named pipe: {e}"))?;
    let name = listener.name().to_owned();
    Ok((Listener::Pipe(listener), name))
}

/// Fails: named pipes exist only on Windows.
#[cfg(not(windows))]
pub(crate) fn listen() -> Result<(Listener, String), String> {
    Err("named pipes are only available on Windows".to_owned())
}

#[cfg(windows)]
pub(crate) use windows::PipeListener;

#[cfg(windows)]
#[allow(unsafe_code)]
mod windows {
    use std::ffi::c_void;
    use std::io;
    use std::ptr::null_mut;

    use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};
    use tokio::sync::Mutex;
    use windows_sys::Win32::Foundation::{CloseHandle, LocalFree, HANDLE};
    use windows_sys::Win32::Security::Authorization::{
        ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW,
        SDDL_REVISION_1,
    };
    use windows_sys::Win32::Security::{
        GetTokenInformation, TokenUser, PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES, TOKEN_QUERY,
        TOKEN_USER,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    /// A named pipe that hands out one connected instance per accept. The
    /// next instance exists before a connected one is handed out, so the
    /// name never disappears while the listener lives.
    pub(crate) struct PipeListener {
        name: String,
        next: Mutex<NamedPipeServer>,
    }

    impl PipeListener {
        pub(crate) fn bind() -> io::Result<PipeListener> {
            let name = super::new_pipe_name()?;
            let first = create_first(&name)?;
            Ok(PipeListener {
                name,
                next: Mutex::new(first),
            })
        }

        pub(crate) fn name(&self) -> &str {
            &self.name
        }

        /// Waits for a client. Cancelling it keeps the waiting instance.
        pub(crate) async fn accept(&self) -> io::Result<NamedPipeServer> {
            let mut next = self.next.lock().await;
            next.connect().await?;
            let fresh = ServerOptions::new().create(&self.name)?;
            Ok(std::mem::replace(&mut *next, fresh))
        }
    }

    /// Creates the first instance, which fails when the name already exists.
    /// Only the user the plugin runs as, the user of the host, may connect,
    /// and clients on other machines are refused. Later instances share the
    /// security of the first.
    fn create_first(name: &str) -> io::Result<NamedPipeServer> {
        let sddl = format!("D:P(A;;GA;;;{})", current_user_sid()?);
        let descriptor = SecurityDescriptor::from_sddl(&sddl)?;
        let mut attributes = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: descriptor.0,
            bInheritHandle: 0,
        };
        let mut options = ServerOptions::new();
        options
            .first_pipe_instance(true)
            .reject_remote_clients(true);
        // SAFETY: attributes points to a valid SECURITY_ATTRIBUTES whose
        // descriptor outlives the call.
        unsafe {
            options.create_with_security_attributes_raw(
                name,
                std::ptr::addr_of_mut!(attributes).cast::<c_void>(),
            )
        }
    }

    /// A security descriptor allocated by Windows, freed on drop.
    struct SecurityDescriptor(PSECURITY_DESCRIPTOR);

    impl SecurityDescriptor {
        fn from_sddl(sddl: &str) -> io::Result<SecurityDescriptor> {
            let wide: Vec<u16> = sddl.encode_utf16().chain(Some(0)).collect();
            let mut descriptor: PSECURITY_DESCRIPTOR = null_mut();
            // SAFETY: wide is NUL terminated and descriptor receives a
            // LocalAlloc'd pointer that Drop frees.
            let ok = unsafe {
                ConvertStringSecurityDescriptorToSecurityDescriptorW(
                    wide.as_ptr(),
                    SDDL_REVISION_1,
                    &mut descriptor,
                    null_mut(),
                )
            };
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(SecurityDescriptor(descriptor))
        }
    }

    impl Drop for SecurityDescriptor {
        fn drop(&mut self) {
            // SAFETY: the pointer came from LocalAlloc and is freed once.
            unsafe { LocalFree(self.0) };
        }
    }

    /// The SID of the user the process runs as, such as `S-1-5-21-...`.
    fn current_user_sid() -> io::Result<String> {
        let mut token: HANDLE = null_mut();
        // SAFETY: GetCurrentProcess returns a pseudo handle and token receives
        // a handle that is closed below.
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let result = token_user_sid(token);
        // SAFETY: token is a valid handle owned here.
        unsafe { CloseHandle(token) };
        result
    }

    fn token_user_sid(token: HANDLE) -> io::Result<String> {
        let mut len = 0u32;
        // SAFETY: a null buffer asks for the required length only.
        unsafe { GetTokenInformation(token, TokenUser, null_mut(), 0, &mut len) };
        if len == 0 {
            return Err(io::Error::last_os_error());
        }
        // u64 elements keep the buffer aligned for TOKEN_USER.
        let mut buf = vec![0u64; (len as usize).div_ceil(8)];
        // SAFETY: buf holds at least len bytes.
        let ok = unsafe {
            GetTokenInformation(token, TokenUser, buf.as_mut_ptr().cast(), len, &mut len)
        };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: GetTokenInformation filled buf with a TOKEN_USER.
        let user = unsafe { &*buf.as_ptr().cast::<TOKEN_USER>() };

        let mut wide = null_mut();
        // SAFETY: the SID points into buf, which lives until the end of this
        // function, and wide receives a LocalAlloc'd string freed below.
        if unsafe { ConvertSidToStringSidW(user.User.Sid, &mut wide) } == 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: wide is a NUL terminated UTF-16 string.
        let sid = unsafe {
            let len = (0..).take_while(|&i| *wide.add(i) != 0).count();
            String::from_utf16_lossy(std::slice::from_raw_parts(wide, len))
        };
        // SAFETY: wide came from LocalAlloc and is freed once.
        unsafe { LocalFree(wide.cast()) };
        Ok(sid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipe_names_are_local_and_unique() {
        // The host accepts only a local pipe whose name starts with a letter
        // or digit and uses letters, digits, dots, dashes and underscores.
        let a = new_pipe_name().unwrap();
        let b = new_pipe_name().unwrap();
        assert_ne!(a, b);
        for name in [&a, &b] {
            let rest = name.strip_prefix(r"\\.\pipe\").unwrap();
            assert!(rest.starts_with(|c: char| c.is_ascii_alphanumeric()));
            assert!(rest
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')));
            assert_eq!(name.len(), PIPE_PREFIX.len() + 32);
        }
    }

    #[cfg(not(windows))]
    #[test]
    fn pipes_need_windows() {
        assert!(listen().is_err());
    }
}
