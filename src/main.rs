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


    let now = Instant::now();

    let starting_net = connect4code::NeuralNet::NeuralNet::zeros_neural_net();
    let human = connect4code::Human::Human{ points: 0 };
    let fakey = connect4code::FakeHuman::FakeHuman{};
    //connect4code::Board::play_connect_four(&human, &fakey, true);

    connect4code::Moderator_benchmark::execute_benchmark_genetic_algorithm( starting_net, id_string.to_string());


    //new comment
    println!("{}", now.elapsed().as_secs());








}
