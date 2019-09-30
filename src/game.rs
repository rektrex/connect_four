use std::io;
use super::board::{ Board, Marker };

fn get_size() -> (i8, i8) {
    loop {
        println!("Enter the number of rows and columns:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let rc: Vec<&str> = input.split(" ").collect();
        let rows: i8 = match rc[0].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter valid integers");
                continue;
            },
        };

        let columns: i8 = match rc[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter valid integers");
                continue;
            },
        };

        if rows >= 4 && columns >= 4 && rows + columns >= 9 {
            return (rows, columns);
        }

        println!("Size must be atleast equal to 5x4 or 4x5");
    }
}

fn get_move(board: &mut Board, marker: &Marker) -> i8 {
    loop {
        println!("Your turn, {}, enter a column:", marker);

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let column: i8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid integer");
                continue;
            }
        };

        if board.add_marker(&column, marker) {
            return column;
        }

        println!("The entered column is filled, choose another column.");
    }
}

pub fn play_game() {
    let (rows, columns) = get_size();
    let mut board = Board::new(rows, columns);
    let mut marker = Marker::X;

    loop {
        let column = get_move(&mut board, &mut marker);
        board.print();

        if board.is_winner(&marker, &column) {
            println!("{} has won the game!", marker);
            break;
        }

        if board.is_filled() {
            println!("Game Over. No winners this time.");
            break;
        }

        marker = marker.invert();
    }
}
