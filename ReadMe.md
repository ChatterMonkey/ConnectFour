# ConnectFour

<pre {}>
Implements a genetic algorithm to produce a resultant neural network designed to play Connect Four.

CODE IMPLEMENTATIONS:


main.rs - Accepts command line inputs as arguments to determine the mutation magnitude of the finnal generation (lower number 						means sharper taper) and takes a name of the run. Mutation magnitude is a decaying exponential as e^{-x*a} where x 						is the generation number and a is the natural logarithm of the value you input, divided by the negation of the
					 numberof total gernetions being run. 

To call main.rs, navigate to the ConnectFour directory and type  
$ cargo run --release folder_name 0.01

connect4code -> Folder containing files called by main.

mod.rs -> Declares the .rs files in connect4code to be public modules

Matrix.rs - Functions for handeling matrixes
					 - Declares the public struct 'matrix' to be a vector of f32 values, as well as width and height dimentions.
					 - zeros_matrix initilized a matrix with 0 in every entry
					 - get returns the value of a matrix at (y,x) 
					 - set changes the value of a matrix at (y,x)
					 - times_a_vector mutipies a matrix by a vector
		
Board.rs -> Manages the Connect Four board.
					 --> Defines the trait player as an object with 'query' and 'add_win' implementations
					 --> Creates the public enum 'Pieces' which enumerates piece types 'Player1','Player2', and 'Nada'
					 --> Defines 'Board' type as a matrix, see Matrix.rs
					 --> print_board function renders type Board in colored ASCCI for human use
					 --> place_piece function finds the appropriate (y,x) pair given a move in the form of a column 0-6, 
					     and places a piece there
					 --> check_for_win recieves a location (y,x) and checks if that piece has won the game.
					 --> play_connect_four facilitates a game of connectfour between to objects with the Player trait, 
					 		 and rewards them based of the outcome of the game.

FakeHuman.rs -> Implements a logic based human simulator. Struct FakeHuman implements the Player trait.

Human.rs -> Implements the Player trait for a human user, allowing me to personally play both FakeHuman 
						and any neural network.

NeuralNet.rs -> Simulates a neural network 
					 --> Defines the public struct NeuralNet as having a name, point value, and four weight matrices
					 --> Creates the public enum 'Layers' which enumerates layer types 'WIH1','WH1H2', 'WH2H3', and 'WH3O'
					 --> sigmoid applies the sigmoid function to every entry in a vector (used to simulate nodes)
					 --> find_max_entry finds the maximum entry in a vector 
					     (used to determine the neural network's answer to a query)
					 --> Implementations for NeuralNet include 
					 				--> mutate, which changes every weight in each weight matrix by a random value within the 
									    given mutation magnitude
									--> zeros_neural_net, which initilizes a neural network with a zeros_matrix of the 
									    appropreate size as every weight matrix
									--> serialize, which uses the serde crate to write this neural network to a file
					 --> The neural network struct also has the Player trait, and uses times_a_vector and sigmoid to 
					     implement the querying of a digital neural network.
file_manager.rs -> Contains functions for managing files				
					 --> Reconstitute uses the serde crate to read a neural network from a file and recreate it
					 --> write_text writes text to a file
					 --> read_text reads text from a file
					 --> write_usize writes a usize to a file
					 
					 
Params.rs -> Holds all the hyperparameters for the genetic algorithm and the neural networks in one place.


 
</pre>
