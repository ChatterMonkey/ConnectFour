extern crate ndarray;
extern crate termcolor;
extern crate random_integer;



use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ndarray::prelude::*;
use std::io;



fn get_human_move(board:&Board) -> usize{
    //get a move from a human player


    loop{
        println!("Move: ");
        let mut guess = String::new();



        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");


        let guess:usize = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input. Try again sucker.");
                continue
            },
        };


        if 0<(guess +1) && guess< COLUMNS{
            //if board[[0,guess]] == 0.0{
            if board.get(0,guess) ==0.0{
                return guess;
            }
            else{
                println!("this column is full");
                continue;
            }


        }
        else{
            println!("Invalid Input. Try again sucker.");
            continue;
        }
    }
}