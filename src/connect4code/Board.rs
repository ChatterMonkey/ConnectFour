extern crate ndarray;
extern crate termcolor;
extern crate random_integer;

use super::Matrix::matrix;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


pub trait Player {
    fn query(& self,board: &mut Board)-> usize;
    fn add_win(&mut self, points: usize);
}

pub enum Pieces{
    Player1,
    Player2,
    Nada,
}
pub type Board = matrix;
pub const ROWS: usize = 6;
pub const COLUMNS: usize = 7;

pub fn print_board(board: &Board) -> &Board{
    for y in 0..ROWS {
        for x in 0..COLUMNS {
            if board.get(y,x) == 0.0{
                let mut stdout = StandardStream::stdout(ColorChoice::Always);
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))
                    .expect("Could not set color, error.");
                write!(&mut stdout, "( )");

            }
            if board.get(y,x) == -0.5{
                let mut stdout = StandardStream::stdout(ColorChoice::Always);
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .expect("Could not set color, error.");
                write!(&mut stdout, "(0)");
            }
            if board.get(y,x) == 0.5{
                let mut stdout = StandardStream::stdout(ColorChoice::Always);
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                    .expect("Could not set color, error.");

                write!(&mut stdout, "(0)").expect("Error writing to stdout");

            }


        }
        println!();
    }
    board
}



pub fn place_piece( board:&mut Board, move_column:usize, piece_type: Pieces) -> (usize,usize){

    for y in (0..ROWS).rev(){
        if board.get(y,move_column) == 0.0{

            match piece_type{
                Pieces::Player1 => {
                    board.set(y,move_column,0.5);
                }
                Pieces::Player2 => {
                    board.set(y,move_column,-0.5);
                }
                Pieces::Nada => {
                    board.set(y, move_column,0.0);
                }
            }

            return (y,move_column);
        }

    }
    return (0,move_column);


}

pub fn check_for_win(board:&Board, piece_y:usize,piece_x:usize) -> bool{
    let hoz_win = check_for_horizontal_win(board,piece_y,piece_x);
    let vet_win = check_for_vertical_win(board, piece_y, piece_x);
    let diag_ur_win = check_for_diagonal_win_ur(board, piece_y, piece_x);
    let diag_dr_win =check_for_diagonal_win_dr(board, piece_y,piece_x);
    // println!("h: {}  v: {}   r: {}  l: {}",hoz_win,vet_win, diag_ur_win, diag_dr_win);
    if (hoz_win == false) && (vet_win ==false) && (diag_ur_win ==false) && (diag_dr_win == false){
        return false;
    }
    else{
        return true;
    }


}

pub fn check_for_vertical_win( board:&Board, piece_y:usize,piece_x:usize) -> bool{
    // println!("YOu have given y:{} x: {}", piece_y, piece_x);
    //println!("rows {} , piece y {}, rows-piece_y {}",ROWS, piece_y, ROWS - piece_y);

    if (ROWS - piece_y) < 4{
        //   print!("The piece is too low to win vertically.");
        return false;
    }
    let piece_type = board.get(piece_y,piece_x);
    //   print!("The piece we think is at {} {} is {}", piece_y, piece_x, piece_type);
    for i in piece_y..piece_y+4{

        if board.get(i, piece_x) != piece_type{

            return false;
        }
    }
    return true;

}

pub fn check_for_horizontal_win( board:&Board, piece_y:usize, piece_x:usize)-> bool{

    // counting pieces of same value to the left
    let pieces_to_the_left = left_count_piece(board, piece_y, piece_x);

    // counting pieces of the same value to the right, INCLUDING THE PLACED PIECE!
    let pieces_to_the_right = right_count_piece(board, piece_y, piece_x);

    // check the total piece continum on both sides, remember, num_on_right_cen INCLUDES the placed piece

    if pieces_to_the_left + pieces_to_the_right > 3{
        true
    }
    else{
        false
    }

} //relies on left_count_piece and right_count_piece

pub fn left_count_piece(board:&Board, piece_y:usize, piece_x:usize) ->usize{
    //println!("YOu have given y:{} x: {}", piece_y, piece_x);
    //let piece_type = board[[piece_y,piece_x]];

    let piece_type = board.get(piece_y, piece_x);

    //println!("Piece type is {}", piece_type);
    let mut num_on_left:usize = 0;

    if piece_x <3{
        // println!("We think that piece_x<3");
        for  space_check in (0..piece_x).rev(){

            // println!("left reach extending to {}", space_check);
            // println!("{}", board[[piece_y, space_check]]);
            //if board[[piece_y, space_check]] == piece_type{
            if board.get(piece_y, space_check) == piece_type{
                num_on_left = num_on_left + 1;

            }
            else{
                return num_on_left;
            }

        }
        return num_on_left;
    }
    else{
        for  space_check in (piece_x-3..piece_x).rev(){

            //  println!("left reach extending to {}", space_check);
            //  println!("{}", board[[piece_y, space_check]]);
            //if board[[piece_y, space_check]] == piece_type{
            if board.get(piece_y, space_check) == piece_type{
                num_on_left = num_on_left + 1;

            }
            else{
                return num_on_left;

            }
        }
        //println!("num_on_left  = {}", num_on_left);
        return num_on_left


    }

}

pub fn right_count_piece(board:&Board, piece_y:usize, piece_x: usize)->usize{
    // println!("YOu have given y:{} x: {}", piece_y, piece_x);
    let piece_type = board.get(piece_y,piece_x);
    //  println!("Piece type is {}, checking right", piece_type);
    let mut num_on_right_cen:usize = 0;
    if piece_x +4 > COLUMNS {
        //   println!("We think that piece_x > COLUMNS -3");
        for  space_check in piece_x..COLUMNS{

            //   println!("right reach extending to {}", space_check);
            //  println!("{}", board[[piece_y, space_check]]);
            //if board[[piece_y, space_check]] == piece_type{
            if board.get(piece_y, space_check) == piece_type{
                num_on_right_cen = num_on_right_cen + 1;

            }
            else{
                return num_on_right_cen;
            }
        }
        //  println!("num_on_right_cen  = {}", num_on_right_cen);
        return num_on_right_cen;
    }
    else{
        for  space_check in piece_x..piece_x+4{

            // println!("right reach extending to {}", space_check);
            // println!("{}", board[[piece_y, space_check]]);
            //if board[[piece_y, space_check]] == piece_type{
            if board.get(piece_y, space_check)==piece_type{
                num_on_right_cen = num_on_right_cen + 1;

            }
            else{
                return num_on_right_cen;
            }
        }
        //  println!("num_on_right_cen  = {}", num_on_right_cen);
        return num_on_right_cen;
    }


}

pub fn check_for_diagonal_win_ur(board:&Board, piece_y:usize, piece_x: usize) ->bool {
    //let piece_type = board[[piece_y,piece_x]];
    let piece_type = board.get(piece_y, piece_x);

    let mut pieces_ur = 0;
    let mut pieces_dr = 0;


// this checks the diagonal up right from the piece upward
    for i in 0..4{
        //println!("i is {}",i);
        if (piece_y  < i) ||  (piece_x > COLUMNS -1 -i){
            //println!( "{} or {} is out of bounds", piece_y -i, piece_x + i);
            break;
        }
        else{
            //if board[[piece_y - i, piece_x + i]] == piece_type{
            if board.get(piece_y - i, piece_x + i) == piece_type{
                pieces_ur = pieces_ur +1;
                //          println!( "{} or {} is in bounds and has type {}", piece_y - i, piece_x + i, board[[piece_y - i, piece_x + i]]);

            }
            else{

                break;
            }
        }
    }
    //  println!("checking the other way");


// this checks from the piece down and left
    for i in 0..4{
        if  (piece_y  > ROWS -1-i) || (piece_x<i) {
            //  println!( "{}+{} or {}-{} is out of bounds", piece_y, i, piece_x, i);
            break;
        }
        else{
            //println!( "{} or {} is in bounds and has type {}", piece_y + i, piece_x - i, board[[piece_y + i, piece_x -i]]);
            //if board[[piece_y + i, piece_x - i]] == piece_type{
            if board.get(piece_y + i, piece_x - i)==piece_type{

                pieces_dr = pieces_dr +1;

            }
            else{

                break;
            }
        }
    }
    //  println!("There are {} diagonal pieces in this segment", pieces_ur + pieces_dr-1);

    if (pieces_ur + pieces_dr-1) > 3{
        return true;
    }
    else{
        return false;
    }

}


pub fn check_for_diagonal_win_dr(board:&Board, piece_y:usize, piece_x:usize) -> bool{
    let piece_type  = board.get(piece_y, piece_x);


    let mut pieces_ur = 0;
    let mut pieces_dr = 0;


// this code covers from the piece and up to the left
    for i in 0..4{
        //  println!("i is {}",i);
        if (piece_y  < i) || (piece_x <i){
            //println!( "{} or {} is out of bounds", piece_y -i, piece_x + i);
            break;
        }
        else{
            //if board[[piece_y - i, piece_x - i]] == piece_type{
            if board.get(piece_y - i, piece_x - i) == piece_type{
                pieces_ur = pieces_ur +1;
                //       println!( "{} or {} is in bounds and has type {}", piece_y - i, piece_x - i, board[[piece_y - i, piece_x - i]]);

            }
            else{

                break;
            }
        }
    }
    //  println!("checking the other way");

//covering from the piece right and down.
    for i in 0..4{
        if (piece_y  > ROWS -1-i) || (piece_x > COLUMNS -1-i){
            //println!( "{} or {} is out of bounds", piece_y + i, piece_x -i);
            break;
        }
        else{
            // println!( "{} or {} is in bounds and has type {}", piece_y + i, piece_x - i, board[[piece_y + i, piece_x -i]]);
            //if board[[piece_y + i, piece_x + i]] == piece_type{
            if board.get(piece_y + i, piece_x + i) == piece_type{

                pieces_dr = pieces_dr +1;

            }
            else{

                break;
            }
        }
    }
//    println!("There are {} diagonal pieces in this segment", pieces_ur + pieces_dr-1);

    if (pieces_ur + pieces_dr-1) > 3{
        return true;
    }
    else{
        return false;
    }
}

pub fn play_connect_four<P1: Player, P2: Player>(player1:&P1, player2:&P2, display:bool )-> (usize, usize){

    let board = &mut matrix::zeros_matrix(COLUMNS,ROWS);
    let mut game_over = false;
    let mut turns:usize = 0;


    while game_over == false{
        let player1_move = player1.query(board);
        let player1_tup = place_piece(board,player1_move,Pieces::Player1);
        game_over = check_for_win(board, player1_tup.0, player1_tup.1);
        if display{
            print_board(board);
            println!("game over = {}", game_over);

        }


        if game_over == true{
            turns = turns + 1;

            if display {
                println!("game over = {}", game_over);
            }
            return (84, turns);
            break;
        }


        let player2_move = player2.query(board);
        let player2_tup = place_piece(board, player2_move, Pieces::Player2);
        game_over = check_for_win(board, player2_tup.0, player2_tup.1);
        if game_over == true{
            turns = turns + 1;

            if display {
                println!("game over = {}", game_over);
            }
            return (turns,84);
            break;
        }

        //print_board(board);
        turns = turns + 1;
    }
    return (42,42);



}

pub fn check_if_column_is_full(board:&Board, possible_move:usize)-> bool {
    0.0 != board.get(0, possible_move)
}


