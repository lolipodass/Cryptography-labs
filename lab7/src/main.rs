use std::char;


fn main() {
    let text = "banana";
    let (bwt, index) = burrows_wheeler_transform(text);
    println!("Original text: {}", text);
    println!("Burrows-Wheeler transform: {}", bwt);
    let reconstructed_text = burrows_wheeler_inverse_transform(&bwt, index);
    println!("Reconstructed text: {}", reconstructed_text);
}



fn burrows_wheeler_transform(text: &str) -> (String, usize) {
    let mut rotations:Vec<String> = Vec::new();
    for i in 0..text.len() {
        let rotation = text[i..].to_string() + &text[0..i];
        rotations.push(rotation);
    }

    print_table(&rotations);

    rotations.sort();


    print_table(&rotations);

    let suffixes: Vec<char> = rotations.iter().map(|rot| rot.chars().last().unwrap()).collect();

    //find index where our initial string  
    let index = rotations.iter().position(|rot| rot == &text[..]).unwrap();

    println!("{}",index);
    (suffixes.iter().collect(), index)

}

fn burrows_wheeler_inverse_transform(bwt: &str, index: usize) -> String {
    let mut result: Vec<String> = (0..bwt.len())
        .map(|_| String::new())
        .collect();

    for _ in 0..bwt.len() {
        println!("index: {:?}",result);

        for (i, c) in bwt.char_indices() {
            result[i] = format!("{}{}", c, result[i]);
        }
        result.sort();
    }

    result[index].chars().collect()
}


fn print_table(rows: &Vec<String>) {
    let mut max_width = 0;
    for row in rows {
        let width = format!("{:?}",row).len();
        if width > max_width {
            max_width = width;
        }
    }

    println!("+----+{:-<width$}+", "-", width=max_width+3);
    println!("| i  | Rotation  |");
    println!("+----+{:-<width$}+", "-", width=max_width+3);
    for (i,r) in rows.iter().enumerate() {
        println!("| {:<3}| {:<9}|", i, r);
    }
    println!("+----+{:-<width$}+", "-", width=max_width+3);
}

