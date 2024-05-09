use rand::Rng;

use crate::iterative_code::IterativeCode;

mod iterative_code;

fn main() {
    let columns = 4;
    let information_length = 4;
    let code_length = 4 * 8;

    let mut message = generate_string(information_length, code_length);

    println!("Generated message {}", vec_bool_to_string(&message));

    while message.len() < code_length {
        message.push(false);
    }

    let inter = interleave(&message, code_length / columns, columns);

    println!("interleave {}", vec_bool_to_string(&inter));

    // Simulate packet error
    let error_rate = 0.5; // 10% error rate
    let error_packet = generate_random_error(&inter, error_rate);
    println!("Packet with error: {}", vec_bool_to_string(&error_packet));

    let deinter = deinterleave(&error_packet, code_length / columns, columns);
    println!("deinterleave {}", vec_bool_to_string(&deinter));

    let initial_message = decode_deinterleaved_data(&deinter, information_length);

    // println!("initial message {}", vec_bool_to_string(&initial_message));
}

fn decode_deinterleaved_data(deinterleaved_data: &[bool], inf_size: usize) -> Vec<bool> {
    let mut decoded_data = Vec::new();
    let (rows, cols) = calculate_rows_cols(inf_size);
    let mut code = IterativeCode::new_empty(rows, cols);

    println!("rows {},cols {}", &rows, &cols);
    println!("len {}", &inf_size);
    println!("code size {}", &code.code_size);

    for chunk in deinterleaved_data.chunks(code.code_size) {
        println!("chunk {}", vec_bool_to_string(&chunk.to_vec()));

        if chunk.len() == code.code_size {
            let decoded_chunk = code.decode(chunk.to_vec()).expect("Decoding failed");

            decoded_data.extend(decoded_chunk);
        }
    }

    decoded_data
}

fn generate_string(word_size: usize, code_len: usize) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    let mut res_not_encoded: Vec<bool> = Vec::new();
    let (rows, cols) = calculate_rows_cols(word_size);

    let mut code = IterativeCode::new_empty(rows, cols);

    println!("rows {},cols {}", rows, cols);
    println!("code size {}", code.code_size);

    for _word in 0..code_len / code.code_size {
        let data = generate_random_binary_(word_size);
        res_not_encoded.extend(data.clone());
        println!("word {}", vec_bool_to_string(&data));

        let encoded = code.encode(data);
        println!("encd {}", vec_bool_to_string(&encoded));
        res.extend(encoded);
    }

    println!("message not encoded {}", vec_bool_to_string(&res_not_encoded));

    res
}

fn interleave(data: &[bool], num_rows: usize, num_cols: usize) -> Vec<bool> {
    let mut interleaved_data = Vec::with_capacity(data.len());

    for col in 0..num_cols {
        for row in 0..num_rows {
            interleaved_data.push(data[row * num_cols + col]);
        }
    }

    interleaved_data
}

fn deinterleave(data: &[bool], num_rows: usize, num_cols: usize) -> Vec<bool> {
    let mut deinterleaved_data = Vec::with_capacity(data.len());

    for row in 0..num_rows {
        for col in 0..num_cols {
            deinterleaved_data.push(data[col * num_rows + row]);
        }
    }

    deinterleaved_data
}

fn generate_random_binary_(length: usize) -> Vec<bool> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_bool(0.5)).collect()
}

fn vec_bool_to_string(vec: &Vec<bool>) -> String {
    vec.clone()
        .into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect()
}

fn calculate_rows_cols(code_len: usize) -> (usize, usize) {
    let mut rows = (code_len as f64).sqrt() as usize;
    let mut cols = (code_len + rows - 1) / rows;

    // Ensure rows * cols = code_len
    if rows * cols < code_len {
        rows += 1;
        cols = (code_len + rows - 1) / rows;
    }

    (rows, cols)
}

fn generate_random_error(packet: &Vec<bool>, error_rate: f64) -> Vec<bool> {
    let mut rng = rand::thread_rng();
    let mut error_packet = Vec::new();

    for bit in packet {
        let flip_bit = rng.gen_bool(error_rate);
        if flip_bit {
            error_packet.push(!bit);
        } else {
            error_packet.push(*bit);
        }
    }

    error_packet
}
