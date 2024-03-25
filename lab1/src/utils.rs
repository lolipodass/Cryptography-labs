use std::{collections::HashMap, fs};

use csv::Writer;

/// Reads a file and returns its contents as either a string or a binary representation.
///
/// This function attempts to open a file with the given `name` and reads its contents based on the specified `mode`.
/// The `mode` parameter determines whether the file should be read as a text string (`mode ==  0`) or as a binary string (`mode ==  1`).
/// For any other value of `mode`, an error will be returned indicating an incorrect read mode.
///
/// # Arguments
///
/// * `name` - A string slice representing the path to the file to be read.
/// * `mode` - An integer representing the mode of reading:
///     * `0` - Reads the file as a text string.
///     * `1` - Reads the file as a binary string, where each byte is represented as a binary number.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or read, or if the `mode` is not `0` or `1`.
///
pub fn read_file(name: &str, mode: i32) -> Result<String, String> {
    match mode {
        0 => fs::read_to_string(name).map_err(|e| e.to_string()),
        1 => {
            let bytes = std::fs::read(name).unwrap();

            let content: String = bytes
                .iter()
                .map(|b| format!("{:0b}", b))
                .collect::<Vec<String>>()
                .join("");
            Ok(content)
        }
        _ => Err("Wrong type of read".to_string()),
    }
}

/// Saves the frequency map of characters to a CSV file.
///
/// This function takes a `HashMap` where the keys are characters and the values are their frequencies,
/// and a file name as a string slice. It writes the character frequencies to a CSV file with the given
/// file name. The CSV file will have two columns: "Char" for the characters and "Amount" for their frequencies.
/// Special characters like newline (`\n`) and carriage return (`\r`) are escaped in the output.
///
/// # Arguments
///
/// * `map_elements` - A `HashMap` where the keys are characters and the values are their frequencies.
/// * `file_name` - A string slice representing the path to the file where the frequencies will be saved.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or written to.
///
pub fn save(map_elements: HashMap<char, i32>, file_name: &str) -> std::io::Result<()> {
    let mut wtr = Writer::from_path(file_name)?;

    wtr.write_record(&["Char", "Amount"])?;

    for (key, value) in map_elements {
        let val = match key {
            '\n' => String::from("\\n"),
            '\r' => String::from("\\r"),
            _ => key.to_string(),
        };
        wtr.write_record(&[val, value.to_string()])?;
    }
    wtr.flush()?;

    Ok(())
}
