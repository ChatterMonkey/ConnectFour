use super::Params::*;
use super::NeuralNet::NeuralNet;
use std::string::*;
use std::fmt::Write;
use super::Board::play_connect_four;

use crate::connect4code::Board::Player;

pub fn execute_genetic_algorithm(){
    let initial_seed = NeuralNet::zeros_neural_net();
    for generation in 0..NUMBER_OF_GENERATIONS{

        let mut pool = vec![initial_seed.clone()];

        for member in 0..GENERATION_SIZE{
            let mut seed_copy = initial_seed.clone();
            seed_copy.mutate(1.0);
            let mut net_name = String::new();
            writeln!(net_name, "{}_{}",member,generation).unwrap();


            seed_copy.name = net_name;
            pool.push(seed_copy);

        }
   //     println!("done");
        pool.push(initial_seed.clone());
    //    println!("done pushing");


        for i in 0..GENERATION_SIZE+1{
            for j in i+1..GENERATION_SIZE+1{

                let (player1_score, player2_score) = play_connect_four(&pool[i], &pool[j], false);
                {
                    let mut net1 = &mut pool[i];

                    net1.add_win(player1_score);
                   // println!("net1 score: {}", net1.points);
                }

                {
                    let mut net2 = &mut pool[j];
                    net2.add_win(player2_score);

                }

                let (player2_score, player1_score) = play_connect_four(&pool[j], &pool[i], false);
                {
                    let mut net1 = &mut pool[j];
                    net1.add_win(player1_score);
                }
                {
                    let mut net2 = &mut pool[i];
                    net2.add_win(player2_score);

                }

            }
        }
     //   println!("done playing")

        for net in 0..GENERATION_SIZE{
            print!(" {} ", pool[net].points)
        }
        println!("");

    }

    println!("genetic algorithm done");

}