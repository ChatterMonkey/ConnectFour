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

    thread::spawn(||{
        connect4code::Moderator::execute_genetic_algorithm();
    });

    connect4code::Moderator::execute_genetic_algorithm();
    println!("{}", now.elapsed().as_secs());

    let mut test_board = connect4code::Matrix::matrix::zeros_matrix(7,6);
    test_board.set(0,0,1.0);


    let w1 = connect4code::Matrix::matrix::zeros_matrix(connect4code::Params::INODES, connect4code::Params::H1NODES);
    let w2 = connect4code::Matrix::matrix::zeros_matrix(connect4code::Params::H1NODES, connect4code::Params::H2NODES);
    let w3 = connect4code::Matrix::matrix::zeros_matrix(connect4code::Params::H2NODES, connect4code::Params::H3NODES);
    let w4 = connect4code::Matrix::matrix::zeros_matrix(connect4code::Params::H3NODES, connect4code::Params::ONODES);

    let human = connect4code::Human::Human {};
    let mut ANN = connect4code::NeuralNet::NeuralNet { name:String::from("ann"), points: 0, w_ih1: w1, w_h1h2: w2, w_h2h3: w3, w_h3o: w4 };
    let fakey = connect4code::FakeHuman::FakeHuman {};
    ANN.mutate(1.0);
    write_text(&serde_json::to_string(&ANN).unwrap(), &String::from("json.txt")).expect("Could not write to file");
    ANN.query(&mut test_board.clone());
    let net_text =read_text(&String::from("json.txt")).unwrap();
    let ANN2 = reconstitute(&net_text);
    ANN2.query(&mut test_board.clone());






   // connect4code::Board::play_connect_four(&human, &ANN);
    //connect4code::Board::play_connect_four(&human, &fakey);


    let human2 = connect4code::Human::Human {};




    fn reconstitute(text: &String) -> connect4code::NeuralNet::NeuralNet {
        let deserialized: connect4code::NeuralNet::NeuralNet = serde_json::from_str(text).unwrap();
        let new_net = deserialized;
        new_net
    }
    fn write_text(text: &String, path_name: &String) -> Result<String, io::Error> {
        let s = String::new();
        let mut f = File::create(path_name)?;
        f.write_all(text.as_bytes())?;
        Ok(s)
    }

    fn read_text(path_name: &String) -> Result<String, io::Error> {
        let mut file = File::open(path_name)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents).expect("failed to read message");

        Ok(contents)
    }
}
