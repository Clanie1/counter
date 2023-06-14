#![no_std]

use gstd::{msg, prelude::*,ActorId};
// use parity_scale_codec::{Encode, Decode};


#[derive(TypeInfo, Decode, Encode, Clone)]
pub struct Wallet {
    pub id: i32,
    pub person: String,
}

// declare and initialize the state
static mut WALLETS: Vec<Wallet> = Vec::new();
static mut COUNTER: i32 = 0;

#[no_mangle]
extern "C" fn handle() {
    let command = msg::load_bytes().expect("Invalid message");

    let mut counter = unsafe { COUNTER };
    let mut wallets = unsafe { WALLETS };

    match command.as_slice() {
        b"inc" => {
            counter += 1;
            wallets.push(Wallet {
                id: 0,
                person: "Daniel".to_string(),
            });
        },
        b"dec" => counter -= 1,
        b"get" => {
            msg::reply_bytes(format!("{counter}"), 0).expect("Unable to reply");
        }
        _ => (),
    }
    unsafe { WALLETS = wallets };
    unsafe { COUNTER = counter };
}

#[no_mangle]
extern "C" fn state(){
    msg::reply(unsafe { WALLETS.clone() }, 0).expect("Failed to share state");
}