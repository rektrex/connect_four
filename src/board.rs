use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Marker {
    X,
    O,
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Marker::X => write!(f, "X"),
            Marker::O => write!(f, "O"),
        }
    }
}

pub struct Board {
    pub rows: i8,
    pub columns: i8,
    pub markers: HashMap<(i8, i8), Marker>,
}

impl Board {
    pub fn new(rows: i8, columns: i8) -> Board {
        Board {
            rows,
            columns,
            markers: HashMap::new(),
        }
    }

    pub fn add_marker(&mut self, column: &i8, marker: &Marker) -> bool {
        match self.find_top(&column) {
            Some(row) => {
                self.markers.insert((row, *column), *marker);
                true
            },
            None => false,
        }
    }

    fn find_top(&self, column: &i8) -> Option<i8> {
        for i in 1 .. self.rows+1 {
            if self.markers.get(&(i, *column)).is_none() {
                return Some(i)
            }
        }
        None
    }

    pub fn print(&self) {
        for i in (1 .. self.rows+1).rev() {
            for j in 1 .. self.columns+1 {
                match self.markers.get(&(i, j)) {
                    Some(marker) => print!("{}", marker),
                    None => print!("_")
                }
            }
            println!();
        }
        println!();
    }

    fn find_contiguous_count(&self, marker: &Marker, row: &i8, column: &i8, count: i8, dir: &(i8, i8)) -> i8 {
        match self.markers.get(&((row + dir.0, column + dir.1))) {
            Some(m) => {
                if marker == m {
                    self.find_contiguous_count(marker, &(row + dir.0), &(column + dir.1), count + 1, dir)
                } else {
                    count
                }
            },
            None => count
        }
    }

    pub fn is_winner(&self, marker: &Marker, row: &i8, column: &i8) -> bool {
        let dirs: [(i8, i8); 4] = [(1, 0), (0, 1), (1, 1), (-1, 1)];

        for dir in dirs.iter() {
            let dir_reverse = (-dir.0, -dir.1);

            let count_1 = self.find_contiguous_count(&marker, &row, &column, 1, dir);
            if count_1 >= 4 {
                return true
            }

            let count_2 = self.find_contiguous_count(&marker, &row, &column, count_1, &dir_reverse);
            if count_2 >= 4 {
                return true
            }
        }
        return false
    }

    pub fn is_filled(&self) -> bool {
        for i in 1 .. self.columns+1 {
            if self.markers.get(&(self.rows, i)).is_none() {
                return false
            }
        }
        true
    }
}

// Tests
#[cfg(test)]
mod test {
    use super::{ Board, Marker };

    #[test]
    fn basics() {
        let mut board = Board::new(6, 7);
        assert_eq!(board.find_top(&1), Some(1));

        board.add_marker(&1, &Marker::O);
        board.add_marker(&1, &Marker::O);
        board.add_marker(&1, &Marker::O);
        board.add_marker(&1, &Marker::O);
        board.add_marker(&1, &Marker::O);
        let a = board.add_marker(&1, &Marker::O);

        assert_eq!(board.find_top(&1), None);
        assert!(a);

        let b = board.add_marker(&1, &Marker::X);
        assert!(!b);
    }

    #[test]
    fn print() {
        let mut board = Board::new(3, 3);
        board.add_marker(&1, &Marker::O);
        board.add_marker(&2, &Marker::X);
        board.add_marker(&2, &Marker::O);

        // Run using cargo test -- --nocapture, not very readable because of async nature of test
        board.print();
    }

    #[test]
    fn check_winner() {
        let mut board = Board::new(5, 4);

        board.add_marker(&1, &Marker::O);
        assert!(!board.is_winner(&Marker::O, &1, &1));
        board.add_marker(&1, &Marker::X);
        assert!(!board.is_winner(&Marker::O, &2, &1));
        assert!(!board.is_winner(&Marker::X, &2, &1));

        board.add_marker(&1, &Marker::O);
        board.add_marker(&1, &Marker::O);
        board.add_marker(&1, &Marker::O);
        assert!(!board.is_winner(&Marker::O, &5, &1));

        board.add_marker(&2, &Marker::O);
        board.add_marker(&3, &Marker::O);
        board.add_marker(&4, &Marker::O);
        assert!(board.is_winner(&Marker::O, &1, &4));

        board.print();
    }

    #[test]
    fn board_filled() {
        let mut board = Board::new(3, 3);

        assert!(!board.is_filled());

        for i in 1 .. 4 {
            for _ in 1 .. 4 {
                assert!(!board.is_filled());
                board.add_marker(&i, &Marker::X);
            }
        }

        assert!(board.is_filled())
    }
}
