use std::collections::HashMap;

pub fn huffman_encoding(frequency_list: &Vec<(char, usize)>) -> HashMap<char, String> {
    let mut freq_map = HashMap::new();

    //convert to Vec<(String, usize)>
    let mut freq_list: Vec<(String, usize)> = frequency_list
        .iter()
        .map(|(c, freq)| (c.to_string(), *freq))
        .collect();

    //initial fill freq_map
    frequency_list.iter().for_each(|(c, _)| {
        freq_map.insert(*c, String::new());
    });

    while freq_list.len() > 1 {
        freq_list.sort_by(|a, b| b.1.cmp(&a.1));

        println!("freq_list {:?}", freq_list);

        //take the lowest two
        let left = freq_list.pop().unwrap();
        let right = freq_list.pop().unwrap();

        let combined = format!("{}{}", left.0, right.0);
        let combined_freq = left.1 + right.1;

        left.0.chars().for_each(|c| {
            freq_map.get_mut(&c).unwrap().insert(0, '1');
        });
        right.0.chars().for_each(|c| { freq_map.get_mut(&c).unwrap().insert(0, '0') });

        freq_list.push((combined, combined_freq));
    }

    freq_map
}
