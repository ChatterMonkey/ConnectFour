# ConnectFour
Implements a genetic algorithm to produce a resultant neural network designed to play Connect Four.

In this file, you will find 
 * [Project walkthrough](https://github.com/ChatterMonkey/ConnectFour/blob/Simple_Model_fixed/README.md#project-walk-through)
 * [What is a Neural Network?](https://github.com/ChatterMonkey/ConnectFour/blob/Simple_Model_fixed/README.md#what-is-a-neural-network)
 * [Code Implementations](https://github.com/ChatterMonkey/ConnectFour/blob/Simple_Model_fixed/README.md#code-implementations)
 * [References](https://github.com/ChatterMonkey/ConnectFour/blob/Simple_Model_fixed/README.md#references)

For the files referenced in the [code implementations](https://github.com/ChatterMonkey/ConnectFour/blob/Simple_Model_fixed/README.md#codeimplementations) section, please see [connect4code under src](https://github.com/ChatterMonkey/ConnectFour/tree/Simple_Model_fixed/src/connect4code) in my repository. (to read main.rs, go here, one directory up: [main.rs](https://github.com/ChatterMonkey/ConnectFour/tree/Simple_Model_fixed/src))


## Project Walk Through

Please click here: [Link to my video](https://www.youtube.com/watch?v=MCsVVSn5-bU )
 or search https://www.youtube.com/watch?v=MCsVVSn5-bU  to view my video on YouTube.
 
 ## What is a Neural Network?

![Diagram of neural network](https://github.com/ChatterMonkey/ConnectFour/blob/Simple_Model_fixed/images/neuralNetwork_diagram.jpeg)


A Neural network consists of hidden layers sandwiched between input and output layers. Each neuron in a given layer is connected to every neuron in the next layer. These pathways are moderated by link weights, the values of which determine how the neural network behaves. The neural network is queried by giving values to the input neurons. When these values travel along the connections to the next layer, they are multiplied by the link weight of the associated link. In the next layer, each neuron sums the values it receives from the previous layer and applies the sigmoid function `y = 1/(1 + e^(-x))`. This process is repeated, and the values at the output neurons are the neural network’s answer to the query.



 




## CODE IMPLEMENTATIONS


#### main.rs
Accepts command line inputs as arguments to determine the mutation magnitude of the finnal generation (lower number means sharper taper) and takes a name of the run. Mutation magnitude is a decaying exponential as e^{-x*a} where x is the generation number and a is the natural logarithm of the value you input, divided by the negation of the number of total gernetions being run. 

To call main.rs, navigate to the ConnectFour directory and type  
`$ cargo run --release folder_name 0.01`

#### connect4code
Folder containing files called by main.
 
#### mod.rs
Declares the .rs files in connect4code to be public modules

#### Matrix.rs
Functions for handeling matrixes
* Declares the public struct `matrix` to be a vector of f32 values, as well as width and height dimentions.
* `zeros_matrix` initilized a matrix with 0 in every entry
* `get` returns the value of a matrix at (y,x) 
* `set` changes the value of a matrix at (y,x)
* `times_a_vector` mutipies a matrix by a vector
		
#### Board.rs
Manages the Connect Four board.
* Defines the trait player as an object with `query` and `add_win` implementations
* Creates the public enum 'Pieces' which enumerates piece types `Player1`,`Player2`, and `Nada`
* Defines `Board` type as a `matrix`, see **Matrix.rs**
* `print_board` function renders type `Board` in colored ASCCI for human use
* `place_piece` function finds the appropriate (y,x) pair given a move in the form of a column 0-6, and places a piece there
* `check_for_win` recieves a location (y,x) and checks if that piece has won the game.
* `play_connect_four` facilitates a game of connect four between to objects with the `Player` trait, and rewards them based of the outcome of the game.

#### FakeHuman.rs
Implements a logic based human simulator. Struct `FakeHuman` implements the `Player` trait.

#### Human.rs
Implements the `Player` trait for a human user, allowing me to personally play both `FakeHuman` 
						and any neural network.

#### NeuralNet.rs
Simulates a neural network 
* Defines the public struct `NeuralNet` as having a name, point value, and four weight matrices
* Creates the public enum `Layers` which enumerates layer types `WIH1`,`WH1H2`, `WH2H3`, and `WH3O`
* `find_max_entry` finds the maximum entry in a vector (used to determine the neural network's answer to a query)
* `mutate`, which changes every weight in each weight matrix by a random value within the given mutation magnitude
* `zeros_neural_net`, which initilizes a neural network with a zeros_matrix of the appropreate size as every weight matrix
* `serialize`, which uses the serde crate to write this neural network to a file
* The neural network struct also has the `Player` trait, and uses `times_a_vector` and `sigmoid` to 
* implement the querying of a digital neural network.


#### file_manager.rs
Contains functions for managing files				
* `Reconstitute` uses the serde crate to read a neural network from a file and recreate it
* `write_text` writes text to a file
* `read_text` reads text from a file
* `write_usize` writes a usize to a file
					 
					 
#### Params.rs
Holds all the hyperparameters for the genetic algorithm and the neural networks in one place.





## References

Books:
* Rust reference book
* Make your own neural network
* Text adventure game in python

Github page, python code
https://github.com/makeyourownneuralnetwork/makeyourownneuralnetwork/blob/master/part3_neural_network_mnist_data_with_rotations.ipynb 

Google tensorflow info
https://developers.google.com/machine-learning/crash-course/first-steps-with-tensorflow/toolkit 

AlphaGo site
https://deepmind.com/blog/alphago-zero-learning-scratch/

Tic tac toe neural network
https://users.auth.gr/kehagiat/Research/GameTheory/12CombBiblio/TicTacToe.pdf

MIT info about networks
https://www.technologyreview.com/s/513696/deep-learning/

Genetic algorithm
https://blog.coast.ai/lets-evolve-a-neural-network-with-a-genetic-algorithm-code-included-8809bece164

Yann LeCun, with minst training and test sets
http://yann.lecun.com

Blog post
https://blog.coast.ai/lets-evolve-a-neural-network-with-a-genetic-algorithm-code-included-8809bece164


AlphaGo vs Lee Sedol Match
https://en.wikipedia.org/wiki/AlphaGo_versus_Lee_Sedol

AlphaZero
https://www.nytimes.com/2018/12/26/science/chess-artificial-intelligence.html

Very helpful info on ndarray crate
https://docs.rs/ndarray/*/ndarray/doc/ndarray_for_numpy_users/index.html

Checkers term project
http://www.cs.uwc.ac.za/~dboonzaaier/docs/projterm4.pdf

Scholarly Articles:

https://www.ijcai.org/Proceedings/89-1/Papers/122.pdf

https://ieeexplore.ieee.org/abstract/document/273950

https://ieeexplore.ieee.org/abstract/document/784219

https://www.sciencedirect.com/science/article/pii/S0378437100004799

http://www.cs.bham.ac.uk/~xin/papers/published_iproc_sep99.pdf


http://repository.bilkent.edu.tr/bitstream/handle/11693/24903/Using%20genetic%20algorithms%20to%20select%20architecture%20of%20a%20feedforward%20artificial%20neural%20network.pdf?sequence=1 



https://ieeexplore.ieee.org/abstract/document/155366




:octocat: 