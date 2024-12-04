pub fn count_x_mas(char_matrix: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in 1..char_matrix[0].len() - 1 {
        for col in 1..char_matrix.len() - 1 {
            if is_x_mas(&char_matrix, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_x_mas(char_matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if char_matrix[row][col] != 'A' {
        return false;
    }

    let corner_chars = format!(
        "{}{}{}{}",
        char_matrix[row - 1][col - 1],
        char_matrix[row - 1][col + 1],
        char_matrix[row + 1][col - 1],
        char_matrix[row + 1][col + 1]
    );
    ["MMSS", "SSMM", "MSMS", "SMSM"].contains(&corner_chars.as_str())
}
