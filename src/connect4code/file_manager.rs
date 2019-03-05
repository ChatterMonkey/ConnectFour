use super::NeuralNet::NeuralNet;
use std::io::*;
use std::fs::File;
use std::io::prelude::*;

use std::io;
use std::io::Read;



pub fn reconstitute(text: &String) -> NeuralNet {
    let deserialized: NeuralNet = serde_json::from_str(text).unwrap();
    let new_net = deserialized;
    new_net
}


pub fn write_text(text: &String, path_name: &String){

    let mut f = File::create(path_name).expect("could not open file, write_text failed");
    f.write_all(text.as_bytes());

}

pub fn push_text(text: &String, path_name: &String)-> Result<String> {
    let _string = String::new();
    let mut file = File::open(path_name)?;
    file.write_all(text.as_bytes())?;
    Ok(_string)

}

pub fn read_text(path_name: &String) -> Result<String> {
    let mut file = File::open(path_name)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("failed to read message");

    Ok(contents)
}


pub fn write_usize(data:usize, file: &mut File){
    file.write_fmt(format_args!("{} \n", data));
}

