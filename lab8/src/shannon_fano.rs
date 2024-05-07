use std::collections::HashMap;

pub fn shannon_fano_encode(freq_map: &Vec<(char, usize)>) -> HashMap<char, String> {
    let mut encoding_map = HashMap::new();

    shannon_fano_helper(&freq_map, "".to_string(), &mut encoding_map);

    encoding_map
}

fn shannon_fano_helper(
    freq_list: &[(char, usize)],
    current_code: String,
    encoding_map: &mut HashMap<char, String>
) {
    //base case
    if freq_list.len() == 1 {
        encoding_map.insert(freq_list[0].0, current_code.to_string());
        return;
    }

    let mid = freq_list.len() / 2;
    let (left, right) = freq_list.split_at(mid);

    shannon_fano_helper(left, format!("{}0", current_code), encoding_map);
    shannon_fano_helper(right, format!("{}1", current_code), encoding_map);
}
