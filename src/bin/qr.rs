use std::env::args;

use qr::encode_qr;

fn main() {
    args()
        .skip(1)
        .next()
        .map_or_else(|| println!("Usage:\n\tqr <text>"), |arg| encode_qr(&arg));
}
