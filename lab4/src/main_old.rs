use std::error::Error;
use std::{ cmp, fmt };

#[derive(Debug, PartialEq, Eq, Clone)]
struct IterativeCode {
    data: Vec<bool>,
    rows: usize,
    cols: usize,
    horizontal_parity: Vec<bool>,
    vertical_parity: Vec<bool>,
    main_diag_parity: Vec<bool>,
    sec_diag_parity: Vec<bool>,
}

impl IterativeCode {
    fn new(data_str: &str, rows: usize, cols: usize) -> Self {
        assert_eq!(data_str.len(), rows * cols, "Invalid data string length");
        let data = data_str
            .chars()
            .map(|c| c == '1')
            .collect();
        IterativeCode {
            data,
            rows,
            cols,
            horizontal_parity: vec![false; rows],
            vertical_parity: vec![false; cols],
            main_diag_parity: vec![false; cmp::min(rows, cols)],
            sec_diag_parity: vec![false; cmp::min(rows, cols)],
        }
    }

    fn compute_parity(&mut self) {
        for row in 0..self.rows {
            for column in 0..self.cols {
                let index = row * self.cols + column;
                let min_axis = cmp::min(self.rows, self.cols);

                self.horizontal_parity[row] ^= self.data[index];
                self.vertical_parity[column] ^= self.data[index];

                self.main_diag_parity[
                    ((row as isize) - (column as isize)).rem_euclid(min_axis as isize) as usize
                ] ^= self.data[index];
                self.sec_diag_parity[(row + column).rem_euclid(min_axis)] ^= self.data[index];
            }
        }
    }

    fn encode(&mut self) -> Vec<bool> {
        self.compute_parity();

        let mut result = self.data.clone();

        result.extend(&self.horizontal_parity);
        result.extend(&self.vertical_parity);
        result.extend(&self.main_diag_parity);
        result.extend(&self.sec_diag_parity);

        result
    }

    fn decode(&mut self) -> Result<(), DecodeError> {
        let transf_horizontal_parity = &self.horizontal_parity.clone();
        let transf_vertical_parity = &self.vertical_parity.clone();
        let transf_main_diag_parity = &self.main_diag_parity.clone();
        let transf_sec_diag_parity = &self.sec_diag_parity.clone();

        let min_axis = cmp::min(self.rows, self.cols);

        self.horizontal_parity = vec![false; self.rows];
        self.vertical_parity = vec![false; self.cols];
        self.main_diag_parity = vec![false; min_axis];
        self.sec_diag_parity = vec![false; min_axis];

        self.compute_parity();

        let mut row_errors = Vec::new();
        let mut col_errors = Vec::new();
        let mut main_diag_errors = Vec::new();
        let mut sec_diag_errors = Vec::new();

        for i in 0..self.rows {
            if transf_horizontal_parity[i] != self.horizontal_parity[i] {
                row_errors.push(i);
            }
        }

        for j in 0..self.cols {
            if transf_vertical_parity[j] != self.vertical_parity[j] {
                col_errors.push(j);
            }
        }

        for k in 0..min_axis {
            if transf_main_diag_parity[k] != self.main_diag_parity[k] {
                main_diag_errors.push(k);
            }
            if transf_sec_diag_parity[k] != self.sec_diag_parity[k] {
                sec_diag_errors.push(k);
            }
        }

        if row_errors.is_empty() && col_errors.is_empty() {
            println!("No errors detected.");
            Ok(())
        } else if row_errors.len() == 1 && col_errors.len() == 1 {
            let row = row_errors[0];
            let col = col_errors[0];
            let error_index = row * self.cols + col;
            self.data[error_index] = !self.data[error_index];

            println!("row {}, col{}", row, col);
            println!("element {}", error_index);
            Ok(())
        } else {
            Err(DecodeError {
                message: String::from("Multiple or ambiguous errors detected"),
            })
        }
    }
}

impl fmt::Display for IterativeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_str: String = self.data
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect();
        write!(f, "{}", data_str)
    }
}

#[derive(Debug)]
struct DecodeError {
    message: String,
}

impl Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn main() {
    let original_data_str = "010111001111";
    let mut code = IterativeCode::new(original_data_str, 3, 4);
    let mut res = code.encode();
    println!("Encoded codeword: \t {}", code);

    // Симулируем ошибку
    let bit = 7;
    res[bit] = !res[bit];
    // code.data[6] = !code.data[6];

    code.data = res.clone();
    if let Err(err) = code.decode() {
        println!("Decoding failed: {}", err);
    } else {
        println!("Decoding successful:\t {}", code);
    }
}
