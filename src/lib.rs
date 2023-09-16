#![allow(clippy::all)]
#![allow(warnings)]

pub mod flic2 {
    include!(concat!(env!("OUT_DIR"), "/flic2.rs"));
}

pub mod flic2_crypto {
    include!(concat!(env!("OUT_DIR"), "/flic2_crypto.rs"));
}

pub mod flic2_packets {
    include!(concat!(env!("OUT_DIR"), "/flic2_packets.rs"));
}
