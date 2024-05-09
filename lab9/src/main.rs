fn lz77_compress(input: String) -> Vec<(usize, usize, char)> {
    let mut result = Vec::new();

    let buffer_size = 32;
    let mut pos = 0;

    while pos < input.len() {
        println!("pos: {}", pos);

        let buff_start = if pos >= buffer_size { pos - buffer_size } else { 0 };

        let buffer_len = if pos >= buffer_size { buffer_size } else { pos - buff_start };

        // println!("buff_start: {}", buff_start);
        // println!("buffer_len: {}", buffer_len);

        let data: String = substring(&input, pos, buffer_size);

        println!("input: {:?}", data);

        let buffer = substring(&input, buff_start, buffer_len);

        println!("buffer: {:?}", buffer);

        let (length, offset) = find_biggest_substring(&buffer, &data);

        println!("length: {}, offset: {}", length, offset);

        let next_char = input
            .chars()
            .nth(pos + length)
            .unwrap();

        println!("pos-offset: {}, next_char: {}", buffer.len() - offset, next_char);

        result.push((if offset == 0 { 0 } else { buffer.len() - offset }, length, next_char));
        pos += if length == 0 { 1 } else { length + 1 };

        // println!("length: {:?}", length);
    }

    result
}

fn lz77_decompress(input: Vec<(usize, usize, char)>) -> String {
    let mut result = String::new();

    for (offset, length, next_char) in input {
        if offset > 0 {
            let substring = substring(&result, result.len() - offset, length);
            result.push_str(&substring);
        }

        result.push(next_char);
    }

    result
}

fn main() {
    let input =
        "u4FLi3szRR3tfNGGauJd3Jt1oJfRdGtHNEV6n90Fjk4J7CjIIQXqP9TTxzIk6rS5ArrM6wdXwr3AVfV47kog93nr3ojkUDNUOeZr!";
    // println!("{:?}", input);

    let compressed = lz77_compress(input.to_string());
    println!("{:?}", compressed);

    let decompressed = lz77_decompress(compressed);
    println!("{}", decompressed);
}

/// Finds the biggest substring in the given string.
///
/// This function searches the `input` string for the longest occurrence of the given `find_str`.
///
/// # Returns
///
/// The function returns a tuple containing the length of the longest substring and its offset in the `input` string.
///
fn find_biggest_substring(input: &String, find_str: &String) -> (usize, usize) {
    let mut max_length = 0;
    let mut max_offset = 0;

    for i in 0..find_str.len() {
        let substring = &find_str[0..=i];

        let found = input.rfind(substring);
        // println!("found: {:?}", found);
        if found.is_some() {
            max_length = substring.len();
            max_offset = found.unwrap();
        }
    }

    (max_length, max_offset)
}

fn substring(input: &String, start: usize, length: usize) -> String {
    input.chars().skip(start).take(length).collect()
}
