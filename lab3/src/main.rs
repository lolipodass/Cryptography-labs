use std::io;

fn main() {
    println!("Enter a binary string to encode:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().replace(" ", ""); // Remove spaces if any

    if !input.chars().all(|c| c == '0' || c == '1') {
        println!("Please enter a valid binary string.");
        return;
    }

    let encoded = encode_hamming(&input);
    println!("Encoded string: {}", encoded);
}

fn encode_hamming(data: &str) -> String {
    let data_len = data.len();
    let mut total_len = data_len;
    let mut parity_positions = Vec::new();

    // Calculate total length including parity bits
    let mut i = 0;
    while (1 << i) <= total_len {
        parity_positions.push(1 << i);
        total_len += 1;
        i += 1;
    }

    // Correct calculation for the total length with parity bits
    total_len = data_len + parity_positions.len();

    let mut encoded: Vec<char> = vec!['0'; total_len];
    let mut data_iter = data.chars();

    // Place data bits
    for j in 1..=total_len {
        if !parity_positions.contains(&j) {
            if let Some(data_bit) = data_iter.next() {
                encoded[j - 1] = data_bit;
            }
        }
    }

    // Calculate and place parity bits
    for &parity_pos in &parity_positions {
        let mut parity = 0;
        let mut k = parity_pos;
        while k <= total_len {
            for l in k..std::cmp::min(k + parity_pos, total_len + 1) {
                parity ^= encoded[l - 1] as u8 - b'0';
            }
            k += 2 * parity_pos;
        }
        encoded[parity_pos - 1] = if parity == 1 { '1' } else { '0' };
    }

    encoded.into_iter().collect()
}
