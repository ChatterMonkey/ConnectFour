//kick start branch

pub mod connect4code;
extern crate ndarray;
extern crate termcolor;
extern crate random_integer;
extern crate file;
extern crate beep;
extern crate dimensioned;
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


use dimensioned::si;
//use std::time::Duration;


fn main() {
    println!("Even later than 'Even later than 'Latest version''. Ha. ");
    /*
    let net_text = connect4code::file_manager::read_text(&String::from("/Users/mayabasu/CLionProject/ConnectFour/cephalopod/arm1_GW366000")).unwrap();
    let net = connect4code::file_manager::reconstitute(&net_text);
    let net2_text = connect4code::file_manager::read_text(&String::from("/Users/mayabasu/CLionProject/ConnectFour/cephalopod/arm2_GW317500")).unwrap();
    let mut net2 = connect4code::file_manager::reconstitute(&net_text);
    let human = connect4code::Human::Human{points:0};
    let human2 = connect4code::Human::Human{points:0};
    let fakehuman = connect4code::FakeHuman::FakeHuman{};
    let fakehuman2 = connect4code::FakeHuman::FakeHuman{};
    */
   //Un comment from here

    let args: Vec<String> = env::args().collect();
    let id_string = &args[1];
    let end_goal_string = &args[2];

    let end_goal:f32 = end_goal_string.trim().parse().expect("please enter a value with type f32");


    let now = Instant::now();



    // to here before starting






//    let mut input_string = String::new();
//    println!("At the 100,000th generation, the mutation magnitude should be:");
    //   io::stdin().read_line(&mut input_string).expect("failed to read line");




    //let human = connect4code::Human::Human{ points: 0 };
    //let net_text = connect4code::file_manager::read_text(&String::from("gmins_1.1_GW100000")).unwrap();

    //let net = connect4code::file_manager::reconstitute(&net_text);
   // loop{
   //     connect4code::Board::play_connect_four(&human, &net, true);

  //  }
    let starting_net = connect4code::NeuralNet::NeuralNet::zeros_neural_net();

   // also UNCOMMENT!

    let starting_generation = 1;


    connect4code::Moderator::execute_genetic_algorithm( starting_net, id_string.to_string(),end_goal, starting_generation);

    //TO HERE



    //new comment
    println!("Time to run: {}s", now.elapsed().as_secs());





}
