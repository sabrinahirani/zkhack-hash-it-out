use zkhack_bls_pedersen::data::puzzle_data;
use blake2s_simd::Params;

use std::io::{BufWriter, Write};
use std::fs::File;

pub fn hash_to_bits(msg: &[u8]) -> Vec<bool> {
    // hash message using Blake2s
    let hash = Params::new().hash_length(32).to_state().update(msg).finalize();
    let mut bits = Vec::with_capacity(256);
    // convert to 256 bits
    for byte in hash.as_bytes() {
        for i in 0..8 {
            bits.push((byte >> i) & 1 == 1); // little-endian
        }
    }

    bits
}

fn main() {

    // 1. write message_bits.txt

    // load puzzle data: public key, messages, signatures
    let (_pk, messages, _sigs) = puzzle_data();

    let mut msg_file = BufWriter::new(File::create("message_bits.txt").expect("Failed to create file message_bits.txt"));

    for msg_bytes in messages.iter() {
        let bits = hash_to_bits(msg_bytes);
        // ensure correct dimensions
        assert_eq!(bits.len(), 256);
        for &bit in bits.iter() {
            let char_bit = if bit { '1' } else { '0' };
            write!(msg_file, "{}", char_bit).unwrap();
        }
        writeln!(msg_file).unwrap(); // newline after each message
    }

    // 2. write target_bits.txt

    let target_str = "sabrinahirani"; // handle
    let target_bits = hash_to_bits(target_str.as_bytes());
    // ensure correct dimensions
    assert_eq!(target_bits.len(), 256);

    let mut target_file = BufWriter::new(File::create("target_bits.txt").expect("Failed to create file target_bits.txt"));
    
    for &bit in target_bits.iter() {
        let char_bit = if bit { '1' } else { '0' };
        write!(target_file, "{}", char_bit).unwrap();
    }
    writeln!(target_file).unwrap(); // optional newline

    println!("Done. Saved message_bits.txt and target_bits.txt.");
}
