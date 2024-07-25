use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::{
    io::{stdin, stdout, Read, Stdout, Write},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut stdo = stdout();
    let board = make_board(20, 10);

    loop {
        let input = get_input(&mut stdo);
        draw_board(&board, input);

        //thread::sleep(Duration::from_secs(1));
        print!("\n\n\n")
    }
}

fn get_input(stdo: &mut Stdout) -> String {
    let mut input = String::new();
    stdo.flush().unwrap();

    stdin()
        .read_line(&mut input)
        .expect("Did not enter a string");

    return input;
}

fn make_board(height: usize, width: usize) -> Vec<Vec<u16>> {
    let board = vec![vec![0; width]; height];
    return board;
}

fn draw_board(board: &Vec<Vec<u16>>, input: String) {
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
            //fun_print(&mut stdo)
            stdo.write_all(input.as_bytes()).unwrap();
        }
        stdo.write_all("\n".as_bytes()).unwrap();
    }

    stdo.queue(cursor::RestorePosition).unwrap();
    stdo.flush().unwrap();
    thread::sleep(Duration::from_millis(128));

    stdo.queue(cursor::RestorePosition).unwrap();
    stdo.queue(terminal::Clear(terminal::ClearType::FromCursorDown))
        .unwrap();

    stdo.execute(cursor::Show).unwrap();
}

fn fun_print(stdo: &mut Stdout) {
    let output = char::from_digit(Instant::now().elapsed().as_nanos() as u32 % 127, 10);
    let matched_output = match output {
        Some(val) => val,
        None => 'x',
    };
    stdo.write_all(format!("{} ", matched_output).as_bytes())
        .unwrap();
}
