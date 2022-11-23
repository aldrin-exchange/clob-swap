use safe_transmute::{ErrorReason, GuardError, Error, transmute_bool_pedantic};


#[test]
fn too_short() {
    assert_eq!(transmute_bool_pedantic([].as_ref()),
               Err(Error::Guard(GuardError {
                   required: 1,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(transmute_bool_pedantic([0x00, 0x01].as_ref()), Ok(&[false, true][..]));
    assert_eq!(transmute_bool_pedantic([0x01, 0x01, 0x00, 0x01].as_ref()),
               Ok(&[true, true, false, true][..]));
}

#[test]
fn invalid_bytes() {
    assert_eq!(transmute_bool_pedantic([0x00, 0x01, 0x02].as_ref()), Err(Error::InvalidValue));
    assert_eq!(transmute_bool_pedantic([0x05, 0x01, 0x00].as_ref()), Err(Error::InvalidValue));
    assert_eq!(transmute_bool_pedantic([0xFF].as_ref()), Err(Error::InvalidValue));
}
