// Copyright © 2018 Bart Massey

//! Provides a server for the TCP portion of an RFC 868
//! compliant time server.

extern crate byteorder;
extern crate chrono;

use byteorder::{BigEndian, WriteBytesExt};
use chrono::naive;

use std::io;
use std::net::TcpListener;

/// Process time requests.
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:37").unwrap();
    let epoch = naive::NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp();

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut now = chrono::Utc::now().timestamp() - epoch;
        stream.write_u32::<BigEndian>(now as u32)?;
    }
    Ok(())    
}
