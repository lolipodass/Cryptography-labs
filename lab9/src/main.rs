fn lz77_compress(input: String) -> Vec<(usize, usize, char)> {
    let mut result = Vec::new();

    let buffer_size = 32;
    let mut pos = 0;

    while pos < input.len() {
        // println!("pos: {}", pos);

        let buff_start = if pos >= buffer_size { pos - buffer_size } else { 0 };

        let buffer_len = if pos >= buffer_size { buffer_size } else { pos - buff_start };

        // println!("buff_start: {}", buff_start);
        // println!("buffer_len: {}", buffer_len);

        let data: String = substring(&input, pos, buffer_size);

        // println!("input: {:?}", data);

        let buffer = substring(&input, buff_start, buffer_len);

        // println!("buffer: {:?}", buffer);

        let (length, offset) = find_biggest_substring(&buffer, &data);

        // println!("length: {}, offset: {}", length, offset);

        let next_char = match input.chars().nth(pos + length) {
            Some(c) => c,
            None => '\0',
        };

        // println!("pos-offset: {}, next_char: {}", buffer.len() - offset, next_char);

        result.push((if offset == 0 { 0 } else { buffer.len() - offset }, length, next_char));
        pos += if length == 0 { 1 } else { length + 1 };

        // println!("length: {:?}", length);
    }

    result
}

fn lz77_decompress(input: &Vec<(usize, usize, char)>) -> String {
    let input = input.clone();
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
        "DjQzRG9xvyQNSzwHd9EWeTlZQL7up8WDlXI1FmRLTUi4eUp3TYyRNvAcHPLQvL16YpsukA9jBKKmEVZFgmoJTG2
        32M6zXwdWJsNoF85ZSTW3e24eLLS3VZZDKoR75VZQc3m56eMew5yT6KEwrHJv1r1HrIcV6QnOkQNYlja0nkM2NLQ
        1Z6m462NrXNkjlxE3SKAepC89GFV8d7sYsOGOZ8eW8LdX1WHIxTneg9pU2amh29lOYJ6l7gPp6t5SjpFxD54hMJD
        ulFgxxTFJIEQiYOhl2vrEeUO3LSfanhGRsXoPvYmt3IXllU9Sh9UUjSAVGtyMIRZC2LZ8le4pdw9ltf3pz1GaXyl
        xWbJKMxJkmtqRpej2HtV4lwvUwzP7DyEDxTpLxUr1Pu3os7yYLNmLylxav0JfqYS7d3kbofWsiheVEFWhzeY9e5e
        4aixszT6oXqlu0yY73Ucct8rlZrzpqTXK7iPaWDK7ooYyMLCneoODsazFpbea!";
    // let input =
    //     "banananababananafananaannaddaffdsfsdlkjojvjsl ksiojdf sdjfj sdklf; sfjiosfjos fjklsdfkljsdfs";

    let compressed = lz77_compress(input.to_string());
    println!("{:?}", compressed);

    let decompressed = lz77_decompress(&compressed);

    println!("input \t{}", input);
    println!("output \t{}", decompressed);

    if input == decompressed.trim_end_matches('\0') {
        println!("input and decompressed are equal");
    } else {
        println!("input and decompressed are not equal");
    }

    let input_len = (input.len() as f64) * 8.0;
    let output_len = (compressed.len() as f64) * 8.0;
    let r1 = output_len / input_len;
    let r2 = 1.0 - r1;

    println!("input_len: {}, output_len: {}", input_len, output_len);
    println!("r1: {}, r2: {}", r1, r2);
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
