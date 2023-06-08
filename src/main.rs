use std::io;
use rand::prelude::*;

fn game_mode()
{
    println!("How many players (1 or 2):");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    if guess == 1
    {
        println!("One player mode selected");
    } else if guess == 2
    {
        println!("Two player mode selected");
    } else {
        println!("Please select between 1 or 2");
    }
}

fn print_hint_board()
{
    let mut array: [[&str; 3]; 3] = [["LT", "MT", "RT"], ["LM", "MM", "RM"], ["LB", "MB", "RB"]];
    println!("{}", format!(" {} | {} | {}", array[0][0], array[0][1], array[0][2]));
    println!("----+----+----");
    println!("{}", format!(" {} | {} | {}", array[1][0], array[1][1], array[1][2]));
    println!("----+----+----");
    println!("{}", format!(" {} | {} | {}", array[2][0], array[2][1], array[2][2]));
}

fn print_game_board(array: [[char; 3]; 3])
{
    println!("{}", format!(" {} | {} | {}", array[0][0], array[0][1], array[0][2]));
    println!("---+---+---");
    println!("{}", format!(" {} | {} | {}", array[1][0], array[1][1], array[1][2]));
    println!("---+---+---");
    println!("{}", format!(" {} | {} | {}", array[2][0], array[2][1], array[2][2]));
}

fn make_choice(array: &mut [[char; 3]; 3], ch: char)
{
    println!("{} - Pick a spot", format!("{}", ch));
    print_hint_board();
    let mut choice = String::new(); 
    io::stdin().read_line(&mut choice);
    choice.trim().to_uppercase();
    if choice.starts_with("L")
    {
        if choice.contains("T")
        {
            array[0][0] = ch;

        } else if choice.contains("M")
        {
            array[1][0] = ch;
        } else if choice.contains("B")
        {
            array[2][0] = ch;
        }
    } else if choice.starts_with("M")
    {
        if choice.contains("T")
        {
            array[0][1] = ch;
        } else if choice.contains("M")
        {
            array[1][1] = ch;
        } else if choice.contains("B")
        {
            array[2][1] = ch;
        }
    } else if choice.starts_with("R")
    {
        if choice.contains("T")
        {
             array[0][2] = ch;
        } else if choice.contains("M")
        {
             array[1][2] = ch;
        } else if choice.contains("B")
        {
             array[2][2] = ch;
        }
    }
    println!();
}

fn main() {
    println!("Welcome to the Tic Tac Toe game!");
    game_mode();

    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("Enter player's 1 name:");
    io::stdin().read_line(&mut player1);
    player1.trim();
    println!("Enter player's 2 name:");
    io::stdin().read_line(&mut player2);
    player2.trim();

    let mut start: bool = rand::random();
    if start {
        println!("{} starts", player2);
    } else {
        println!("{} starts", player1);
    }

    let mut array: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];

    loop {
        if start
        {
            make_choice(&mut array, 'o');
        } else {
            make_choice(&mut array, 'x');
        }
        print_game_board(array);
        println!();
        start = !start;
    }

}
