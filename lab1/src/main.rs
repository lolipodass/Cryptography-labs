#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Write;
use std::{f64, io};

mod utils;

fn calculate_entropy_shannon(text: &str, file_name: &str) -> f64 {
    let mut freq_map = HashMap::new();

    for ch in text.chars() {
        let elem = freq_map.entry(ch).or_insert(0);
        *elem += 1;
    }
    let mut entropy = 0.0;
    let amount_characters = text.len() as f64;
    let mut prob_map = HashMap::new();

    for (symbol, &amount) in freq_map.iter() {
        let char_prob = amount as f64 / amount_characters;
        prob_map.insert(symbol, char_prob);

        entropy -= char_prob * char_prob.log2();
    }
    // println!("{:?}", prob_map);
    let _ = utils::save(freq_map, file_name);

    entropy
}

fn calculate_entropy_hartli(text: &str) -> f64 {
    let mut freq_map = HashMap::new();

    for ch in text.chars() {
        let elem = freq_map.entry(ch).or_insert(0);
        *elem += 1;
    }
    let entropy = (freq_map.len() as f64).log2();

    entropy
}

fn main() {
    let mut input = String::new();
    print!("Please enter a string to encode: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let shannon_entropy = calculate_entropy_shannon(input, "symbol.scv");
    let hartli_entropy = calculate_entropy_hartli(input);

    let encoded_string = string_to_base64(input);

    let base64_entropy = calculate_entropy_shannon(&encoded_string, "base64.csv");

    println!("symbol shannon entropy: {}", shannon_entropy);
    println!("symbol hartli entropy: {}", hartli_entropy);
    println!("redundancy: {}", 1.0 - (shannon_entropy / hartli_entropy));
    println!("base64 shannon entropy: {}", base64_entropy);
    println!("base64 string: {}", encoded_string);

    let binary_and = string_to_binary("andrey");
    let binary_kov = string_to_binary("kovalev");
    let xor = xor_binary_strings(&binary_and, &binary_kov);
    let xor_xor = xor_binary_strings(&xor, &binary_kov);

    println!("{}  binary andrey ", binary_and);
    println!("{}  binary kovalev ", binary_kov);
    println!("{} xor", xor);
    println!("{} xor xor", xor_xor);
}

fn string_to_base64(input: &str) -> String {
    const BASE64_CHARS: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let bytes = input.as_bytes();
    let mut base64 = String::new();
    let padding = input.len() % 3;

    for chunk in bytes.chunks(3) {
        let mut binary_representation = String::new();
        for &byte in chunk {
            binary_representation.push_str(&format!("{:08b}", byte));
        }
        while binary_representation.len() < (chunk.len() + 1) * 6 {
            binary_representation.push('0');
        }

        for i in (0..binary_representation.len()).step_by(6) {
            let index = u32::from_str_radix(&binary_representation[i..i + 6], 2).unwrap();

            base64.push(BASE64_CHARS[index as usize] as char);
        }
    }

    if padding == 1 {
        base64.push_str("==");
    } else if padding == 2 {
        base64.push('=');
    }

    base64
}

fn compute_information(entropy: f64, name: &str, error_level: f64) -> f64 {
    match error_level {
        _ if error_level == 0.0 => name.len() as f64 * entropy,
        _ if error_level == 1.0 => 1.0f64,
        _ if error_level > 0.0 && error_level < 1.0 => {
            let conditional_entropy = -error_level * error_level.log2()
                - (1.0f64 - error_level) * (1.0f64 - error_level).log2();
            1.0f64 - conditional_entropy
        }
        _ => 0.0f64,
    }
}

fn base64_to_bytes(base64_str: &str) -> Vec<u8> {
    #[allow(deprecated)]
    base64::decode(base64_str).expect("Failed to decode base64 string")
}

fn string_to_binary(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |i| ((byte >> i) & 1) as u8))
        .map(|bit| if bit == 0 { '0' } else { '1' })
        .collect()
}

fn xor_binary_strings(a: &str, b: &str) -> String {
    // Преобразуем строки в векторы u8, представляющие бинарные числа
    let a_binary_vec: Vec<u8> = a.chars().map(|c| c as u8 - '0' as u8).collect();
    let b_binary_vec: Vec<u8> = b.chars().map(|c| c as u8 - '0' as u8).collect();

    // Определяем размеры векторов
    let a_len = a_binary_vec.len();
    let b_len = b_binary_vec.len();

    // Определяем, какой вектор больше
    let max_len = a_len.max(b_len);

    // Дополняем меньший вектор нулями до размера большего
    let a_binary_vec = a_binary_vec
        .into_iter()
        .chain(std::iter::repeat(0).take(max_len - a_len))
        .collect::<Vec<u8>>();
    let b_binary_vec = b_binary_vec
        .into_iter()
        .chain(std::iter::repeat(0).take(max_len - b_len))
        .collect::<Vec<u8>>();

    // Выполняем XOR операцию и преобразуем результат в строку
    a_binary_vec
        .iter()
        .zip(b_binary_vec.iter())
        .map(|(&x, &y)| x ^ y)
        .map(|bit| if bit == 0 { '0' } else { '1' })
        .collect()
}
