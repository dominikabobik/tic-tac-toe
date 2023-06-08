fn print_board(array: [[char; 3]; 3])
{
    println!("{}", format!(" {} | {} | {}", array[0][0], array[0][1], array[0][2]));
    println!("---+---+---");
    println!("{}", format!(" {} | {} | {}", array[1][0], array[1][1], array[1][2]));
    println!("---+---+---");
    println!("{}", format!(" {} | {} | {}", array[2][0], array[2][1], array[2][2]));
}

fn main() {
    println!("Hello, world!");
    let mut array: [[char; 3]; 3] = [['o', 'x', 'o'], ['o', 'x', 'o'], ['o', 'x', 'o']];
    print_board(array);
}
