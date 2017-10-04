use safe_transmute::{ErrorReason, Error, guarded_transmute_many_pedantic};


#[test]
fn too_short() {
    unsafe {
        assert_eq!(guarded_transmute_many_pedantic::<u16>(&[]),
                   Err(Error {
                       required: 16 / 8,
                       actual: 0,
                       reason: ErrorReason::NotEnoughBytes,
                   }));
        assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00]),
                   Err(Error {
                       required: 16 / 8,
                       actual: 1,
                       reason: ErrorReason::NotEnoughBytes,
                   }));
    }
}

#[test]
fn just_enough() {
    unsafe {
        assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00, 0x01]), Ok([0x0100u16].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00, 0x01, 0x00, 0x02]),
                   Ok([0x0100u16, 0x0200u16].into_iter().as_slice()));
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00, 0x01, 0x00]),
                   Err(Error {
                       required: 16 / 8,
                       actual: 3,
                       reason: ErrorReason::InexactByteCount,
                   }));
        assert_eq!(guarded_transmute_many_pedantic::<u16>(&[0x00, 0x01, 0x00, 0x02, 0x00]),
                   Err(Error {
                       required: 16 / 8,
                       actual: 5,
                       reason: ErrorReason::InexactByteCount,
                   }));
    }
}