use minicbor::Encoder;
use seedle_derive::seedle;
use seedle_extra::infallible_encoder::InfallibleEncoder;
use std::ffi::c_void;

#[seedle(file = "examples/test.cddl", language = "c")]
pub mod cddl {}

fn make_byte_str<const N: usize>(s: &str) -> [u8; N] {
    let mut ret = [0; N];
    let min = std::cmp::min(s.len(), N);
    ret[0..min].copy_from_slice(&s.as_bytes()[0..min]);
    ret[min..].fill(0);
    ret
}

fn make_netw() -> cddl::network {
    cddl::network {
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
    assert_eq!(cddl::CDDL::NETWORK as u8, 0);
    assert_eq!(cddl::CDDL::PORT as u8, 1);
    assert_eq!(cddl::CDDL::THING as u8, 2);
}

#[test]
fn test_literals() {
    assert_eq!(cddl::GROUPA_LITERAL_CHAR, 'C');
    assert_eq!(cddl::GROUPA_LITERAL_THREE, 3);
    assert_eq!(cddl::GROUPB_LITERAL_FOUR, 4);
}

#[test]
fn test_encode_decode() {
    let mut buff = [0; 4096];
    let mut decoded = std::mem::MaybeUninit::<cddl::network>::uninit();
    let netw = make_netw();
    let cap = cddl::len(
        cddl::CDDL::NETWORK,
        &netw as *const cddl::network as *const c_void,
    );
    let ret_encode = cddl::encode(
        buff.as_mut_ptr(),
        cap as u32,
        cddl::CDDL::NETWORK,
        &netw as *const cddl::network as *const c_void,
    );
    let mut encoder = Encoder::new(InfallibleEncoder::new(cap as usize));
    encoder.encode(&netw).unwrap();
    let ret_decode = cddl::decode(
        decoded.as_mut_ptr() as *mut c_void,
        cddl::CDDL::NETWORK,
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
    let mut decoded = std::mem::MaybeUninit::<[cddl::network; 2]>::uninit();
    let cap = cddl::array_len(cddl::CDDL::NETWORK, netw.as_ptr() as *const c_void, 2);
    let ret_encode = cddl::encode_array(
        buff.as_mut_ptr(),
        cap as u32,
        cddl::CDDL::NETWORK,
        netw.as_ptr() as *const c_void,
        2,
    );
    let mut encoder = Encoder::new(InfallibleEncoder::new(cap as usize));
    encoder.encode(&netw).unwrap();
    let ret_decode = cddl::decode_array(
        decoded.as_mut_ptr() as *mut c_void,
        2,
        cddl::CDDL::NETWORK,
        buff.as_ptr(),
        cap,
    );
    let decoded = unsafe { decoded.assume_init_mut() };
    assert_eq!(cap as i32, ret_encode);
    assert_eq!(cap as i32, ret_decode);
    assert_eq!(encoder.into_writer().into_inner(), buff[0..cap as usize]);
    assert_eq!(*decoded, netw);
}
