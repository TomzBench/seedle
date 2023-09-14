use minicbor::Encoder;
use seedle_derive::seedle;
use seedle_extra::infallible_encoder::InfallibleEncoder;
use std::ffi::c_void;

#[seedle(file = "examples/test.cddl", language = "c")]
pub mod c {}

fn make_byte_str<const N: usize>(s: &str) -> [u8; N] {
    let mut ret = [0; N];
    let min = std::cmp::min(s.len(), N);
    ret[0..min].copy_from_slice(&s.as_bytes()[0..min]);
    ret[min..].fill(0);
    ret
}

fn make_netw() -> c::network {
    c::network {
        dhcp: true,
        ip: make_byte_str("192.168.168.1"),
        sn: make_byte_str("255.255.255.0"),
        gw: make_byte_str("192.168.168.0"),
        mac: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
        ids: [1, 2],
        ..Default::default()
    }
}

#[test]
fn test_enum() {
    assert_eq!(c::KEY::NETWORK as u8, 0);
    assert_eq!(c::KEY::PORT as u8, 1);
    assert_eq!(c::KEY::THING as u8, 2);
}

#[test]
fn test_literals() {
    assert_eq!(c::GROUPA_LITERAL_CHAR, 'C');
    assert_eq!(c::GROUPA_LITERAL_THREE, 3);
    assert_eq!(c::GROUPB_LITERAL_FOUR, 4);
}

#[test]
fn test_encode_decode() {
    let mut buff = [0; 4096];
    let mut decoded = std::mem::MaybeUninit::<c::network>::uninit();
    let netw = make_netw();
    let cap = c::len(c::KEY::NETWORK, &netw as *const c::network as *const c_void);
    let ret_encode = c::encode(
        buff.as_mut_ptr(),
        cap as u32,
        c::KEY::NETWORK,
        &netw as *const c::network as *const c_void,
    );
    let mut encoder = Encoder::new(InfallibleEncoder::new(cap as usize));
    encoder.encode(&netw).unwrap();
    let ret_decode = c::decode(
        decoded.as_mut_ptr() as *mut c_void,
        c::KEY::NETWORK,
        buff.as_ptr(),
        cap,
    );
    let decoded = unsafe { decoded.assume_init_mut() };
    assert_eq!(cap as i32, ret_encode);
    assert_eq!(cap as i32, ret_decode);
    assert_eq!(encoder.into_writer().into_inner(), buff[0..cap as usize]);
    assert_eq!(*decoded, netw);
}

#[test]
fn test_encode_decode_array() {
    let mut buff = [0; 4096];
    let netw = [make_netw(), make_netw()];
    let mut decoded = std::mem::MaybeUninit::<[c::network; 2]>::uninit();
    let cap = c::array_len(c::KEY::NETWORK, netw.as_ptr() as *const c_void, 2);
    let ret_encode = c::encode_array(
        buff.as_mut_ptr(),
        cap as u32,
        c::KEY::NETWORK,
        netw.as_ptr() as *const c_void,
        2,
    );
    let mut encoder = Encoder::new(InfallibleEncoder::new(cap as usize));
    encoder.encode(&netw).unwrap();
    let ret_decode = c::decode_array(
        decoded.as_mut_ptr() as *mut c_void,
        2,
        c::KEY::NETWORK,
        buff.as_ptr(),
        cap,
    );
    let decoded = unsafe { decoded.assume_init_mut() };
    assert_eq!(cap as i32, ret_encode);
    assert_eq!(cap as i32, ret_decode);
    assert_eq!(encoder.into_writer().into_inner(), buff[0..cap as usize]);
    assert_eq!(*decoded, netw);
}
