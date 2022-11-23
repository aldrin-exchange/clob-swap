use safe_transmute::{ErrorReason, GuardError, Error, transmute_to_bytes, transmute_one};


#[test]
fn too_short() {
    assert_eq!(transmute_one::<u32>(transmute_to_bytes::<u32>(&[])),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
    assert_eq!(transmute_one::<u32>(&transmute_to_bytes::<u32>(&[0])[..1]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    let words: &[u32] = &[0x1234_5678];
    assert_eq!(transmute_one::<u32>(transmute_to_bytes(words)), Ok(words[0]));
}

#[test]
fn too_much() {
    let words: &[u32] = &[0x0100_0000, 0, 0];
    let bytes = transmute_to_bytes(words);
    assert_eq!(transmute_one::<u32>(&bytes[..5]), Ok(0x0100_0000));
    assert_eq!(transmute_one::<u32>(&bytes[..6]), Ok(0x0100_0000));
    assert_eq!(transmute_one::<u32>(&bytes[..7]), Ok(0x0100_0000));
    assert_eq!(transmute_one::<u32>(&bytes[..8]), Ok(0x0100_0000));
    assert_eq!(transmute_one::<u32>(&bytes[..9]), Ok(0x0100_0000));
}
