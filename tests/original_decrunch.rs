#![cfg(feature = "alloc")]

use crate::common::{DEMO_COMPRESSED_P61_FAST, DEMO_UNCOMPRESSED};
use exomizer::original::{DecrunchOptions, decrunch_raw};

mod common;

#[test]
fn original_decrunch() {
    let output = unsafe {
        decrunch_raw(
            &mut (),
            DEMO_COMPRESSED_P61_FAST,
            &DecrunchOptions::builder()
                .flags_proto(61)
                .direction_forward(true)
                .write_reverse(true)
                .build()
                .unwrap(),
        )
    };
    assert_eq!(DEMO_UNCOMPRESSED, &output);
}
