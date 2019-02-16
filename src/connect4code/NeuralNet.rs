extern crate ndarray;
extern crate termcolor;
extern crate random_integer;


use super::Board::*;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use ndarray::prelude::*;
use std::io;
pub struct NeuralNet{
    pub inodes: usize,
    pub hnodes: usize,
    pub onondes: usize,
}


impl Player for NeuralNet{
    fn query(& self, board:&mut Board)-> usize{
        3
    }
    fn add_win(&mut self, points:usize){}
}



