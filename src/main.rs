fn main() {
    //welcome message
    println!("welcome to noughts and crosses");
    //make defalt array and initial variables
    let mut grid = [[" "," "," "],[" "," "," "],[" "," "," "]];

    let mut player = 0;
    let mut won = false;
    let mut x= 1;
    let mut y = 1;
    let mut valid = false;


    //draw initial grid
    display(grid);

    while won == false{
        while valid == false{
            //get row
            println!("enter the row");
            y = get_number();
            //get collumn
            println!("enter the collumn");
            x = get_number();
            
            
            if grid[y][x] == " "{
                valid = true;
            }  

        }
        
        if player == 1{
            grid[y][x] = "X";
        }
        else{
            grid[y][x] = "O";
        }

        player = switch_player(player);
        display(grid);
        valid = false;
    }
    
}


fn display(grid:[[&str; 3]; 3]) {
    println!("{}|{}|{}",grid[0][0],grid[0][1],grid[0][2]);
    println!("-----");
    println!("{}|{}|{}",grid[1][0],grid[1][1],grid[1][2]);
    println!("-----");
    println!("{}|{}|{}",grid[2][0],grid[2][1],grid[2][2]);
}


fn switch_player(player:i32) -> i32{
    if player == 0{
        return 1;
    }
    else{
        return 0;
    }
}

fn get_number() -> usize{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    return buffer.trim().parse::<usize>().unwrap();
}