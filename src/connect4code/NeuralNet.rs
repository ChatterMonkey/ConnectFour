extern crate ndarray;
extern crate termcolor;
extern crate random_integer;

use super::Matrix::*;
use super::Board::*;



// ANN dimensions

pub const INODES:usize =  7;
pub const H1NODES:usize = 10;
pub const H2NODES:usize = 10;
pub const H3NODES:usize = 10;
pub const ONODES:usize = 10;



pub struct NeuralNet{

    pub w_ih1: matrix,
    pub w_h1h2: matrix,
    pub w_h2h3: matrix,
    pub w_h3o: matrix,

}


impl Player for NeuralNet{
    fn query(& self, board:&mut Board)-> usize{
        // convert board to a 1d array
        let inputs = &board.data;
        println!("inputs are {:#?}",inputs);

        //insure that the matrices are compatable;
        println!("len of inputs {}", inputs.len());
        assert_eq!(self.w_ih1.cols, inputs.len());
        println!("w_ih1");
        self.w_ih1.print_matrix();




        // multiply the inputs by the weights from the input layer to the hidden layer
        let inputs_to_h1 = self.w_ih1.times_a_vector(inputs);
        println!("{:#?}", inputs_to_h1);
        let outputs_from_h1 = sigmoid(&inputs_to_h1);
        println!(" outputs_from_h1 {:#?}", outputs_from_h1);
        // inputs into 2nd hidden layer are the outputs from h1 times the weights from h1 to h2
        let inputs_to_h2 = self.w_h1h2.times_a_vector(&outputs_from_h1);
        println!("inputs to h2 {:#?}",inputs_to_h2);

        //apply sigmoid to get outputs from h2
        let outputs_from_h2 = sigmoid(&inputs_to_h2);
        println!("outputs from h2 {:#?}", outputs_from_h2);

        //inputs from h2 into h3 are outputs from h2 times the weights from h2 to h3
        let inputs_to_h3 = self.w_h2h3.times_a_vector(&outputs_from_h2);
        println!("inputs_to_h3 {:#?}",inputs_to_h3);
        // outputs from h3 are inputs to h3 with sigmoid applied
        let outputs_from_h3 = sigmoid(&inputs_to_h3);

        println!("output from h3 are {:#?}", outputs_from_h3);
        // inputs to output layer are made by multiplying the outputs from h3 by the weights from h3 to o
        let inputs_to_o = self.w_h3o.times_a_vector(&outputs_from_h3);
        println!("inputs to output layer {:#?}",inputs_to_o);

        // find the finnal outputs by applying sigmoid to the inputs to o

        let final_outputs = sigmoid(&inputs_to_o);
        println!("finnal outputs {:#?}", final_outputs);

        let (value, answer)= find_max_entry(&final_outputs);
        return answer;



    }
    fn add_win(&mut self, points: usize){}

}
pub fn sigmoid(vec:&Vec<f32>)->Vec<f32>{
    let mut outputs:Vec<f32> = vec![];
    for i in 0..vec.len(){
        let new_value = 1.0/(1.0 + ((-vec[i]).exp()));
        outputs.push(new_value);

    }
    return outputs;
}

pub fn find_max_entry(vector: &Vec<f32>)-> (f32, usize){
    let mut temp_max = vector[0];
    let mut position = 0;
    for item in 0..vector.len(){
        if vector[item] > temp_max{
            temp_max = vector[item];
            position = item;
        }
    }
    return (temp_max,position);
}




