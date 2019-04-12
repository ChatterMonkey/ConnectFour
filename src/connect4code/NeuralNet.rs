extern crate ndarray;
extern crate termcolor;
extern crate random_integer;
use serde::{Serialize, Deserialize};
use super::file_manager::*;
use super::Matrix::*;
use super::Matrix::matrix;
use super::Board::*;

use rand::Rng;



use rand::prelude::*;
use super::Params::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeuralNet{
    pub name: String,
    pub points: usize,
    pub w_ih1: matrix,
    pub w_h1h2: matrix,
    pub w_h2h3: matrix,
    pub w_h3o: matrix,


}
pub enum Layers{
    WIH1,
    WH1H2,
    WH2H3,
    WH3O,

}


impl Player for NeuralNet{
    fn query(& self, board:&mut Board)-> usize{
        // convert board to a 1d array

        let inputs = &board.data;
  //      println!("inputs {:?}", inputs);


        //insure that the matrices are compatable;
//        println!("len of inputs {}", inputs.len());
        assert_eq!(self.w_ih1.cols, inputs.len());

        // multiply the inputs by the weights from the input layer to the hidden layer
        let inputs_to_h1 = self.w_ih1.times_a_vector(inputs);

        let outputs_from_h1 = sigmoid(&inputs_to_h1);

        // inputs into 2nd hidden layer are the outputs from h1 times the weights from h1 to h2



        let inputs_to_h2 = self.w_h1h2.times_a_vector(&outputs_from_h1);



        //apply sigmoid to get outputs from h2



        let outputs_from_h2 = sigmoid(&inputs_to_h2);





        //inputs from h2 into h3 are outputs from h2 times the weights from h2 to h3



        let inputs_to_h3 = self.w_h2h3.times_a_vector(&outputs_from_h2);



        // outputs from h3 are inputs to h3 with sigmoid applied



        let outputs_from_h3 = sigmoid(&inputs_to_h3);



        // inputs to output layer are made by multiplying the outputs from h3 by the weights from h3 to o



        let inputs_to_o = self.w_h3o.times_a_vector(&outputs_from_h3);






        // find the finnal outputs by applying sigmoid to the inputs to o

        let final_outputs = sigmoid(&inputs_to_o);
     //   println!("finnal outputs are {:?}", final_outputs);
//        println!("finnal outputs {:#?}", final_outputs);
        let mut final_outputs_copy = final_outputs.clone();

        for output in 0..final_outputs.len(){
            if check_if_column_is_full(board, output) ==true{
                final_outputs_copy[output] = -100.0;
            }

        }
        let mut possible_move = false;
        for output in 0..final_outputs_copy.len(){
            if final_outputs_copy[output] != -100.0{
                possible_move = true;
            }
        }
        if possible_move == false{
            println!("A NEURAL NETWORK HAS NO MOVE TO MAKE!")
        }


        let (value, answer)= find_max_entry(&final_outputs_copy);
        return answer;



    }
    fn add_win(&mut self, points: usize){
        self.points = self.points + points;
    }

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


impl NeuralNet{
    pub fn serialize(&mut self, path:String){

        write_text(&serde_json::to_string(&self).unwrap(), &path);

    }


    pub fn mutate(&mut self, mutation_magnitude:f32){
        let mut value:f32;
        let mut orig:f32;
        let layer_id = [&mut self.w_ih1, &mut self.w_h1h2, &mut self.w_h2h3, &mut self.w_h3o];

        for layer in 0..4{
            for y in 0..layer_id[layer].rows{
                for x in 0..layer_id[layer].cols{
                    value = rand::thread_rng().gen();
                    //println!("value increment is {}", value);
                    orig = layer_id[layer].get(y,x);

                    if (value+orig)*mutation_magnitude > 1.0 {
                        layer_id[layer].set(y,x,1.0);

                    }
                    if(value+orig)*mutation_magnitude < -1.0 {
                        layer_id[layer].set(y,x,-1.0);

                    }
                    else{
                        layer_id[layer].set(y,x,(value+orig)*mutation_magnitude);
                    }



                   // println!(" set to {}",(value+orig)*mutation_magnitude)
                }
            }
        }


    }


    pub fn zeros_neural_net()-> NeuralNet{

        let ann = NeuralNet{name: String::from("def_name"), points:0, w_ih1: matrix::zeros_matrix(INODES, H1NODES), w_h1h2: matrix::zeros_matrix(H1NODES,H2NODES), w_h2h3: matrix::zeros_matrix(H2NODES, H3NODES), w_h3o: matrix::zeros_matrix(H3NODES, ONODES)};
        ann
    }


}

