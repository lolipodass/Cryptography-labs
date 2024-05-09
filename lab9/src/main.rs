fn lz77_compress(input: String) -> Vec<(usize, usize, char)> {
    let mut result = Vec::new();

    let buffer_size = 32;
    let mut pos = 0;

    while pos < input.len() {
        //(offset, length)
        let mut max_occurrence = (0, 1);

        let input_buffer: String = input.chars().skip(pos).take(buffer_size).collect();

        let start = if pos >= buffer_size { pos - buffer_size } else { 0 };
        let end = pos;

        let dict_buffer: String = input
            .chars()
            .skip(start)
            .take(end - start)
            .collect();

        find_biggest_substring(dict_buffer, input_buffer);
    }

    result
}

fn main() {
    let input = "Hello, Worldel!";
    println!("{:?}", input);

    let compressed = lz77_compress(input.to_string());
    println!("{:?}", compressed);
}

//(size, offset)
fn find_biggest_substring(input: String, find_str: String) -> (usize, usize) {
    let mut max_length = 0;
    let mut max_offset = 0;

    for i in 0..find_str.len() {
        let substring = &find_str[0..=i];

        let found = input.find(substring);
        if found.is_some() {
            max_length = substring.len();
            max_offset = found.unwrap();
        }
    }

    (max_length, max_offset)
}
