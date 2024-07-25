use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::{
    char::from_digit,
    io::{stdout, Write},
    iter::Sum,
    thread,
    time::{Duration, Instant},
};

fn main() {
    let board = make_board(20, 10);

    loop {
        draw_board(&board);

        //thread::sleep(Duration::from_secs(1));
        print!("\n\n\n")
    }
}

fn make_board(height: u16, width: u16) -> Vec<Vec<u16>> {
    let board = vec![vec![0u16; width as usize]; height as usize];
    return board;
}

fn draw_board(board: &Vec<Vec<u16>>) {
    let mut stdo = stdout();

    stdo.execute(cursor::Hide).unwrap();
    // for i in (1..30).rev() {
    //     stdo.queue(cursor::SavePosition).unwrap();
    //     stdo.write_all(format!("{}: FOOBAR ", i).as_bytes())
    //         .unwrap();
    //     stdo.queue(cursor::RestorePosition).unwrap();
    //     stdo.flush().unwrap();
    //     thread::sleep(Duration::from_millis(100));

    //     stdo.queue(cursor::RestorePosition).unwrap();
    //     stdo.queue(terminal::Clear(terminal::ClearType::FromCursorDown))
    //         .unwrap();
    // }
    // stdo.execute(cursor::Show).unwrap();

    for row in board {
        stdo.write_all("\t".as_bytes()).unwrap();
        for column in row {
            if *column == 0 {
                // if (Instant::now().elapsed().as_nanos() % 2 == 0) {
                //     stdo.write_all("x ".as_bytes()).unwrap();
                // } else {
                //     stdo.write_all("f ".as_bytes()).unwrap();
                // }

                let output = char::from_digit(Instant::now().elapsed().as_nanos() as u32 % 127, 10);
                let matched_output = match output {
                    Some(val) => val,
                    None => 'x',
                };
                stdo.write_all(format!("{} ", matched_output).as_bytes())
                    .unwrap();
            }
        }

        stdo.write_all("\n".as_bytes()).unwrap();
    }

    stdo.queue(cursor::RestorePosition).unwrap();
    stdo.flush().unwrap();
    thread::sleep(Duration::from_millis(10));

    stdo.queue(cursor::RestorePosition).unwrap();
    stdo.queue(terminal::Clear(terminal::ClearType::FromCursorDown))
        .unwrap();

    stdo.execute(cursor::Show).unwrap();
}
