// Copyright © 2018 Bart Massey

//! Provides a server for the TCP portion of an RFC 868
//! compliant time server.

extern crate byteorder;
extern crate chrono;
#[macro_use] extern crate lazy_static;

use byteorder::{BigEndian, WriteBytesExt};
use chrono::naive;

use std::{net, thread};

lazy_static! {
    static ref EPOCH: i64 = naive::NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp();
}

/// Process TCP time requests.
fn tcp_handler() {
    let listener = net::TcpListener::bind("127.0.0.1:37").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream
            .expect("could not start stream");
        let now = chrono::Utc::now().timestamp() - *EPOCH;
        stream.write_u32::<BigEndian>(now as u32)
            .expect("could not write to stream");
    }
}

/// Process UDP time requests.
fn udp_handler() {
    let socket = net::UdpSocket::bind("127.0.0.1:37").unwrap();
    loop {
        let mut buf = [0; 0];
        let (amt, src) = socket.recv_from(&mut buf)
            .expect("bad request packet");
        assert_eq!(amt, 0);
        let now = chrono::Utc::now().timestamp() - *EPOCH;
        let mut buf: Vec<u8> = Vec::with_capacity(4);
        buf.write_u32::<BigEndian>(now as u32)
            .expect("could not create packet");
        socket.send_to(&mut buf, &src)
            .expect("could not send packet");
    }
}

fn main() {
    let tcp_id = thread::spawn(|| tcp_handler());
    let udp_id = thread::spawn(|| udp_handler());
    tcp_id.join().expect("TCP thread failed");
    udp_id.join().expect("UDP thread failed");
}
