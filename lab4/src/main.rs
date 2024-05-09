use rand::Rng;

use crate::iterative_code::IterativeCode;

mod iterative_code;

fn main() {
    let mut code = IterativeCode::new_empty(6, 5);

    let data = "010011011101101101011101011011";
    let bool_data: Vec<bool> = data
        .chars()
        .map(|x| x == '1')
        .collect();
    let mut encode = code.encode(bool_data);

    print!("encoded word \t{}", vec_bool_to_string(&encode));

    let bit = rand::thread_rng().gen_range(0..encode.len());
    encode[bit] ^= true;
    print!("\nerror word \t{}", vec_bool_to_string(&encode));

    print!("\nerror bit {}\t", bit);
    println!("{:>bit$}^", "");

    match code.decode(encode) {
        Ok(decoded) => {
            print!("\ndecoded word ");

            for elem in &decoded {
                print!("{}", *elem as i32);
            }
        }
        Err(err) => {
            println!("Decoding error: {:?}", err);
        }
    }
}

fn vec_bool_to_string(vec: &Vec<bool>) -> String {
    vec.clone()
        .into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect()
}
