use std::io;
use super::board::{ Board, Marker };

fn get_size() -> (i8, i8) {
    loop {
        println!("Enter the number of rows and columns:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let rc: Vec<&str> = input.split(" ").collect();
        let rows: i8 = match rc[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter valid integers");
                continue;
            },
        };

        let columns: i8 = match rc[1].parse() {
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

pub fn play_game() {
}
