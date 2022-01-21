fn main() {
    //welcome message
    println!("welcome to noughts and crosses");
    //make defalt array and initial variables
    let mut grid = [[" "," "," "],[" "," "," "],[" "," "," "]];

    let mut player = 1;
    let mut won = false;
    let mut draw = false;
    let mut x= 1;
    let mut y = 1;
    let mut valid = false;


    //draw initial grid
    display(grid);

    while won == false && draw == false{
        player = switch_player(player);
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

        
        display(grid);
        

        won = check_win_conditions(grid);
        draw = check_draw_conditions(grid);

        valid = false;
        

    }

    if draw == true{
        println! ("Draw");
    }
    else{
        if player == 0{
            println!("O wins");
        }
        else{
            println!("X wins");
        }
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

fn check_win_conditions(grid : [[&str; 3]; 3]) -> bool{
    let mut won = false;
    //let mut index = 0;
    //let mut index2 = 0;
    let players = ["X","O"];

    for index2 in 0..2{
        //collomn checker
        for index in 0..3  {
            if grid[0][index] == players[index2] && grid[1][index] == players[index2] && grid[2][index] == players[index2]{
                won = true;
            }
        }
        //row checker
        for index in 0..3  {
            if grid[index][0] == players[index2] && grid[index][1] == players[index2] && grid[index][2] == players[index2]{
                won = true;
            }
        }
        //diagonal checker
        if grid[0][0] == players[index2] && grid[1][1] == players[index2] && grid[2][2] == players[index2]{
            won = true;
        }
        //invirse diagonal checker
        if grid[0][2] == players[index2] && grid[1][1] == players[index2] && grid[2][0] == players[index2]{
            won = true;
        }

    }
    return won;
}

fn check_draw_conditions(grid : [[&str; 3]; 3]) -> bool{
    let mut draw = true;

    //collomn checker
    for index in 0..3  {
        if grid[0][index] == " " || grid[1][index] == " " || grid[2][index] == " "{
            draw = false;
        }
    }
    //row checker
    for index in 0..3  {
        if grid[index][0] == " "|| grid[index][1] == " " || grid[index][2] == " "{
            draw = false;
        }
    }
    //diagonal checker
    if grid[0][0] == " " || grid[1][1] == " " || grid[2][2] == " "{
        draw = false;
    }
    //invirse diagonal checker
    if grid[0][2] == " " || grid[1][1] == " " || grid[2][0] == " "{
        draw = false;
    }

    return draw;
}


