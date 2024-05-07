use std::io::{self, Write};

//Циклический код
fn main() {
    let max_size_bits = 11;
    let generator: Vec<u8> = vec![1, 0, 0, 1, 1];

    let parity_check_matrix = build_parity_check_matrix(&generator);

    display_matrix(&parity_check_matrix);

    let message = read_binary_message(max_size_bits);

    let remainder = compute_remainder(&message, &generator);

    let encoded = add_remainder_to_message(&message, &remainder);

    println!("Encoded message:\t {}", u8_vec_to_string(&encoded));

    let mut received = encoded.clone();

    let flip_bit_index = 1;

    flip_bit_at_index(&mut received, flip_bit_index);

    println!("Received message:\t {}", u8_vec_to_string(&received));

    let redundant_bits = compute_remainder(&received, &generator);
    println!("red\t {}", u8_vec_to_string(&redundant_bits));

    let error_index = find_matching_row_index(&redundant_bits, &parity_check_matrix);

    println!("error Index: {:?}", error_index.expect("no error"));
}

fn build_parity_check_matrix(generator: &[u8]) -> Vec<Vec<u8>> {
    let n = 11;

    let mut parity_check_matrix = Vec::new();

    parity_check_matrix.push(vec![1, 0, 0, 0]);
    parity_check_matrix.push(vec![0, 1, 0, 0]);
    parity_check_matrix.push(vec![0, 0, 1, 0]);
    parity_check_matrix.push(vec![0, 0, 0, 1]);

    for i in 0..n {
        let mut pol = vec![0; n];
        pol[i] = 1;
        let row = compute_remainder(&pol, generator);
        parity_check_matrix.push(row);
    }
    parity_check_matrix
}

fn compute_remainder(message: &[u8], generator: &[u8]) -> Vec<u8> {
    let mut padded_message = message.to_vec();
    for _ in 0..(generator.len() - 1) {
        padded_message.push(0);
    }

    let mut remainder = padded_message.clone();

    for i in 0..message.len() {
        if remainder[i] == 1 {
            // println!("compute i -{}", u8_vec_to_string(&remainder));
            for j in 0..generator.len() {
                remainder[i + j] ^= generator[j];
            }
        }
    }

    remainder[remainder.len() - 4..].to_vec()
}

fn add_remainder_to_message(message: &[u8], remainder: &[u8]) -> Vec<u8> {
    let mut encoded: Vec<u8> = message.iter().cloned().collect();
    encoded.extend(remainder.iter().cloned());
    encoded
}

fn find_matching_row_index(
    redundant_bits: &[u8],
    parity_check_matrix: &[Vec<u8>],
) -> Option<usize> {
    for (index, row) in parity_check_matrix.iter().enumerate() {
        if row == redundant_bits {
            return Some(index);
        }
    }
    None
}

fn u8_vec_to_string(vec: &Vec<u8>) -> String {
    vec.iter().map(|&i| (i + '0' as u8) as char).collect()
}

fn display_matrix(matrix: &Vec<Vec<u8>>) {
    for row in matrix {
        for &item in row {
            print!("{} ", item);
        }
        println!();
    }
}

fn read_binary_message(max_amount: usize) -> Vec<u8> {
    print!("Enter a binary message: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .chars()
        .take(max_amount)
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn flip_bit_at_index(received: &mut Vec<u8>, error_index: usize) {
    if let Some(bit) = received.get_mut(error_index) {
        *bit = !(*bit != 0) as u8;
    }
}
