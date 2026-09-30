//! Small helpers shared by the transports.

use std::fmt::Write as _;
use std::io;
use std::path::{Path, PathBuf};

/// Longest Unix socket path the platform accepts, `sun_path` minus its
/// terminating NUL.
pub(crate) fn max_socket_path_len() -> usize {
    if cfg!(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "dragonfly"
    )) {
        103
    } else {
        107
    }
}

/// Compares two byte strings without stopping at the first difference. The
/// work depends on the length of `secret` only.
pub(crate) fn constant_time_eq(secret: &[u8], candidate: &[u8]) -> bool {
    let mut diff = usize::from(secret.len() != candidate.len());
    for (i, byte) in secret.iter().enumerate() {
        diff |= usize::from(*byte ^ candidate.get(i).copied().unwrap_or(0));
    }
    diff == 0
}

/// Returns `n` random bytes as lower case hex.
pub(crate) fn random_hex(n: usize) -> io::Result<String> {
    let mut bytes = vec![0u8; n];
    getrandom::fill(&mut bytes).map_err(|e| io::Error::other(format!("random bytes: {e}")))?;
    let mut out = String::with_capacity(n * 2);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    Ok(out)
}

/// Creates a private directory (mode 0700 on Unix) named `<prefix><random>`
/// under `base`.
pub(crate) fn create_private_dir(base: &Path, prefix: &str) -> io::Result<PathBuf> {
    for _ in 0..16 {
        let dir = base.join(format!("{prefix}{}", random_hex(6)?));
        #[allow(unused_mut)]
        let mut builder = std::fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        match builder.create(&dir) {
            Ok(()) => return Ok(dir),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::other("could not create a private directory"))
}

/// Removes a stale socket file left behind by a crashed predecessor. Other
/// kinds of file are left alone.
pub(crate) fn remove_stale_socket(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileTypeExt;
        if let Ok(meta) = std::fs::symlink_metadata(path) {
            if meta.file_type().is_socket() {
                std::fs::remove_file(path)?;
            }
        }
    }
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

/// Restricts a socket to the user of the process.
pub(crate) fn restrict_permissions(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_time_eq_compares_values() {
        assert!(constant_time_eq(b"secret", b"secret"));
        assert!(!constant_time_eq(b"secret", b"secreT"));
        assert!(!constant_time_eq(b"secret", b"secre"));
        assert!(!constant_time_eq(b"secret", b"secrets"));
        assert!(!constant_time_eq(b"secret", b""));
        assert!(!constant_time_eq(b"", b"x"));
    }

    #[test]
    fn random_hex_has_the_requested_length() {
        let a = random_hex(32).unwrap();
        assert_eq!(a.len(), 64);
        assert_ne!(a, random_hex(32).unwrap());
    }
}
