mod shannon_fano;
mod huffman;

use std::{ char, collections::HashMap };

use crate::{ huffman::huffman_encoding, shannon_fano::shannon_fano_encode };

fn main() {
    let input =
        "0QKZ3vlk9hQ79iK426Qcr0wl2E927i8eu994V1Ku8Zl91A3Sfbr0Pxp
        jJjNEyraRIESuxexUwwWaeEh0TV2f8IJwmLUfZEOpcto8M9WNVqS5YeV
        DD0XfgprGLPyDZh0fckRF4wlGuxZmXYpOznrCYCxu6fYwoiZpWbVz5Zo
        QT7c2siiK81C3bwba93u5WcC3ZTmzGcIXGmbTJf5y2I8KplnETFkQhPG
        ifIaJc1CCY6aJahKxlR2EmKszKRhRxBsIDeQmr2ojEWNZ0QQ76J6xVuz
        nfR4lnJepPYuo6CgEJT75kQdbkwCLduNSt3DshKdMG2xeitXPZsn4cUn
        UMrCrGqHvQIZY4FAQs0eh1BWWHMfJ7BUgeCW5WnSeRDu5luZVuqQ98nN
        upZfhV40UdqMKLMeCRJQa8MUdJmUV7XSeShrNs9AhoqLzBtMOTGYYdCQ
        n4hOkqZQCRuzks9B7kpGHrRYOiQ59OzrsABsKKTLPHTFTrtcZxgHLxCr
        uYUFkVwfhC8I8Y311lUAm14xfyc9VaKfrXkChhDPNUw0BMirHh8K8U86
        k8mXtCdFdH5gUlXXBwmdKR45A4wdInOnIMx5wJ3XPpMl64cptP07IsZR
        eHqiRWirajfFRMhVJJt2WArFQCDu7Wgn2IIeU3fB1vV9lvK6YixIX2eq
        qgI3pzMXalOC0kSDc44Y0NqgfJCAQSAAOAZrsWRU16NgpUBBw68pHZgY
        9CGAYWpU10iEqjTO9mLAyuxpEqGI2ETwdvv1hhdbpMLoMtU3LEfTnlm1
        WwLg4JcdTd3ofkruXccetNj66Njq8eFqCv2DbIs90wLugyL0SbMxIY0J
        HDjWHZgNhiyw20J9ZRahv8SGeJKRD7fFQnY9NFXxFDbcCKORbwxZhpcu
        N7XysPdmBe8e7YkBVC0WBDgUkRGzDKUda3SXrdSzuN41djDovPRdUwXz";
    // let input = "AAAAAAAAAAAAAAABBBBBBBGGGGGGDDDDDDLLLLL";

    let frequency_map = count_symbol_frequency(input);

    println!("Frequency map");
    for (key, value) in &frequency_map {
        println!("{}: {}", key, value);
    }
    let encoding_shannon_map = shannon_fano_encode(&frequency_map);

    //print in rows

    println!("Encoding map shannon");
    for (key, value) in &encoding_shannon_map {
        println!("{}: {}", key, value);
    }

    let encoding_huffman_map = huffman_encoding(&frequency_map);

    println!("Encoding map huffman");
    for (key, value) in &encoding_huffman_map {
        println!("{}: {}", key, value);
    }

    let mut input_binary = String::new();
    for character in input.to_string().clone().into_bytes() {
        input_binary += &format!("0{:b} ", character);
    }

    println!("Input binary: {}", input_binary);

    let coded_string_shannon = encode_string(input.to_string(), &encoding_shannon_map);

    println!("Coded string shannon: {}", coded_string_shannon);

    let coded_string_huffman = encode_string(input.to_string(), &encoding_huffman_map);

    println!("Coded string huffman: {}", coded_string_huffman);

    let decoded_string_shannon = decode_string(coded_string_shannon, &encoding_shannon_map);

    println!("Decoded string shannon: {}", decoded_string_shannon);

    let decoded_string_huffman = decode_string(coded_string_huffman, &encoding_huffman_map);

    println!("Decoded string huffman: {}", decoded_string_huffman);
}

fn encode_string(input: String, encoding_map: &HashMap<char, String>) -> String {
    let mut coded_string = String::new();

    input.chars().for_each(|c| {
        coded_string.push_str(&encoding_map.get(&c).unwrap().to_string().clone());
    });

    coded_string
}

fn decode_string(input: String, encoding_map: &HashMap<char, String>) -> String {
    let mut decoded_string = String::new();

    let mut buf = String::new();

    //invert hashmap
    let reverse_map = encoding_map
        .clone()
        .drain()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<String, char>>();

    println!("Reverse map: {:?}", reverse_map);

    for char in input.chars() {
        buf.push(char);

        let value = reverse_map.get(buf.as_str());
        if value.is_some() {
            decoded_string.push(value.unwrap().clone());
            buf = String::new();
        }
    }

    decoded_string
}

fn count_symbol_frequency(input: &str) -> Vec<(char, usize)> {
    let mut frequency_map = HashMap::new();

    for c in input.chars() {
        let count = frequency_map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut res: Vec<_> = frequency_map.into_iter().collect();
    res.sort_by(|(_, a), (_, b)| b.cmp(a));

    res
}
