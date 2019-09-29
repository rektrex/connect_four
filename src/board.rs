use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Marker {
    X,
    O,
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Marker::X => write!(f, "W"),
            Marker::O => write!(f, "O"),
        }
    }
}

pub struct Board {
    pub rows: u8,
    pub columns: u8,
    pub markers: HashMap<(u8, u8), Marker>
}

impl Board {
    pub fn new(rows: u8, columns: u8) -> Board {
        Board {
            rows,
            columns,
            markers: HashMap::new(),
        }
    }

    pub fn add_marker(&mut self, column: &u8, marker: &Marker) -> bool {
        match self.find_top(&column) {
            Some(row) => {
                self.markers.insert((row, *column), *marker);
                true
            },
            None => false,
        }
    }

    fn find_top(&self, column: &u8) -> Option<u8> {
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
}
