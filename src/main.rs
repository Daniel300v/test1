

fn main() {
    //welcome message
    println!("welcome to noughts and crosses");
    //make defalt array
    let mut grid = [" "," "," "," "," "," "," "," "," ",];
    
    //draw initial grid
    display(grid)
}


fn display(grid:[&str; 9]) {
    println!("{}|{}|{}",grid[0],grid[1],grid[2]);
    println!("-----");
    println!("{}|{}|{}",grid[3],grid[4],grid[5]);
    println!("-----");
    println!("{}|{}|{}",grid[6],grid[7],grid[8]);
}