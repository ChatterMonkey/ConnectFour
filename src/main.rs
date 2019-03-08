//kick start branch

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
use std::env;

use std::time::{Duration, Instant};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::thread;
//use std::time::Duration;


fn main() {
    let args: Vec<String> = env::args().collect();
    let id_string = &args[1];
    let end_goal_string = &args[2];

//    let mut input_string = String::new();
//    println!("At the 100,000th generation, the mutation magnitude should be:");
 //   io::stdin().read_line(&mut input_string).expect("failed to read line");

    let end_goal:f32 = end_goal_string.trim().parse().expect("please enter a value with type f32");


    let now = Instant::now();


    //let human = connect4code::Human::Human{ points: 0 };
    //let net_text = connect4code::file_manager::read_text(&String::from("gmins_1.1_GW100000")).unwrap();

    //let net = connect4code::file_manager::reconstitute(&net_text);
   // loop{
   //     connect4code::Board::play_connect_four(&human, &net, true);

  //  }
    let starting_net = connect4code::NeuralNet::NeuralNet::zeros_neural_net();




    connect4code::Moderator::execute_genetic_algorithm( starting_net, id_string.to_string(),end_goal);


    //new comment
    //println!("{}", now.elapsed().as_secs());








}
