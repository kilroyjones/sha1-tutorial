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
               // rotating means a small change in input affects the output more
               // wrapping add means that when a value overflow (exceeds its available space for a
               // variable) that it starts again from 0.
               // This is the avalanche affect. A tiny change input produces a large change in
               // output.

               // Note this "diffusion" of fuckery moves from e down to a,
               // You'll notice that the last rounds a is put in b, and this
               // trickles down to c then d and e and so forth. It might be the
               // one place where a trickle down policy is actually effective.
*
*/
// To look up
// 1. hash state initialization attacks.
mod sha1;

use sha1::Sha1;

fn main() {
    let s = "teststring";
    let mut sha1 = Sha1::new();
    let hash = sha1.hash(s);
    for byte in &hash {
        print!("{:02x}", byte);
    }
}
