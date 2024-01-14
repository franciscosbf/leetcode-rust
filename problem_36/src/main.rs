pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::{BTreeMap, BTreeSet};

    fn map_of_sets() -> BTreeMap<u8, BTreeSet<char>> {
        BTreeMap::from_iter((0..=8).map(|column| (column, BTreeSet::<char>::new())))
    }

    let mut columns_check = map_of_sets();
    let mut square_check = map_of_sets();

    for (row, row_values) in board.iter().enumerate() {
        let mut row_check = BTreeSet::<char>::new();
        for (column, &value) in row_values.iter().enumerate() {
            if !value.is_numeric() {
                continue;
            }
            let square = (row as u8 / 3) * 3 + (column as u8 / 3);
            if !row_check.insert(value)
                || !columns_check
                    .get_mut(&(column as u8))
                    .unwrap()
                    .insert(value)
                || !square_check.get_mut(&square).unwrap().insert(value)
            {
                return false;
            }
        }
    }

    true
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("{}", is_valid_sudoku(board));
}
