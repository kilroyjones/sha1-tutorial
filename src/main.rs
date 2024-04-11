mod sha1;

use sha1::Sha1;

fn main() {
    let s = "TESTING";
    let mut sha1 = Sha1::new();
    let hash = sha1.hash(s);
    for byte in &hash {
        print!("{:02x}", byte);
    }
}
