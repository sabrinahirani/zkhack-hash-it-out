use zkhack_bls_pedersen::data::puzzle_data;
use blake2s_simd::Params;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Hash a message to 256 bits using Blake2s, output as Vec<bool> (LSB-first)
pub fn hash_to_bits(msg: &[u8]) -> Vec<bool> {
    let hash = Params::new().hash_length(32).to_state().update(msg).finalize();
    let mut bits = Vec::with_capacity(256);

    for byte in hash.as_bytes() {
        for i in 0..8 {
            bits.push((byte >> i) & 1 == 1); // little-endian
        }
    }

    bits
}

fn main() {
    // Load puzzle data: (pk, messages, signatures)
    let (_pk, messages, _sigs) = puzzle_data();

    // === Write message_bits.txt ===
    let mut msg_file = BufWriter::new(File::create("message_bits.txt").expect("Failed to create message_bits.txt"));

    for msg_bytes in messages.iter() {
        let bits = hash_to_bits(msg_bytes);
        assert_eq!(bits.len(), 256);
        for &bit in bits.iter() {
            let char_bit = if bit { '1' } else { '0' };
            write!(msg_file, "{}", char_bit).unwrap();
        }
        writeln!(msg_file).unwrap(); // newline after each message
    }

    // === Write target_bits.txt ===
    let target_string = "sabrinahirani";
    let target_bits = hash_to_bits(target_string.as_bytes());
    assert_eq!(target_bits.len(), 256);

    let mut target_file = BufWriter::new(File::create("target_bits.txt").expect("Failed to create target_bits.txt"));
    for &bit in target_bits.iter() {
        let char_bit = if bit { '1' } else { '0' };
        write!(target_file, "{}", char_bit).unwrap();
    }
    writeln!(target_file).unwrap(); // optional newline

    println!("✅ Done. Saved message_bits.txt and target_bits.txt");
}
