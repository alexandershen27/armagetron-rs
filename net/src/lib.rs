use serde::{Serialize, de::DeserializeOwned};
use std::io::{BufRead, Result as IoResult, prelude::*};
use std::net::TcpStream;

pub fn send<T: Serialize>(stream: &mut TcpStream, message: &T) -> IoResult<()> {
    serde_json::to_writer(&mut *stream, &message)?;
    stream.write_all(b"\n")?;
    Ok(())
}

pub fn recv<T: DeserializeOwned>(reader: &mut impl BufRead) -> IoResult<Option<T>> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Ok(None);
    }
    Ok(Some(serde_json::from_str(&line)?))
}
