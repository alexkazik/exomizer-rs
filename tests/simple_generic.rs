use crate::common::{DEMO_COMPRESSED_P61_FAST, DEMO_UNCOMPRESSED};
use exomizer::simple::generic::{GenProto, decrunch_exact};

mod common;

#[test]
fn simple_generic() {
    let mut output = [0; DEMO_UNCOMPRESSED.len()];
    if let Err(err) = decrunch_exact(
        GenProto::P61,
        DEMO_COMPRESSED_P61_FAST.iter().copied(),
        &mut output,
    ) {
        panic!("simple_generic: decrunch failed {err}");
    }
    assert_eq!(DEMO_UNCOMPRESSED, &output);
}
