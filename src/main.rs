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

/// Return the current time in seconds as an offset from the
/// RFC 868 epoch.
fn get_now(epoch: i64) -> i32 {
    (chrono::Utc::now().timestamp() - epoch) as i32
}

/// Process TCP time requests.
fn tcp_handler(epoch: i64) {
    let listener = net::TcpListener::bind("127.0.0.1:37").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream
            .expect("could not start stream");
        let now = get_now(epoch);
        stream.write_u32::<BigEndian>(now as u32)
            .expect("could not write to stream");
    }
}

/// Process UDP time requests.
fn udp_handler(epoch: i64) {
    let socket = net::UdpSocket::bind("127.0.0.1:37").unwrap();
    loop {
        let mut buf = [0; 0];
        let (amt, src) = socket.recv_from(&mut buf)
            .expect("bad request packet");
        assert_eq!(amt, 0);
        let now = get_now(epoch);
        let mut buf: Vec<u8> = Vec::with_capacity(4);
        buf.write_u32::<BigEndian>(now as u32)
            .expect("could not create packet");
        socket.send_to(&mut buf, &src)
            .expect("could not send packet");
    }
}

fn main() {
    let epoch = naive::NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp();
    let tcp_id = thread::spawn(move || tcp_handler(epoch));
    let udp_id = thread::spawn(move || udp_handler(epoch));
    tcp_id.join().expect("TCP thread failed");
    udp_id.join().expect("UDP thread failed");
}
