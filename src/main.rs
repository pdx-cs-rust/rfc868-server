// Copyright © 2018 Bart Massey
// [This work is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


//! Provides a server for the TCP portion of an RFC 868
//! compliant time server.

extern crate byteorder;
extern crate chrono;

use byteorder::{BigEndian, WriteBytesExt};
use chrono::naive;

use std::{net, thread};

/// Unwrap and return, else log a message to stderr and
/// continue.
macro_rules! try_log {
    ($r:expr, $e:expr) => {
        match $r {
            Ok(r) => r,
            Err(e) => {
                eprintln!("rfc868-server: {}: {}", $e, e);
                continue;
            },
        }
    }
}

/// Return the current time in seconds as an offset from the
/// RFC 868 epoch. Note the RFC 868 erratum, which says that
/// the time is actually a `u32`.
fn get_now(epoch: i64) -> Result<u32, &'static str> {
    let now: i64 = chrono::Utc::now().timestamp() - epoch;
    if now < 0 || now > u32::max_value() as i64 {
        return Err("timestamp out of range");
    }
    Ok(now as u32)
}

/// Process TCP time requests.
fn tcp_handler(epoch: i64) -> ! {
    let listener = net::TcpListener::bind("127.0.0.1:37").unwrap();

    // accept connections and process them serially
    loop {
        let stream = listener.accept();
        let (mut stream, _) = try_log!(stream, "could not start stream");
        let now = get_now(epoch);
        let now = try_log!(now, "current time past end of epoch");
        let r = stream.write_u32::<BigEndian>(now);
        try_log!(r, "could not write to stream");
    }
}

/// Process UDP time requests.
fn udp_handler(epoch: i64) -> ! {
    let socket = net::UdpSocket::bind("127.0.0.1:37").unwrap();
    loop {
        let mut buf = [0; 0];
        let s = socket.recv_from(&mut buf);
        let (amt, src) = try_log!(s, "could not read request packet");
        if amt > 0 {
            eprintln!("invalid data in request packet");
        }
        let now = get_now(epoch);
        let now = try_log!(now, "current time past end of epoch");
        let mut buf: Vec<u8> = Vec::with_capacity(4);
        buf.write_u32::<BigEndian>(now)
            .expect("could not create packet");
        let r = socket.send_to(&mut buf, &src);
        try_log!(r, "could not send packet");
    }
}

fn main() {
    let epoch = naive::NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp();
    let tcp = thread::spawn(move || tcp_handler(epoch));
    let _ = thread::spawn(move || udp_handler(epoch));
    tcp.join().unwrap();
    panic!("child exited");
}
