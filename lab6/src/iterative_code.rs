use std::error::Error;
use std::{fmt, vec};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IterativeCode {
    pub data: Vec<bool>,
    pub rows: usize,
    pub cols: usize,
    pub code_size: usize,
    pub horizontal_parity: Vec<bool>,
    pub vertical_parity: Vec<bool>,
}

impl IterativeCode {
    #[allow(dead_code)]
    pub fn new(data_str: &str, rows: usize, cols: usize) -> Self {
        assert_eq!(data_str.len(), rows * cols, "Invalid data string length");
        let data = data_str.chars().map(|c| c == '1').collect();
        IterativeCode {
            data,
            rows,
            cols,
            horizontal_parity: vec![false; rows],
            vertical_parity: vec![false; cols],
            code_size: (cols + 1) * (rows + 1) - 1,
        }
    }

    pub fn new_empty(rows: usize, cols: usize) -> Self {
        IterativeCode {
            data: vec![false; rows * cols], // Initialize data with false values
            rows,
            cols,
            code_size: (cols + 1) * (rows + 1) - 1,
            horizontal_parity: vec![false; rows],
            vertical_parity: vec![false; cols],
        }
    }

    pub fn compute_parity(&mut self) {
        self.horizontal_parity = vec![false; self.rows];
        self.vertical_parity = vec![false; self.cols];

        for row in 0..self.rows {
            for column in 0..self.cols {
                let index = row * self.cols + column;

                self.vertical_parity[column] ^= self.data[index];
                self.horizontal_parity[row] ^= self.data[index];
            }
        }
    }

    pub fn encode(&mut self, input_data: Vec<bool>) -> Vec<bool> {
        self.data = input_data;

        self.compute_parity();

        let mut result = self.data.clone();

        result.extend(&self.horizontal_parity);
        result.extend(&self.vertical_parity);

        result
    }

    pub fn decode(&mut self, input_data: Vec<bool>) -> Result<Vec<bool>, DecodeError> {
        if input_data.len() > self.code_size {
            return Err(DecodeError {
                message: String::from("Input data size is bigger that code size"),
            });
        }
        let data_bits = self.rows * self.cols;

        let (data, parity) = input_data.split_at(data_bits);

        self.data = data.to_vec();

        let (old_rows, old_column) = (parity[..self.rows].to_vec(), parity[self.rows..].to_vec());
        self.compute_parity();

        println!("\nrecomputed\n{}", self);

        let (mut row_errors, mut col_errors) = (vec![], vec![]);

        for (i, (&old, &new)) in old_rows
            .iter()
            .zip(self.horizontal_parity.iter())
            .enumerate()
        {
            if old != new {
                row_errors.push(i);
            }
        }

        for (j, (&old, &new)) in old_column
            .iter()
            .zip(self.vertical_parity.iter())
            .enumerate()
        {
            if old != new {
                col_errors.push(j);
            }
        }

        match (row_errors.len(), col_errors.len()) {
            (0, 0) => {
                println!("No errors detected.");
                Ok(self.data.clone())
            }
            (1, 1) => {
                let row = row_errors[0];
                let col = col_errors[0];
                let error_index = row * self.cols + col;
                self.data[error_index] = !self.data[error_index];
                println!(
                    "Error corrected at row {}, col {}, index {}",
                    row, col, error_index
                );
                Ok(self.data.clone())
            }
            (1, _) => {
                let row = row_errors[0];
                self.horizontal_parity[row] = !self.horizontal_parity[row];
                println!("Row parity bit corrected for row {}", row);
                Ok(self.data.clone())
            }
            (_, 1) => {
                let col = col_errors[0];
                self.vertical_parity[col] = !self.vertical_parity[col];
                println!("Column parity bit corrected for col {}", col);
                Ok(self.data.clone())
            }
            _ => Err(DecodeError {
                message: String::from("Multiple or ambiguous errors detected, cannot correct"),
            }),
        }
    }
}

impl fmt::Display for IterativeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = row * self.cols + col;
                table.push_str(&format!("{}", if self.data[index] { '1' } else { '0' }));
            }
            // Add vertical parity bit at the end of each row
            table.push_str(&format!(
                "│{}",
                if self.horizontal_parity[row] {
                    '1'
                } else {
                    '0'
                }
            ));
            table.push('\n');
        }

        // Add a row of horizontal lines before the horizontal parity row
        table.push_str(&"─".repeat(self.cols));
        table.push_str("┘\n");

        // Add the horizontal parity row
        for col in 0..self.cols {
            table.push_str(&format!(
                "{}",
                if self.vertical_parity[col] { '1' } else { '0' }
            ));
        }
        table.push('\n');

        write!(f, "{}", table)
    }
}
#[derive(Debug)]
pub struct DecodeError {
    message: String,
}

impl Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
