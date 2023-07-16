
use rand::RngCore;

// tbh, we copied that one
const F_TABLE: [u8; 256] = [
    0xa3, 0xd7, 0x09, 0x83, 0xf8, 0x48, 0xf6, 0xf4, 0xb3, 0x21, 0x15, 0x78, 0x99, 0xb1, 0xaf, 0xf9,
    0xe7, 0x2d, 0x4d, 0x8a, 0xce, 0x4c, 0xca, 0x2e, 0x52, 0x95, 0xd9, 0x1e, 0x4e, 0x38, 0x44, 0x28,
    0x0a, 0xdf, 0x02, 0xa0, 0x17, 0xf1, 0x60, 0x68, 0x12, 0xb7, 0x7a, 0xc3, 0xc9, 0xfa, 0x3d, 0x53,
    0x96, 0x84, 0x6b, 0xba, 0xf2, 0x63, 0x9a, 0x19, 0x7c, 0xae, 0xe5, 0xf5, 0xf7, 0x16, 0x6a, 0xa2,
    0x39, 0xb6, 0x7b, 0x0f, 0xc1, 0x93, 0x81, 0x1b, 0xee, 0xb4, 0x1a, 0xea, 0xd0, 0x91, 0x2f, 0xb8,
    0x55, 0xb9, 0xda, 0x85, 0x3f, 0x41, 0xbf, 0xe0, 0x5a, 0x58, 0x80, 0x5f, 0x66, 0x0b, 0xd8, 0x90,
    0x35, 0xd5, 0xc0, 0xa7, 0x33, 0x06, 0x65, 0x69, 0x45, 0x00, 0x94, 0x56, 0x6d, 0x98, 0x9b, 0x76,
    0x97, 0xfc, 0xb2, 0xc2, 0xb0, 0xfe, 0xdb, 0x20, 0xe1, 0xeb, 0xd6, 0xe4, 0xdd, 0x47, 0x4a, 0x1d,
    0x42, 0xed, 0x9e, 0x6e, 0x49, 0x3c, 0xcd, 0x43, 0x27, 0xd2, 0x07, 0xd4, 0xde, 0xc7, 0x67, 0x18,
    0x89, 0xcb, 0x30, 0x1f, 0x8d, 0xc6, 0x8f, 0xaa, 0xc8, 0x74, 0xdc, 0xc9, 0x5d, 0x5c, 0x31, 0xa4,
    0x70, 0x88, 0x61, 0x2c, 0x9f, 0x0d, 0x2b, 0x87, 0x50, 0x82, 0x54, 0x64, 0x26, 0x7d, 0x03, 0x40,
    0x34, 0x4b, 0x1c, 0x73, 0xd1, 0xc4, 0xfd, 0x3b, 0xcc, 0xfb, 0x7f, 0xab, 0xe6, 0x3e, 0x5b, 0xa5,
    0xad, 0x04, 0x23, 0x9c, 0x14, 0x51, 0x22, 0xf0, 0x29, 0x79, 0x71, 0x7e, 0xff, 0x8c, 0x0e, 0xe2,
    0x0c, 0xef, 0xbc, 0x72, 0x75, 0x6f, 0x37, 0xa1, 0xec, 0xd3, 0x8e, 0x62, 0x8b, 0x86, 0x10, 0xe8,
    0x08, 0x77, 0x11, 0xbe, 0x92, 0x4f, 0x24, 0xc5, 0x32, 0x36, 0x9d, 0xcf, 0xf3, 0xa6, 0xbb, 0xac,
    0x5e, 0x6c, 0xa9, 0x13, 0x57, 0x25, 0xb5, 0xe3, 0xbd, 0xa8, 0x3a, 0x01, 0x05, 0x59, 0x2a, 0x46,
];

// generate random 80-bit key
fn generate_random_key() -> [u8; 10] {
    // empty 80-bit array (10 bytes)
    let mut key = [0u8; 10];
    // fill it with garbage
    rand::thread_rng().fill_bytes(&mut key);
    // out it goes
    key
}

// word w = [g_1, g_2]
// G_perm takes g_1 and g_2, does stuff, spits out another word g_5||g_6
// k is the "step number", to use diff parts of the key, written as G^k
fn g_perm(k: &usize, w: u16, key: &[u8; 10]) -> u16 {
    let mut g_values = [0u8; 6]; //g_1 .. g_6
    let w_bytes = w.to_be_bytes();
    g_values[0] = w_bytes[0];
    g_values[1] = w_bytes[1];

    for i in 3..7 {
        let key_index = (4 * k + i - 3) % 10;
        // the index starts from 0 unlike i which starts at 1
        // so the i - 1 is there to make it fit in the arrays
        let f: usize = (g_values[(i-1) - 1] ^ key[key_index]).into();
        g_values[i-1] = F_TABLE[f] ^ g_values[(i-1) - 2];
    }    

    u16::from_be_bytes([g_values[4], g_values[5]])
}

// the inverse baby
// [g_5, g_6] as input, [g_1, g_2] as output
fn g_perm_inv(k: &usize, w: u16, key: &[u8; 10]) -> u16 {
    let mut g_values = [0u8; 6]; //g_1 .. g_6
    let w_bytes = w.to_be_bytes();
    g_values[4] = w_bytes[0];
    g_values[5] = w_bytes[1];

    for i in (3..7).rev() {
        let key_index = (4 * k + i - 3) % 10;
        let f: usize = (g_values[(i-1) - 1] ^ key[key_index]).into();
        g_values[(i-1) - 2] = F_TABLE[f] ^ g_values[i-1];
    }
    
    u16::from_be_bytes([g_values[0], g_values[1]])
}

// counter = 0 though 7.
pub fn rule_a(w: [u16; 4], counter: &u16, step: &usize, key: &[u8; 10]) -> [u16; 4] {
    // step 1) new w_2 = w_1 through G
    let w_2_new = g_perm(step, w[0], key);
    // step 2) new w_3 = previous w_2
    let w_3_new = w[1];
    // step 3) new w_4 = previous w_3
    let w_4_new = w[2];
    // step 4) the new w_1 depends on w_4 and the counter
    let w_1_new = w_2_new ^ w[3] ^ counter;

    [w_1_new, w_2_new, w_3_new, w_4_new]
}

pub fn rule_b(w: [u16; 4], counter: &u16, step: &usize, key: &[u8; 10]) -> [u16; 4] {
    // step 1) new w_1 is old w_4
    let w_1_new = w[3];
    // step 2) new w_2 is old w_1 through G
    let w_2_new = g_perm(step, w[0], key);
    // step 3) new w_3 is more annoying
    let w_3_new = w[0] ^ w[1] ^ counter;
    // step 4) new w_4 is old w_3
    let w_4_new = w[2];

    [w_1_new, w_2_new, w_3_new, w_4_new]
}

pub fn rule_a_inv(w: [u16; 4], counter: &u16, step: &usize, key: &[u8; 10]) -> [u16; 4] {
    let w_1_new = g_perm_inv(&(step-1), w[1], key);
    let w_2_new = w[2];
    let w_3_new = w[3];
    let w_4_new = w[0] ^ w[1] ^ counter;

    [w_1_new, w_2_new, w_3_new, w_4_new]
}

pub fn rule_b_inv(w: [u16; 4], counter: &u16, step: &usize, key: &[u8; 10]) -> [u16; 4] {
    let w_1_new = g_perm_inv(&(step-1), w[1], key);
    let w_2_new = w_1_new ^ w[2] ^ counter;
    let w_3_new = w[3];
    let w_4_new = w[0];

    [w_1_new, w_2_new, w_3_new, w_4_new]
}

fn encrypt(key: &[u8; 10], mut data: [u16; 4]) -> [u16; 4] {
    let mut counter = 1;

    for _step in 0..8 {
        data = rule_a(data, &counter, &((counter as usize)-1), key);
        counter += 1;
    }
    for _step in 0..8 {
        data = rule_b(data, &counter, &((counter as usize)-1), key);
        counter += 1;
    }
    for _step in 0..8 {
        data = rule_a(data, &counter, &((counter as usize)-1), key);
        counter += 1;
    }
    for _step in 0..8 {
        data = rule_b(data, &counter, &((counter as usize)-1), key);
        counter += 1;
    }

    data
}

fn decrypt(key: &[u8; 10], mut data: [u16; 4]) -> [u16; 4] {
    let mut counter = 32;

    for _step in 0..8 {
        data = rule_b_inv(data, &counter, &(counter as usize), key);
        counter -= 1;
    }
    for _step in 0..8 {
        data = rule_a_inv(data, &counter, &(counter as usize), key);
        counter -= 1;
    }
    for _step in 0..8 {
        data = rule_b_inv(data, &counter, &(counter as usize), key);
        counter -= 1;
    }
    for _step in 0..8 {
        data = rule_a_inv(data, &counter, &(counter as usize), key);
        counter -= 1;
    }

    data
}

fn main() {
    todo!("Please run `cargo test` if you just want to see it working.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g_perm_test() {
        // generate a key, a random word (w), permutates it through G and inverts it.
        let key: [u8; 10] = generate_random_key();
        let w: u16 = 0x1337;
        // k a byte of the key, ranges from 0 to 9 since key has 10 bytes 
        for k in 0..10 {
            let g_permed = g_perm(&k, w, &key);
            let g_inv = g_perm_inv(&k, g_permed, &key);
            assert_eq!(w, g_inv);
        }
    }

    #[test]
    fn encrypt_decrypt_test() {
        // generate a key, a random word (w), permutates it through G and inverts it.
        let key: [u8; 10] = generate_random_key();
        let block: [u16; 4] = [0x1337, 0x8008, 0x69, 0x420];
        // &1 is the index of the key, it doesn't matter for testing G_perm.
        let encrypted_block = encrypt(&key, block);
        let decrypted_block = decrypt(&key, encrypted_block);
        assert_eq!(block, decrypted_block);
    }
}
