use std::io::{self, Read};

// To look up
// 1. hash state initialization attacks.

// Convert these to decimal. They are fractional parts of a the square root of values 2, 3, 5, 7, and 11
// These are chosen for randomness and to prevent hash state initialization attacks.
const H0: u32 = 0x67452301;
const H1: u32 = 0xEFCDAB89;
const H2: u32 = 0x98BADCFE;
const H3: u32 = 0x10325476;
const H4: u32 = 0xC3D2E1F0;

fn main() {
    let s = "teststring";
    let hash = sha1(s.as_bytes());
    for byte in &hash {
        print!("{:02x}", byte);
    }
    println!(); // Add a newline for clean output
}

struct Sha1 {}

impl Sha1 {
    fn new() -> Self {
        Sha1 {}
    }
}
/**
 * Steps for SHA-1
 *
 * 1). Take input and split it into ASCII values
 * 2). Convert each to binary and prefix with padding to 8 bits
 * 3). Join and append 1 to the suffix
 * 4). Pad the binary string with 0s until length % 512 == 448.
 * 5). Get length of our initial value (string we're hashing) - get it's length and convert that to binary
 * 6). Pad that length value in binary with prefix 0s until it's 64 characters long.
 * 7). Append to the binary message from step(4). This gives us mod 512.
 * 8). Break into chunks of 512 characters.
 * 9). Break these 512 chunks into 32 bit words (chunks)
 * 10). Loop through these 32 bit chunks and then extend the array doing WHAT?
 * 11). using the above the initial five variables just reassign using bitwise operations
 * 12). convert them to hexadecimal
 *
 */
fn sha1(input: &[u8]) -> [u8; 20] {
    let mut h0 = H0;
    let mut h1 = H1;
    let mut h2 = H2;
    let mut h3 = H3;
    let mut h4 = H4;

    let mut bytes = input.to_vec();

    // Save this for later
    let original_bit_length = bytes.len() as u64 * 8;

    // Padding with 0x80 to demarcate where the padding begins
    bytes.push(0x80);

    // Add padding
    while (bytes.len() * 8) % 512 != 448 {
        bytes.push(0);
    }

    // This is used so that message with the same content, but different lengths will produce
    // different hashes. Important for collision resistance. "Test" vs. "Test "
    bytes.extend_from_slice(&original_bit_length.to_be_bytes());

    // Gets a chunk of 64 bytes at a time 512 bit block
    for chunk in bytes.chunks(64) {
        println!("{:?}", chunk);
        let mut w = [0u32; 80];

        // This gets 4 byte (32 bits) from the block at a time, putting it in w (0..15)
        for (i, block) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes(block.try_into().expect("Slice with incorrect length"));
        }

        // Padding the rest of w using parts of of the previous blocks XOR'ed together
        for i in 16..80 {
            w[i] = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16];
            w[i] = w[i].rotate_left(1); // move leftmost bit to the right
        }

        // TODO: fix this, just get intiail values but mut
        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

        // Loop over these 80 chunk and scramble the fuck out
        // These constants come from prime square and cube roots

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };

            // rotating means a small change in input affects the output more
            // wrapping add means that when a value overflow (exceeds its available space for a
            // variable) that it starts again from 0.
            // This is the avalanche affect. A tiny change input produces a large change in
            // output.

            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut hash = [0u8; 20];
    hash[0..4].copy_from_slice(&h0.to_be_bytes());
    hash[4..8].copy_from_slice(&h1.to_be_bytes());
    hash[8..12].copy_from_slice(&h2.to_be_bytes());
    hash[12..16].copy_from_slice(&h3.to_be_bytes());
    hash[16..20].copy_from_slice(&h4.to_be_bytes());
    hash
}
