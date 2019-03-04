pub mod connect4code;
extern crate ndarray;
extern crate termcolor;
extern crate random_integer;
extern crate file;
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;
use serde::Serialize;
use serde::Deserialize;
use crate::connect4code::Board::Player;


use std::time::{Duration, Instant};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::thread;
//use std::time::Duration;


fn main() {


    let now = Instant::now();



    connect4code::Moderator::execute_genetic_algorithm(String::from("scores.txt"));



    println!("{}", now.elapsed().as_secs());

    let mut test_board = connect4code::Matrix::matrix::zeros_matrix(7,6);
    test_board.set(0,0,1.0);








   // connect4code::Board::play_connect_four(&human, &ANN);
    //connect4code::Board::play_connect_four(&human, &fakey);








}
