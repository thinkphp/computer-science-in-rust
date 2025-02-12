use std::io::{self, Write};

fn print_board(board: &[char; 9]) {
    for i in (0..9).step_by(3) {
        println!(" {} | {} | {} ", board[i], board[i+1], board[i+2]);
        if i < 6 {
            println!("-----------");
        }
    }
}

fn check_winner(board: &[char; 9]) -> Option<char> {
    // Check rows
    for i in (0..9).step_by(3) {
        if board[i] != ' ' && board[i] == board[i+1] && board[i+1] == board[i+2] {
            return Some(board[i]);
        }
    }

    // Check columns
    for i in 0..3 {
        if board[i] != ' ' && board[i] == board[i+3] && board[i+3] == board[i+6] {
            return Some(board[i]);
        }
    }

    // Check diagonals
    if board[0] != ' ' && board[0] == board[4] && board[4] == board[8] {
        return Some(board[0]);
    }
    if board[2] != ' ' && board[2] == board[4] && board[4] == board[6] {
        return Some(board[2]);
    }

    None
}

fn is_board_full(board: &[char; 9]) -> bool {
    !board.contains(&' ')
}

fn get_valid_input(current_player: char) -> usize {
    loop {
        print!("Player {}, enter position (1-9): ", current_player);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(position) = input.trim().parse::<usize>() {
            if position >= 1 && position <= 9 {
                return position - 1;
            }
        }
        println!("Invalid input! Please enter a number between 1 and 9.");
    }
}

fn main() {
    let mut board = [' '; 9];
    let mut current_player = 'X';

    println!("Welcome to Tic Tac Toe!");
    println!("Positions are numbered from 1-9, left to right, top to bottom.");
    println!("1 | 2 | 3");
    println!("---------");
    println!("4 | 5 | 6");
    println!("---------");
    println!("7 | 8 | 9\n");

    loop {
        print_board(&board);

        let position = get_valid_input(current_player);

        if board[position] != ' ' {
            println!("That position is already taken! Try again.");
            continue;
        }

        // Make move
        board[position] = current_player;

        // Check for winner
        if let Some(winner) = check_winner(&board) {
            print_board(&board);
            println!("Player {} wins!", winner);
            break;
        }

        // Check for draw
        if is_board_full(&board) {
            print_board(&board);
            println!("It's a draw!");
            break;
        }

        // Switch players
        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}
