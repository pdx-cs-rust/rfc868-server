// Copyright © 2018 Bart Massey

//! Provides a server for the TCP portion of an RFC 868
//! compliant time server.

extern crate byteorder;
extern crate chrono;
#[macro_use] extern crate lazy_static;

use byteorder::{BigEndian, WriteBytesExt};
use chrono::naive;

use std::net::TcpListener;
use std::thread;

lazy_static! {
    static ref EPOCH: i64 = naive::NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp();
}

/// Process time requests.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:37").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream
                .expect("could not start stream");
            let now = chrono::Utc::now().timestamp() - *EPOCH;
            stream.write_u32::<BigEndian>(now as u32)
                .expect("could not write to stream");
        });
    }
}
