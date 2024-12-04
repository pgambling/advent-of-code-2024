const XMAS: &str = "XMAS";

pub fn count_xmas(char_matrix: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in 0..char_matrix[0].len() {
        for col in 0..char_matrix.len() {
            let current_char = char_matrix[row][col];
            if current_char == 'X' {
                count += count_words(&char_matrix, row, col);
            }
        }
    }

    count
}

fn count_words(char_matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let row_len = char_matrix[0].len();
    let col_len = char_matrix.len();
    let mut count = 0;
    for row_dir in -1..=1 {
        for col_dir in -1..=1 {
            let word =
                get_possible_word(char_matrix, row_len, col_len, row, col, row_dir, col_dir, 4);
            if word == XMAS {
                count += 1;
            }
        }
    }
    count
}

fn get_possible_word(
    char_matrix: &Vec<Vec<char>>,
    row_len: usize,
    col_len: usize,
    current_row: usize,
    current_col: usize,
    row_dir: isize,
    col_dir: isize,
    num_chars: usize,
) -> String {
    let mut word = String::new();
    let mut new_row: isize = current_row as isize;
    let mut new_col: isize = current_col as isize;
    for _ in 0..num_chars {
        word.push(char_matrix[new_row as usize][new_col as usize]);
        new_row = new_row + row_dir;
        new_col = new_col + col_dir;
        if new_row == row_len as isize || new_col == col_len as isize || new_row < 0 || new_col < 0
        {
            return word;
        }
    }
    word
}
