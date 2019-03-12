# ConnectFour
Implements a genetic algorithm to produce a resultant neural network designed to play Connect Four.

CODE IMPLEMENTATIONS:


main.rs -> Accepts command line inputs as arguments to determine the mutation magnitude of the finnal generation (lower number means sharper taper) and takes a name of the run. To call, navigate to the directory and type 'cargo run --release folder_name x.xxx'. 

connect4code -> Folder containing files called by main.

mod.rs -> Declares the .rs files in connect4code to be public modules

Board.rs -> Manages the Connect Four board.
					 --> Defines the trait player as an object with 'query' and 'add_win' implementations
					 --> Creates the public enum 'Pieces' which enumerates piece types 'Player1', 'Player2', and 'Nada'
					 --> Defines 'Board' type as a matrix, see Matrix.rs
					 --> print_board function renders type Board in colored ASCCI 
					 --> place_piece function finds the appropriate (y,x) pair given a move in the form of a column 0-6, and places a 								 piece there
					 --> check_for_win recieves	a location (y,x) and checks if that piece has won the game. This function is broken up 								 into  check_for_vertical_win, check_for_horizontal_win, left_count_piece, right_count_piece, check_for_diagonal_win_ur, and check_for_diagonal_win_dr, 


In order to simulate a neural network and implement a genetic algorithm, I wrote code in Rust (a statically typed programing language similar to C and C++) that preformed the following functions:

The moderator executes the genetic algorithm by managing the games played by neural networks in the generational pool, and calling on the seed to mutate itself to create the next generation

board.rs contains several decelerations, including that of the player trait, which applies to objects with implementations of query and add_win. Also here are functions which place a piece on a board given a column to move in, print the board in colored ASCCI, check if a column is filled and check if a piece placed wins the game.

matrix.rs contains the underlying structure of the connector board as well as functions which let you multiply two matrices, multiply a matrix by a vector, get the value at location (y,x), set the value at location (y,x) to something, create a matrix from a vector of values, and initialize a matrix filled with zeros.
NeuralNet.rs defines the neural network struct and its implementations with include the player trait, as well as functions for mutating the neural network, sterilizing a neural network, and initializing a zeros neural network.
FakeHuman.rs implements the player trait for a player struct. FakeHuman answers query by following a conventional logic based method.
file_manager.rs contains functions for reading, and writing from files. In addition, it contains a deserialization function for a neural network.
human.rs implements the player trait for a human, allowing me to personally play against neural networks and fake human.
params.rs contains all of the hyperparameters for the genetic algorithm. 
main.rs accepts command line inputs as arguments to determine the mutation magnitude of the finnal generation (lower number means sharper taper) and the name of the run. 
