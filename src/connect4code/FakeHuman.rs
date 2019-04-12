extern crate ndarray;
extern crate termcolor;
extern crate random_integer;

use super::Matrix::*;
use super::Board::*;

#[derive(Clone)]
pub struct FakeHuman{}

impl Player for FakeHuman{
    fn query(& self,board: &mut Board)-> usize{
        fake_human_query(board)

    }
    fn add_win(&mut self, points:usize){

    }
}





pub fn fake_human_query( original_board: &Board)-> usize{
    let mut board_copy = original_board.clone();
    let  board = &mut board_copy;



    // determine if a move will make you win
    fn can_you_win(board_address: &mut Board, search_left_to_right: bool)-> (usize, bool) {
        if search_left_to_right == true {
            for winning_move_q in 0..COLUMNS  {
                let tup: (usize, usize, bool) = place_piece(board_address, winning_move_q, Pieces::Player1);
                if !tup.2{
                    continue;
                }
                if check_for_win(board_address, tup.0, tup.1) == true {
                    //   println!("FOUND A WINNING MOVE");
                    // board_address[[tup.0, tup.1]] = 0.0;
                    board_address.set(tup.0, tup.1, 0.0);
                    //  println!("board as is");
                    //print_board(board_address);
                    return (winning_move_q, true);
                }
                board_address.set(tup.0, tup.1, 0.0);
            }
            return (0, false);
        }
        else {
            for winning_move_q  in (0..COLUMNS).rev(){
                let tup: (usize, usize,bool) = place_piece(board_address,winning_move_q, Pieces::Player1);
                if !tup.2{
                    continue;
                }

                if check_for_win(board_address, tup.0, tup.1) == true{
                    //println!("FOUND A WINNING MOVE");
                    //board_address[[tup.0,tup.1]] = 0.0;
                    board_address.set(tup.0, tup.1, 0.0);
                    //  println!("board as is");
                    // print_board(board_address);
                    return (winning_move_q, true);
                }
                board_address.set(tup.0, tup.1, 0.0);
            }
            return (0,false);
        }
    }


    // determine if you must block
    fn need_to_block(board_address: &mut Board)-> (usize, bool){


        for blocking_move_q  in 0..COLUMNS{
            let tup: (usize, usize, bool) = place_piece(board_address,blocking_move_q, Pieces::Player2);
            if !tup.2{
                continue;
            }

            if check_for_win(board_address, tup.0, tup.1) == true{
                //   println!("FOUND A BLOCK");
                //board_address[[tup.0,tup.1]] = 0.0;
                board_address.set(tup.0, tup.1, 0.0);
                return (blocking_move_q, true);
            }
            //board_address[[tup.0,tup.1]] = 0.0;
            board_address.set(tup.0,tup.1,0.0);
        }
        return (0,false);
    }

    //Garentee a win next turn
    fn can_garentee_next_turn_win(board_address: &mut Board)-> (usize, bool){
        for first_move in 0..COLUMNS {
            let first_move_tup: (usize, usize,bool) = place_piece(board_address, first_move, Pieces::Player1);
            if !first_move_tup.2{
                continue;
            }
            let mut opponent_can_win = false;

            for winning_o_move in 0..COLUMNS  {
                let tup: (usize, usize,bool) = place_piece(board_address, winning_o_move, Pieces::Player2);
                if !tup.2{
                    continue;
                }
                if check_for_win(board_address, tup.0, tup.1) == true {
                    //    println!("Oponent could win");
                    board_address.set(tup.0, tup.1, 0.0);
                    opponent_can_win = true;
                }
                board_address.set(tup.0, tup.1, 0.0);
            }

            if opponent_can_win == false{
                //  println!("{}","before tup2");
                //  print_board(board_address);
                let tup2: (usize,bool) = can_you_win(board_address, true);
                //  println!("{}","before tup3");
                //  print_board(board_address);
                let tup3: (usize,bool) = can_you_win(board_address, false);
                //board_address[[first_move_tup.0,first_move_tup.1]]=0.0;
                board_address.set(first_move_tup.0,first_move_tup.1,0.0);
                if tup2 == tup3{
                    //    println!("either their is only one winning move that you can get from this move, of this position will not let you win next turn");
                    board_address.set(first_move_tup.0,first_move_tup.1,0.0);
                    continue;
                }
                else{
                    board_address.set(first_move_tup.0,first_move_tup.1,0.0);

                    //  println!("returning true, {}", first_move);
                    return(first_move,true);

                }

            }
            board_address.set(first_move_tup.0,first_move_tup.1,0.0);


        }
        return(0,false);
    }

  //  let mut decision:usize;

   // let verification_board = original_board.clone();

    let can_win:(usize,bool) = can_you_win(board, true);

  /*  if !verification_board.is_equal_to(board) {
        println!("Verification failed for can win!");
        println!("Modified matrix:");
        print_board(board);
        println!("Original matrix:");
        print_board(&verification_board);
        panic!();
    }

   */

    if can_win.1 == true{

        return can_win.0;
    }
    let can_block:(usize,bool) = need_to_block(board);

  /*  if !verification_board.is_equal_to(board) {
        println!("Verification failed for need to block!");
        println!("Modified matrix:");
        print_board(board);
        println!("Original matrix:");
        print_board(&verification_board);
        panic!();
    }
*/

    if can_block.1 == true{

        return can_block.0;
    }
    let can_win_in_two_turns:(usize,bool) = can_garentee_next_turn_win(board);
/*
    if !verification_board.is_equal_to(board) {
        println!("Verification failed for can win in two turns!");
        println!("Modified matrix:");
        print_board(board);
        println!("Original matrix:");
        print_board(&verification_board);
        panic!();
    }
*/

    if can_win_in_two_turns.1 == true{
        //  println!("Can win in two turns");

        return can_win_in_two_turns.0;
    }
    else{

        //  println!("my move is {}", randint);
        let mut no_legal_move = true;
        loop {
            let randint = random_integer::random_usize(0, COLUMNS-1);

            no_legal_move = check_if_column_is_full(&board_copy,randint);
            if no_legal_move == false{
                return randint;
            }
        }

    }


}

