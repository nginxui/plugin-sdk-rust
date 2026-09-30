use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// Largest single message accepted or produced, trailing newline included.
pub const MAX_MESSAGE_BYTES: usize = 4 << 20;

/// The JSON-RPC protocol version written on every message.
pub const VERSION: &str = "2.0";

/// One line read from the input.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Line {
    /// A complete line without its newline, possibly empty.
    Data(Vec<u8>),
    /// A line that exceeded [`MAX_MESSAGE_BYTES`]. Its payload was dropped
    /// but the stream is synchronised on the next newline.
    TooLong,
    /// The input ended.
    Eof,
}

/// Reads one NDJSON line, bounding the memory it takes.
pub(crate) async fn read_line<R: AsyncBufRead + Unpin>(
    reader: &mut R,
    max: usize,
) -> std::io::Result<Line> {
    let mut line: Vec<u8> = Vec::new();
    let mut too_long = false;
    let mut seen_any = false;

    loop {
        let (consumed, done) = {
            let chunk = reader.fill_buf().await?;
            if chunk.is_empty() {
                // End of input. A last line without a newline still counts.
                return Ok(if too_long {
                    Line::TooLong
                } else if seen_any {
                    Line::Data(line)
                } else {
                    Line::Eof
                });
            }
            seen_any = true;
            match chunk.iter().position(|b| *b == b'\n') {
                Some(pos) => {
                    if !too_long {
                        // The newline counts toward the limit.
                        if line.len() + pos + 1 > max {
                            too_long = true;
                        } else {
                            line.extend_from_slice(&chunk[..pos]);
                        }
                    }
                    (pos + 1, true)
                }
                None => {
                    if !too_long {
                        if line.len() + chunk.len() >= max {
                            too_long = true;
                            line = Vec::new();
                        } else {
                            line.extend_from_slice(chunk);
                        }
                    }
                    (chunk.len(), false)
                }
            }
        };
        reader.consume(consumed);
        if done {
            return Ok(if too_long {
                Line::TooLong
            } else {
                Line::Data(line)
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn splits_lines_and_reports_eof() {
        let mut input: &[u8] = b"one\ntwo\n\nlast";
        assert_eq!(
            read_line(&mut input, 64).await.unwrap(),
            Line::Data(b"one".to_vec())
        );
        assert_eq!(
            read_line(&mut input, 64).await.unwrap(),
            Line::Data(b"two".to_vec())
        );
        assert_eq!(read_line(&mut input, 64).await.unwrap(), Line::Data(vec![]));
        assert_eq!(
            read_line(&mut input, 64).await.unwrap(),
            Line::Data(b"last".to_vec())
        );
        assert_eq!(read_line(&mut input, 64).await.unwrap(), Line::Eof);
    }

    #[tokio::test]
    async fn drops_an_oversized_line_and_stays_in_sync() {
        let mut data = vec![b'x'; 100];
        data.push(b'\n');
        data.extend_from_slice(b"ok\n");
        let mut input: &[u8] = &data;
        assert_eq!(read_line(&mut input, 16).await.unwrap(), Line::TooLong);
        assert_eq!(
            read_line(&mut input, 16).await.unwrap(),
            Line::Data(b"ok".to_vec())
        );
    }

    #[tokio::test]
    async fn a_line_at_the_limit_fits() {
        let mut data = vec![b'x'; 15];
        data.push(b'\n');
        let mut input: &[u8] = &data;
        assert_eq!(
            read_line(&mut input, 16).await.unwrap(),
            Line::Data(vec![b'x'; 15])
        );
    }
}
