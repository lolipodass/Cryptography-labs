use std::io;

fn main() {
    println!("Enter a binary string to encode:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().replace(" ", "");

    if !input.chars().all(|c| c == '0' || c == '1') {
        println!("Please enter a valid binary string.");
        return;
    }

    let encoded_string = encode_hamming(&input);
    println!("Encoded string without error: {}", encoded_string);

    let error_position = 5;
    let _encoded_string_with_error = flip_bit(&encoded_string, error_position);
    // println!(
    //     "Encoded string with error:    {}",
    //     encoded_string_with_error
    // );

    // let error_position = 4;
    // let encoded_string_with_error2 = flip_bit(&encoded_string_with_error, error_position);
    // println!(
    //     "Encoded string with 2 error:  {}",
    //     encoded_string_with_error2
    // );

    let (decoded_string, error_pos) = decode_hamming(&encoded_string);

    println!("Decoded string: {}", decoded_string);
    match error_pos {
        Some(pos) => println!(
            "A single-bit error was detected and corrected at position: {}",
            pos
        ),
        None => println!("No errors were detected."),
    }
}

fn flip_bit(encoded: &str, position: usize) -> String {
    let mut encoded_chars: Vec<char> = encoded.chars().collect();
    let adjusted_position = position - 1;
    if adjusted_position < encoded_chars.len() {
        encoded_chars[adjusted_position] = if encoded_chars[adjusted_position] == '0' {
            '1'
        } else {
            '0'
        };
    }
    encoded_chars.into_iter().collect()
}

fn encode_hamming(data: &str) -> String {
    let data_len = data.len();
    let mut total_len = data_len;
    let mut parity_positions = Vec::new();

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

fn decode_hamming(encoded: &str) -> (String, Option<usize>) {
    let mut error_pos = 0;
    let encoded_len = encoded.len();
    let mut parity_positions = Vec::new();
    let mut parity_checks = Vec::new();

    // Determine parity bit positions
    let mut i = 0;
    while (1 << i) <= encoded_len {
        parity_positions.push(1 << i);
        parity_checks.push(0); // Initialize parity checks
        i += 1;
    }

    // Calculate parity checks
    for (index, &parity_pos) in parity_positions.iter().enumerate() {
        let mut parity = 0;
        let mut k = parity_pos;
        while k <= encoded_len {
            for l in k..std::cmp::min(k + parity_pos, encoded_len + 1) {
                parity ^= encoded.chars().nth(l - 1).unwrap() as u8 - b'0';
            }
            k += 2 * parity_pos;
        }
        parity_checks[index] = parity;

        if parity == 1 {
            error_pos += parity_pos; // Accumulate error position
        }
    }

    // Correct the single-bit error if exists
    let mut corrected_encoded: Vec<char> = encoded.chars().collect();
    if error_pos > 0 {
        corrected_encoded[error_pos - 1] = if corrected_encoded[error_pos - 1] == '0' {
            '1'
        } else {
            '0'
        };
    }

    // Remove parity bits to extract original data
    let mut data = String::new();
    for j in 1..=encoded_len {
        if !parity_positions.contains(&j) {
            data.push(corrected_encoded[j - 1]);
        }
    }

    (data, if error_pos > 0 { Some(error_pos) } else { None })
}
