use super::Params::*;
use super::NeuralNet::NeuralNet;
use std::string::*;


use super::Board::play_connect_four;
use super::Matrix::matrix;
use crate::connect4code::Board::Player;
use super::FakeHuman::FakeHuman;
use std::f32::*;
use std::u64::*;
use std::thread;
use std::f64::consts::E;
use super::file_manager::*;
use std::fs::*;
use std::fmt::Write;
use std::io::prelude::*;
pub fn execute_genetic_algorithm(scores_file_path_name:String, starting_net:NeuralNet){

    let mut scores_file = File::create(scores_file_path_name).unwrap();
    let benchmark = FakeHuman{};
    let mut initial_seed = starting_net;

    let a:f32 = -((0.0001f32).ln())/(NUMBER_OF_GENERATIONS as f32); //adjust the rate of change for the mutation_magnitude


    for generation in 0..NUMBER_OF_GENERATIONS{
        let gen32 = generation as f32;

        let mut pool = vec!(initial_seed);

        for member in 0..GENERATION_SIZE{

            let mut seed_copy = pool[0].clone();

            let mutation_magnitude = (-1.0*gen32*a).exp();
            println!("mutation magnitude = {}", mutation_magnitude);

            seed_copy.mutate(mutation_magnitude);



            let mut net_name = String::new();
            writeln!(net_name, "{}_{}",member,generation).unwrap();


            seed_copy.name = net_name;

            pool.push(seed_copy);

        }  // create the pool



        for i in 0..GENERATION_SIZE+1{
            for j in 0..GENERATION_SIZE+1{
                if i == j{
                    continue;
                }
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
            }
        } // Battle!


        let winner_index = highest_point_network(&pool);
        initial_seed = pool[winner_index].clone();

        if generation%10 == 0 {
            //for member in &pool{
          //      println!("net score in pool is {}", &member.points);
         //   }
            let (bench_points1, seed_points1) = play_connect_four(&benchmark, &initial_seed,false);
            let (bench_points2, seed_points2) = play_connect_four(&benchmark, &initial_seed,false);
            let (bench_points3, seed_points3) = play_connect_four(&benchmark, &initial_seed,false);
          //  println!(" 1fakehuman points{} ann points{}", bench_points1, seed_points1);
          //  println!("2fakehuman points{} ann points{}", bench_points2, seed_points2);
          //  println!("3fakehuman points{} ann points{}", bench_points3, seed_points3);

            let mut data = (seed_points1+seed_points2+seed_points3)/3;
          //  println!("wrote {} to file, {} won", data, winner_index);
            write_usize(data,&mut scores_file);

            if generation%500 ==0{
                pool[winner_index + 1].clone().serialize(format!("GW{}", generation))

            }

        }

        initial_seed.points = 0;






    }

    println!("genetic algorithm done");

}


pub fn highest_point_network( list:&Vec<NeuralNet> )-> usize{
    let mut winner_index = 0;
    for i in 1..list.len(){
        if &list[i].points > &list[winner_index].points{
            winner_index = i;
        }
    }
    return winner_index;


} //get the index of the ANN with the most points