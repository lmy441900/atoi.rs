#![feature(assert_matches, default_alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use alloc::{boxed::Box, string::String};
use atoi::comm::http::DummyHttpClient;
use atoi::types::Node;
use atoi::Client;
use core::assert_matches::assert_matches;
#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[cfg_attr(not(feature = "std"), panic_handler)]
#[cfg(not(feature = "std"))]
fn panic(_info: &PanicInfo) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();

    #[allow(clippy::empty_loop)]
    loop {}
}

fn main() {
    let client = Client::builder()
        .node(Node {
            url: String::from("https://example.iota.org"),
            auth: None,
        })
        .http(Box::new(DummyHttpClient::new()))
        .build();

    let health = client.health();
    let info = client.info();

    assert_matches!(health, Ok(()));
    assert_matches!(info, Ok(_));
}
