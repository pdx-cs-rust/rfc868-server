// Copyright © 2018 Bart Massey

//! Provides a server for the TCP portion of an RFC 868
//! compliant time server.

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt};

use std::io;
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

/// Process time requests.
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:37").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("cannot get time since epoch")
            .as_secs() as u32;
        // XXX Adjust time since 1970 to produce time since 1900.
        now += 2_208_988_800;
        stream.write_u32::<BigEndian>(now)?;
    }
    Ok(())    
}
