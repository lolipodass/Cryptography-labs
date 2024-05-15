use std::{ collections::HashMap, time::Instant };

use bigdecimal::BigDecimal;

fn compute_freq_map(message: String) -> Vec<(char, BigDecimal)> {
    let mut frequency_map = HashMap::new();
    let length: usize = message.chars().count();
    let prec_size: u64 = u64::try_from(length.clone()).unwrap();

    for c in message.chars() {
        let count = frequency_map.entry(c).or_insert(BigDecimal::from(0));
        *count += BigDecimal::from(1);
    }

    let mut res: Vec<_> = frequency_map
        .iter()
        .map(|(char, count)| (*char, count / BigDecimal::from(length as i64).with_prec(prec_size)))
        .collect();

    res.sort_by(|(_, a), (_, b)| a.cmp(b));
    res
}

fn start_interval(alphabet: Vec<(char, BigDecimal)>) -> Vec<(char, BigDecimal, BigDecimal)> {
    let mut interval: Vec<(char, BigDecimal, BigDecimal)> = Vec::new();
    let mut min: BigDecimal = BigDecimal::from(0);

    for item in &alphabet {
        if item == &alphabet[0] {
            interval.push((alphabet[0].0.clone(), BigDecimal::from(0), alphabet[0].1.clone()));
            min = alphabet[0].1.clone();
        } else {
            interval.push((item.0.clone(), min.clone(), min.clone() + item.1.clone()));
            min = min.clone() + item.1.clone();
        }
    }

    interval
}

fn calculate_interval(
    start_interval: Vec<(char, BigDecimal, BigDecimal)>,
    min: BigDecimal,
    max: BigDecimal
) -> Vec<(char, BigDecimal, BigDecimal)> {
    let mut interval: Vec<(char, BigDecimal, BigDecimal)> = Vec::new();

    for item in start_interval {
        let new_min = min.clone() + (max.clone() - min.clone()) * item.1.clone();
        let new_max = min.clone() + (max.clone() - min.clone()) * item.2.clone();
        interval.push((item.0, new_min, new_max));
    }

    interval
}

fn get_char_in_interval(
    value: BigDecimal,
    start_interval: Vec<(char, BigDecimal, BigDecimal)>
) -> (BigDecimal, BigDecimal, char) {
    let mut min = BigDecimal::from(0);
    let mut max = BigDecimal::from(0);
    let mut ch: char = ' ';

    for item in start_interval {
        if item.1 <= value && value <= item.2 {
            min = item.1;
            max = item.2;
            ch = item.0;
        }
    }

    return (min, max, ch);
}

fn encode_message(
    message: String,
    start_interval: Vec<(char, BigDecimal, BigDecimal)>
) -> BigDecimal {
    let mut step_interval = start_interval.clone();

    for i in 0..message.chars().count() - 1 {
        let item = step_interval
            .iter()
            .find(|item| item.0 == message.chars().nth(i).unwrap())
            .unwrap();
        let min = item.1.clone();
        let max = item.2.clone();
        step_interval = calculate_interval(start_interval.clone(), min, max);
    }

    let result = step_interval
        .iter()
        .find(|item| item.0 == message.chars().last().unwrap())
        .unwrap();
    result.1.clone()
}

fn decode_message(
    encoded_message: BigDecimal,
    start_interval: Vec<(char, BigDecimal, BigDecimal)>,
    message_length: usize
) -> String {
    let mut decoded_message = String::new();
    let mut step_value = encoded_message.clone();

    for _ in 0..message_length {
        let (min, high_boundary, ch) = get_char_in_interval(
            step_value.clone(),
            start_interval.clone()
        );
        step_value = (step_value.clone() - min.clone()) / (high_boundary.clone() - min.clone());
        decoded_message.push(ch);
    }

    decoded_message
}

fn main() {
    // let message: &str = "testingingedsffdss9jdslfjsldjslsf";
    // let message: &str = "testingingedsffdss9jdslfjsldjslsffdfddfffdssfddsfd";
    let message: &str = "малосимпатичныйдостопримечательность";

    let alphabet = compute_freq_map(message.to_string());
    let start_interval = start_interval(alphabet.clone());

    let start = Instant::now();

    let encoded_duration = Instant::now() - start;
    let encoded_message = encode_message(message.to_string(), start_interval.clone());
    let start = Instant::now();
    let decoded_message = decode_message(
        encoded_message.clone(),
        start_interval.clone(),
        message.chars().count()
    );
    let decoded_duration = Instant::now() - start;

    println!("Encoded message - {}", encoded_message);
    println!("Decoded message - {}", decoded_message);
    println!("Encoded message time - {:?}", encoded_duration);
    println!("Decoded message time - {:?}", decoded_duration);
}
