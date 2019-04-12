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


pub fn execute_genetic_algorithm( starting_net:NeuralNet, run_id:String, end_target_mutation_magnitude:f32, starting_generation: usize){// 0.0001f32 before
    println!("starting genetic algorithm");
    println!("the run id is {}", run_id);
    let mut benchmark_scores_file = File::create(format!("{}_benchmark_scores_bt.txt", run_id)).unwrap();
    println!("created file for benchmark scores");
    let mut peer_scores_file = File::create(format!("{}_peer_scores_bt.txt", run_id)).unwrap();
    println!("created file for peer scores");
    let benchmark = FakeHuman{};
    let mut initial_seed = starting_net;

    let a:f32 = -((end_target_mutation_magnitude).ln())/(NUMBER_OF_GENERATIONS as f32); //adjust the rate of change for the mutation_magnitude

    println!("Number of generations {}, alpha {}, end target {}", NUMBER_OF_GENERATIONS, a, end_target_mutation_magnitude);

    for generation in starting_generation..NUMBER_OF_GENERATIONS+1{
        println!("generation #{}", generation);
        //println!("next generation {}", generation);
        // create the pool
        let gen32 = generation as f32;

        let mut pool = vec!(initial_seed);

        for member in 0..GENERATION_SIZE{

            let mut seed_copy = pool[0].clone();

            let mutation_magnitude = (-1.0*gen32*a).exp();
            //println!("mutation magnitude = {}", mutation_magnitude);

            seed_copy.mutate(mutation_magnitude);



            let mut net_name = String::new();
            writeln!(net_name, "{}_{}",member,generation).unwrap();


            seed_copy.name = net_name;
            seed_copy.clone().serialize(format!("{}_intermediate_test_generation_{}_net_{}", run_id, generation, member));

            pool.push(seed_copy);

        }


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




        // pool[19].points = pool[19].points + 1000;


        let winner_index = highest_point_network(&pool);
        //  println!("winner index is {}", &winner_index);

        initial_seed = pool[winner_index].clone();
        //  println!("ownership moved");


        if generation%10 == 0 {


            //  println!("wrote {} to file, {} won", data, winner_index);
            //println!("initial seed has {}", initial_seed.points);
            write_usize(initial_seed.points,&mut peer_scores_file);

            let (bench_points1, seed_points1) = play_connect_four(&benchmark, &initial_seed,false);
            let (bench_points2, seed_points2) = play_connect_four(&benchmark, &initial_seed,false);
            let (bench_points3, seed_points3) = play_connect_four(&benchmark, &initial_seed,false);

            let mut data = (seed_points1+seed_points2+seed_points3)/3;
            // println!("data is {}", &data);
            //  println!("wrote {} to file, {} won", data, winner_index);
            write_usize(data,&mut benchmark_scores_file);


            if generation%500 ==0{
               // let (p1, p2) = play_connect_four(&initial_seed, &benchmark, true);
              //  println!("{}    {}", p1, p2);
                //println!("appending...");

                pool[winner_index ].clone().serialize(format!("{}_GW{}", run_id, generation));
                //println!("{}_GW{}", run_id, generation);

            }

        }

        initial_seed.points = 0;


    }


    println!("The genetic algorithm finished...and there was much rejoicing!");

}

pub fn highest_point_network( list:&Vec<NeuralNet> )-> usize{
    let mut winner_index = 0;

    for i in 1..list.len(){
      //  println!("{}", i);

        if &list[i].points > &list[winner_index].points{
            winner_index = i;
        }
    }
    return winner_index;


} //get the index of the ANN with the most points