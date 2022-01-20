use std::io;
use std::io::*;

fn main() {
    //welcome message
    println!("welcome to noughts and crosses");
    //make defalt array and initial variables
    let mut grid = [" "," "," "," "," "," "," "," "," ",];

    let mut player = 0;
    let mut won = false;
    let mut x= -1;
    let mut y = -1;


    //draw initial grid
    display(grid);
    while won == false{
        //get row
        let mut line = String::new();
        println!("enter the row");
        let y = std::io::stdin().read_line(&mut line).unwrap();
        //get collumn
        println!("enter the collumn");
        let x = std::io::stdin().read_line(&mut line).unwrap();

    }
}


fn display(grid:[&str; 9]) {
    println!("{}|{}|{}",grid[0],grid[1],grid[2]);
    println!("-----");
    println!("{}|{}|{}",grid[3],grid[4],grid[5]);
    println!("-----");
    println!("{}|{}|{}",grid[6],grid[7],grid[8]);
}


fn switchPlayer(player:i32) -> i32{
    if (player == 0){
        return 1;
    }
    else{
        return 0;
    }
}