pub mod connect4code;
extern crate ndarray;
extern crate termcolor;
extern crate random_integer;



use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ndarray::prelude::*;
use std::io;













fn main() {
   // let player1_moves = [0,0,0,0,0];
   // let player2_moves = [1,1,1,3,4];
  //  println!("hi 1");
    let n1 = connect4code::NeuralNet::NeuralNet{inodes:8,hnodes:8,onondes:2};
    let n2 = connect4code::NeuralNet::NeuralNet{inodes:8,hnodes:8,onondes:2};
    connect4code::Board::play_connect_four(n1,n2);




    let mut cupcake = connect4code::Matrix::matrix::zeros_matrix(1,2);
    cupcake.set(0,0,7.0);
    cupcake.set(1,0,6.3);
    cupcake.print_matrix();
    let mut bonzo = connect4code::Matrix::matrix::zeros_matrix(2,2);
    bonzo.set(0,0,5.0);
    bonzo.set(1,0,1.3);
    bonzo.set(0,1,7.0);
    bonzo.set(1,1,6.0);
    bonzo.print_matrix();
    println!("should have printed");









    let mut board = connect4code::Matrix::matrix::zeros_matrix(connect4code::Board::COLUMNS,connect4code::Board::ROWS);

    let ghost_board = &mut board;

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
   // let tup: (usize,usize) = (0,0);
    //print_board(board);
  //  println!("hi 2");
    loop{

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));

        println!("Robottoisimasimo");

        let human_move = connect4code::FakeHuman::fake_human_query(ghost_board);
        let tup: (usize,usize) = connect4code::Board::place_piece(ghost_board,human_move,connect4code::Board::Pieces::Player1);

        connect4code::Board::print_board(ghost_board);

        let game_over = connect4code::Board::check_for_win(ghost_board,tup.0, tup.1);
        println!("{}",game_over);

        if game_over{
            println!("And there was much rejoicing");
            break;
        }




        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));

        println!("Robo Player");

        let guess = connect4code::FakeHuman::fake_human_query(ghost_board);
        let tup: (usize,usize) = connect4code::Board::place_piece(ghost_board,guess,connect4code::Board::Pieces::Player2);

        connect4code::Board::print_board(ghost_board);

        let game_over = connect4code::Board::check_for_win(ghost_board,tup.0, tup.1);
        println!("{}",game_over);

        if game_over{
            println!("And there was much rejoicing");
            break;
        }



        //let tup: (usize,usize) = place_piece(ghost_board,player1_moves[i],Pieces::Player1);
        //print!("{}",check_for_vertical_win(ghost_board, tup.0, tup.1));
        //print_board(ghost_board);
        //let tup: (usize,usize) = place_piece(ghost_board,player2_moves[i],Pieces::Player2);
        //print!("{}",check_for_vertical_win(ghost_board, tup.0, tup.1));
        //print_board(ghost_board);

    }




}










