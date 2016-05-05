//! The murmur hash is a relatively fast non-cryptographic hash function for platforms with efficient multiplication.
//!
//! This implementation is based on the murmurhash3 variant.
static C1: u32 = 0xcc9e2d51u32;
static C2: u32 = 0x1b873593u32;
static R1: u32 = 15u32;
static R2: u32 = 13u32;
static M: u32 = 5u32;
static N: u32 = 0xe6546b64u32;

pub fn murmur3_32(data: &[u8]) -> u32 {
    murmur3_32_seed(data, 0)
}

pub fn murmur3_32_seed(data: &[u8], seed: u32) -> u32 {
    let mut hash = seed;
    let length = data.len() as u32;

    let n_blocks = length / 4;
    for i in 0 .. n_blocks {
        let mut k = get_u32(&data[(i * 4) as usize..]);
        k = k.wrapping_mul(C1);
        k = (k << R1) | (k >> (32 - R1));
        k = k.wrapping_mul(C2);

        hash ^= k;
        hash = ((hash << R2) | (hash >> (32 - R2))).wrapping_mul(M).wrapping_add(N);
    }

    let tail = &data[(n_blocks * 4) as usize..];
    let remainder = length & 3;
    let mut k1 = 0u32;

    if remainder == 3 {
        k1 ^= (tail[2] as u32) << 16;
    }

    if remainder >= 2 {
        k1 ^= (tail[1] as u32) << 8
    }

    if remainder >= 1 {
        k1 ^= tail[0] as u32;

        k1 = k1.wrapping_mul(C1);
        k1 = (k1 << R1) | (k1 >> (32 - R1));
        k1 = k1.wrapping_mul(C2);
        hash ^= k1;
    }

    hash ^= length;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    hash
}

fn get_u32(data: &[u8]) -> u32 {
    ((0xff & (data[3] as u32)) << 24) |
    ((0xff & (data[2] as u32)) << 16) |
    ((0xff & (data[1] as u32)) << 8) |
    (0xff & (data[0] as u32))
}

#[test]
fn basic_tests() {
    assert_eq!(0, murmur3_32("".as_bytes()));
    assert_eq!(3530670207, murmur3_32("0".as_bytes()));
    assert_eq!(1642882560, murmur3_32("01".as_bytes()));
    assert_eq!(3966566284, murmur3_32("012".as_bytes()));
    assert_eq!(3558446240, murmur3_32("0123".as_bytes()));
    assert_eq!(433070448, murmur3_32("01234".as_bytes()));
    assert_eq!(1364076727, murmur3_32_seed("".as_bytes(), 1));
    assert_eq!(2832214938, murmur3_32("I will not buy this record, it is scratched.".as_bytes()));
}
