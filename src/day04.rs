#[derive(Debug)]
struct Board {
    board: Vec<Vec<Option<usize>>>,
}

impl Board {
    pub fn new(board: Vec<Vec<Option<usize>>>) -> Self {
        Self { board }
    }

    pub fn mark(&mut self, number: usize) {
        for row in &mut self.board {
            for num in row.iter_mut() {
                if let Some(n) = num {
                    if *n == number {
                        *num = None;
                    }
                } else {
                };
            }
        }
    }

    pub fn unmarked_sum(&self) -> usize {
        self.board.concat().iter().fold(
            0usize,
            |acc, x| {
                if let Some(n) = x {
                    acc + n
                } else {
                    acc
                }
            },
        )
    }

    pub fn bingo(&self) -> bool {
        let row_len = self.board[0].len();
        for row in &self.board {
            if row.iter().all(|x| x.is_none()) {
                return true;
            }
        }

        for col in 0..row_len {
            if self
                .board
                .concat()
                .iter()
                .skip(col)
                .step_by(row_len)
                .all(|x| x.is_none())
            {
                return true;
            }
        }
        false
    }
}

fn string_to_vec_nums(s: &str) -> Vec<Option<usize>> {
    let mut v = Vec::<Option<usize>>::new();
    s.split_whitespace()
        .for_each(|x| v.push(Some(x.to_string().parse::<usize>().unwrap())));
    v
}

fn parse_lines(lines: &[String]) -> (Vec<usize>, Vec<Board>) {
    let mut iter = lines.iter();
    let bingo_numbers_str = &iter
        .by_ref()
        .take_while(|x| !x.is_empty())
        .cloned()
        .next()
        .unwrap();
    let mut boards: Vec<Board> = Vec::new();
    let _ = &iter.next();

    while iter.len() > 0 {
        let mut board_rows: Vec<Vec<Option<usize>>> = Vec::new();
        let board = &iter
            .by_ref()
            .take_while(|x| !x.is_empty())
            .cloned()
            .collect::<Vec<String>>();
        board
            .iter()
            .for_each(|x| board_rows.push(string_to_vec_nums(x)));
        boards.push(Board::new(board_rows));
    }

    let bingo_numbers: Vec<usize> = bingo_numbers_str
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    (bingo_numbers, boards)
}

fn part1(lines: &[String]) -> usize {
    let (numbers, mut boards) = parse_lines(lines);

    for num in numbers {
        boards.iter_mut().for_each(|board| board.mark(num));
        for board in &boards {
            if board.bingo() {
                return board.unmarked_sum() * num;
            }
        }
    }

    0
}

fn part2(lines: &[String]) -> usize {
    let (numbers, mut boards) = parse_lines(lines);
    let mut num_iter = numbers.iter();
    let mut last_number = 0;

    while boards.len() > 1 {
        let last_number = *num_iter.next().unwrap();
        boards.iter_mut().for_each(|board| board.mark(last_number));
        boards.retain(|board| !board.bingo());
    }

    let last_board = &mut boards[0];
    while !last_board.bingo() {
        last_number = *num_iter.next().unwrap();
        last_board.mark(last_number);
    }

    last_board.unmarked_sum() * last_number
}

pub fn run(lines: &[String]) {
    println!("[Day 04] Part 1: {}", part1(lines));
    println!("[Day 04] Part 2: {}", part2(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11  0".to_string(),
            " 8  2 23  4 24".to_string(),
            "21  9 14 16  7".to_string(),
            " 6 10  3 18  5".to_string(),
            " 1 12 20 15 19".to_string(),
            "".to_string(),
            " 3 15  0  2 22".to_string(),
            " 9 18 13 17  5".to_string(),
            "19  8  7 25 23".to_string(),
            "20 11 10 24  4".to_string(),
            "14 21 16 12  6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ]
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input()), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input()), 1924);
    }
}
