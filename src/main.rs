use std::io;
use rand::prelude::*;
use std::process::exit;

fn game_mode()
{
    println!("How many players (1 or 2):");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    if guess == 1
    {
        // println!("One player mode selected");
        println!("Only a two player mode is currently supported");
    } 
    else if guess == 2
    {
        println!("Two player mode selected");
    } 
    else 
    {
        println!("Please select between 1 or 2");
    }
}

fn print_hint_board()
{
    let array: [[&str; 3]; 3] = [["LT", "MT", "RT"], ["LM", "MM", "RM"], ["LB", "MB", "RB"]];
    println!("{}", format!(" {} | {} | {}", array[0][0], array[0][1], array[0][2]));
    println!("----+----+----");
    println!("{}", format!(" {} | {} | {}", array[1][0], array[1][1], array[1][2]));
    println!("----+----+----");
    println!("{}", format!(" {} | {} | {}", array[2][0], array[2][1], array[2][2]));
    println!();
}

fn print_game_board(array: [[char; 3]; 3])
{
    println!("{}", format!(" {} | {} | {}", array[0][0], array[0][1], array[0][2]));
    println!("---+---+---");
    println!("{}", format!(" {} | {} | {}", array[1][0], array[1][1], array[1][2]));
    println!("---+---+---");
    println!("{}", format!(" {} | {} | {}", array[2][0], array[2][1], array[2][2]));
    println!();
}

fn make_choice(array: &mut [[char; 3]; 3], ch: char, name: &String)
{
    println!("{} - {} - pick a spot", name, format!("{}", ch));
    println!();
    print_hint_board();
    let mut choice: String = String::new(); 
    io::stdin().read_line(&mut choice);
    let choice: String = choice.trim().to_uppercase();
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

fn assert_win(array: &mut [[char; 3]; 3], ch: char) -> bool
{
    let mut row_loop: bool = true;
    let mut column_loop: bool = true;
    let mut accross_loop: bool = true;

    // Check same row
    for row in &mut *array 
    {
        for spot in row 
        {
            if *spot != ch
            {
                row_loop = false;
                break;
            }
        }
        if row_loop == true
        {
            return true;
        }
        row_loop = true;
    }

    // Check same column
    for i in 0..=2
    {
        for j in 0..=2
        {
            if array[i][j] != ch
            {
                column_loop = false;
                break;
            }
        }
        if column_loop == true
        {
            return true;
        }
        column_loop = true;
    }

    // Check across
    for j in 0..=2
    {
        if array[j][j] != ch
        {
            accross_loop = false;
            break;
        }
    }

    if accross_loop == true
    {
        return true;
    }

    accross_loop = true;
    let mut counter: usize = 2;

    for g in 0..=2
    {
        if array[g][counter] != ch
        {
            accross_loop = false;
            break;
        }
        counter -= 1;
    }
    if accross_loop == true
    {
        return true;
    }

    return false;
}

fn main() {
    println!("Welcome to the Tic Tac Toe game!");
    game_mode();

    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("Enter player's 1 name:");
    io::stdin().read_line(&mut player1);
    let player1 = player1.trim().to_string();
    println!("Enter player's 2 name:");
    io::stdin().read_line(&mut player2);
    let player2 = player2.trim().to_string();

    let mut start: bool = rand::random();
    let o_name: String;
    let x_name: String;
    if start {
        println!("{} starts", player2);
        o_name = player2;
        x_name = player1;
    } else {
        println!("{} starts", player1);
        o_name = player1;
        x_name = player2;
    }

    let mut array: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];

    loop {
        if start
        {
            make_choice(&mut array, 'o', &o_name);
            print_game_board(array);
            if assert_win(&mut array, 'o')
            {
               println!("{} won!!!", &o_name); 
               exit(0);
            }
        } else {
            make_choice(&mut array, 'x', &x_name);
            print_game_board(array);
            if assert_win(&mut array, 'x')
            {
               println!("{} won!!!", &x_name); 
               exit(0);
            }
        }
        start = !start;
    }

}
