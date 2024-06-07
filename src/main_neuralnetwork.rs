use core::num;
use std::{borrow::Borrow, iter::zip};
use rand::prelude::*;
use rongor::{draw, prelude::*};

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
fn tanh(x: f64) -> f64 {
    x.tanh()
}
fn relu(x: f64) -> f64 {
    x.max(0.0)
}
fn swish(x: f64) -> f64 {
    x / (1.0 + (-x).exp())
}
fn gelu(x: f64) -> f64 {
    0.5 * x * (1.0 + (x / (2.0f64).sqrt() * (std::f64::consts::PI / 8.0).sqrt()).tanh())
}


#[derive(PartialEq, Clone, Debug)]
enum ActivationFunction {
    sigmoid,
    tanh,
    relu,
    leaky_relu,
    prelu,
    softmax,
    swish,
    gelu,
    none
}

#[derive(Debug)]
struct Node {
    NumOfNodesInPreviousLayer: u32,
    ActivationFunction: ActivationFunction,
    Wheights: Vec<f64>,
    Bias: f64,
    Connections: Vec<bool>
}

impl Node {
    fn new(
        NumOfNodesInPreviousLayer: u32,
        ActivationFunction: ActivationFunction,
        Wheights: Vec<f64>,
        Bias: f64,
        Connections: Vec<bool>
    )
    -> Self
    {
        Node {
            NumOfNodesInPreviousLayer,
            ActivationFunction,
            Wheights,
            Bias,
            Connections
        }
    }

    fn get_output(&self, inputs: Vec<f64>) -> f64 {
        let mut wheighted_output = 0.;

        for ((wheight, input), connected) in zip(zip(self.Wheights.clone(), inputs), self.Connections.clone()) {
            wheighted_output += input * wheight * 
            if connected {
                1.
            }
            else {
                0.
            };
        }

        println!("{}", wheighted_output);

        wheighted_output += self.Bias;


        println!("{} {}", self.Bias, wheighted_output);

        wheighted_output = 
        if self.ActivationFunction == ActivationFunction::sigmoid {
            sigmoid(wheighted_output)
        } else if self.ActivationFunction == ActivationFunction::tanh {
            tanh(wheighted_output)
        } else if self.ActivationFunction == ActivationFunction::relu {
            relu(wheighted_output)
        } else if self.ActivationFunction == ActivationFunction::swish {
            swish(wheighted_output)
        } else if self.ActivationFunction == ActivationFunction::gelu {
            gelu(wheighted_output)
        } else if self.ActivationFunction == ActivationFunction::none {
            wheighted_output
        } else {
            panic!("no activation function selected")
        };

        wheighted_output
    }
}

#[derive(Debug)]
struct Layer 
{
    NumOfNodesInPreviousLayer: u32,
    NumOfNodesInThisLayer: u32,
    ActivationFunction: ActivationFunction,
    Wheights: Vec<Vec<f64>>,
    Biases: Vec<f64>,
    Connections: Vec<Vec<bool>>
}

impl Layer 
{
    fn new(
        NumOfNodesInPreviousLayer: u32,
        NumOfNodesInThisLayer: u32,
        ActivationFunction: ActivationFunction,
        Wheights: Vec<Vec<f64>>,
        Biases: Vec<f64>,
        Connections: Vec<Vec<bool>>
    )
    -> Self
    {
        Layer {
            NumOfNodesInPreviousLayer,
            NumOfNodesInThisLayer,
            ActivationFunction,
            Wheights,
            Biases,
            Connections
        }
    }

    fn get_nodes(&self) -> Vec<Node> {
        let mut nodes = vec![];

        for ((connections, wheights), Bias) in zip(zip(self.Connections.clone(), self.Wheights.clone()), self.Biases.clone()) {
            nodes.push(Node::new(self.NumOfNodesInPreviousLayer, self.ActivationFunction.clone(), wheights, Bias, connections))
        }

        nodes
    }

    fn get_output(&self, inputs: Vec<f64>) -> Vec<f64> 
    {
        let nodes = self.get_nodes();

        let mut outputs = vec![];

        for node in nodes {
            outputs.push(node.get_output(inputs.clone()));
        }

        outputs
    }
}

#[derive(Debug)]
struct NeuralNetwork {
    NumOfInputNodes: u32,
    HIddenLayers: Vec<u32>,
    NumOfOutputNodes: u32,
    ActivationFunction: ActivationFunction,
    Wheights: Vec<Vec<Vec<f64>>>,
    Biases: Vec<Vec<f64>>,
    Connections: Vec<Vec<Vec<bool>>>
}

impl NeuralNetwork {
    fn new(
        NumOfInputNodes: u32,
        HIddenLayers: Vec<u32>,
        NumOfOutputNodes: u32,
        ActivationFunction: ActivationFunction,
        Wheights: Vec<Vec<Vec<f64>>>,
        Biases: Vec<Vec<f64>>,
        Connections: Vec<Vec<Vec<bool>>>
    )
    -> NeuralNetwork
    {
        NeuralNetwork {
            NumOfInputNodes,
            HIddenLayers,
            NumOfOutputNodes,
            ActivationFunction,
            Wheights,
            Biases,
            Connections
        }
    }

    fn get_layers(&self) -> Vec<Layer> {
        let mut layers = vec![];

        let mut last_num_of_nodes = self.NumOfInputNodes;

        for i in 0..self.HIddenLayers.len() {
            let NumOfNodesInPreviousLayer = 
            if i == 0 {
                self.NumOfInputNodes
            } else {
                self.HIddenLayers[i - 1]
            };

            let layer = Layer::new(NumOfNodesInPreviousLayer, self.HIddenLayers[i], self.ActivationFunction.clone(),  self.Wheights[i].clone(),  self.Biases[i].clone(),  self.Connections[i].clone());

            layers.push(layer);

            last_num_of_nodes = self.HIddenLayers[i];
        }

        let output_layer = Layer::new(last_num_of_nodes, self.NumOfOutputNodes, self.ActivationFunction.clone(), self.Wheights[self.Wheights.len() - 1].clone(), self.Biases[self.Biases.len() - 1].clone(), self.Connections[self.Connections.len() - 1].clone());
        layers.push(output_layer);

        layers
    }

    fn get_output(&self, inputs: Vec<f64>) -> Vec<f64> {
        let layers = self.get_layers();

        let mut outputs = inputs;

        for layer in layers {
            outputs = layer.get_output(outputs);
        }

        outputs
    }
}


fn generate_wheights(
    min: f64, 
    max: f64, 
    NumOfInputNodes: u32,
    HIddenLayers: Vec<u32>,
    NumOfOutputNodes: u32,
)
-> Vec<Vec<Vec<f64>>>
{
    let mut rng: ThreadRng = thread_rng();

    let mut wheights = vec![];

    let mut numnodes = HIddenLayers;
    numnodes.append(&mut vec![NumOfOutputNodes]);

    for (x, num) in numnodes.iter().enumerate() {
        wheights.push(vec![]);

        for i in 0..*num {
            wheights[x].push(vec![]);

            let numprevnodes =
            if x == 0 {
                NumOfInputNodes
            } else {
                numnodes[i as usize - 1]
            };

            for _ in 0..numprevnodes {
                wheights[x][i as usize].push(rng.gen::<f64>() * (max - min) - min.abs())
            }
        }
    }

    wheights
}

fn generate_connections(
    NumOfInputNodes: u32,
    HIddenLayers: Vec<u32>,
    NumOfOutputNodes: u32,
)
-> Vec<Vec<Vec<bool>>>
{
    let mut connections = vec![];

    let mut numnodes = HIddenLayers;
    numnodes.append(&mut vec![NumOfOutputNodes]);

    for (x, num) in numnodes.iter().enumerate() {
        connections.push(vec![]);

        for i in 0..*num {
            connections[x].push(vec![]);

            let numprevnodes =
            if x == 0 {
                NumOfInputNodes
            } else {
                numnodes[i as usize - 1]
            };

            for _ in 0..numprevnodes {
                connections[x][i as usize].push(true);
            }
        }
    }

    connections
}

fn generate_biases(
    min: f64, 
    max: f64, 
    NumOfInputNodes: u32,
    HIddenLayers: Vec<u32>,
    NumOfOutputNodes: u32,
)
-> Vec<Vec<f64>>
{
    let mut rng: ThreadRng = thread_rng();

    let mut biases = vec![];

    let mut numnodes = HIddenLayers;
    numnodes.append(&mut vec![NumOfOutputNodes]);

    for (x, num) in numnodes.iter().enumerate() {
        biases.push(vec![]);

        for _ in 0..*num {
            biases[x].push(rng.gen::<f64>() * (max - min) - min.abs());
        }
    }

    biases
}

async fn run(window: Window){     
    let default_camera = Camera2d::default();



    let mut NN = NeuralNetwork::new(2, vec![], 2, ActivationFunction::sigmoid, generate_wheights(-5., 5., 2, vec![], 2), generate_biases(-5., 5., 2, vec![], 2), generate_connections(2, vec![], 2));

    
    let mut slider1 = Slider::new(Vec2::newi(200, 30), Vec2::newi(1730, 200), 3., Color::GRAY, Color::DARKGRAY, Color::BLACK, 0.5, 17., Color::WHITE, Color::BLACK, 3.);
    let mut slider2 = Slider::new(Vec2::newi(200, 30), Vec2::newi(1730, 250), 3., Color::GRAY, Color::DARKGRAY, Color::BLACK, 0.5, 17., Color::WHITE, Color::BLACK, 3.);
    let mut slider3 = Slider::new(Vec2::newi(200, 30), Vec2::newi(1730, 300), 3., Color::GRAY, Color::DARKGRAY, Color::BLACK, 0.5, 17., Color::WHITE, Color::BLACK, 3.);
    let mut slider4= Slider::new(Vec2::newi(200, 30), Vec2::newi(1730, 350), 3., Color::GRAY, Color::DARKGRAY, Color::BLACK, 0.5, 17., Color::WHITE, Color::BLACK, 3.);

    let mut slider5 = Slider::new(Vec2::newi(200, 30), Vec2::newi(1730, 450), 3., Color::GRAY, Color::DARKGRAY, Color::BLACK, 0.5, 17., Color::WHITE, Color::BLACK, 3.);
    let mut slider6 = Slider::new(Vec2::newi(200, 30), Vec2::newi(1730, 500), 3., Color::GRAY, Color::DARKGRAY, Color::BLACK, 0.5, 17., Color::WHITE, Color::BLACK, 3.);


    let graph_size = Vec2::new(1920. / 1.3, 1080. / 1.3);
    let graph_pos = Vec2::new(65., 1080. - 65. - graph_size.y);

    let resolution = 100;

    while window.update_screen().await {
        draw2d::clear(Color::DARKBROWN);

        let delta = window.get_delta_time();

        for x in (graph_pos.x as i32..graph_pos.x as i32 + graph_size.x as i32).step_by(resolution) {
            for y in (graph_pos.y as i32..graph_pos.y as i32 + graph_size.y as i32).step_by(resolution) {
                let inp1 = (x - graph_pos.x as i32 / graph_size.x as i32) * 10;
                let inp2 = (y - graph_pos.y as i32 / graph_size.y as i32) * 10;

                let output = NN.get_output(vec![inp1 as f64, inp2 as f64]);
                
                if output[0] > output[1] {
                    draw2d::square::square_top_left(Vec2::newi(x, y), resolution as f32, resolution as f32, Color::GREEN);
                } else {
                    draw2d::square::square_top_left(Vec2::newi(x, y), resolution as f32, resolution as f32, Color::RED);
                }
            }
        }
        
        slider1.draw(10);
        slider2.draw(10);
        slider3.draw(10);
        slider4.draw(10);


        slider5.draw(10);
        slider6.draw(10);


        let w1 = slider1.fill_percent * 10. - 5.;
        let w2 = slider2.fill_percent * 10. - 5.;
        let w3 = slider3.fill_percent * 10. - 5.;
        let w4 = slider4.fill_percent * 10. - 5.;

        let b1 = slider5.fill_percent * 10. - 5.;
        let b2 = slider6.fill_percent * 10. - 5.;


        NN.Wheights[0][0][0] = w1 as f64;
        NN.Wheights[0][0][1] = w2 as f64;
        NN.Wheights[0][1][0] = w3 as f64;
        NN.Wheights[0][1][1] = w4 as f64;

        NN.Biases[0][0] = b1 as f64;
        NN.Biases[0][1] = b2 as f64;

        draw2d::square::square_top_left_lines(graph_pos, graph_size.x + resolution as f32, graph_size.y + resolution as f32, 10., Color::WHITE);

        default_camera.set_as_current_cam();
    }
}






#[rongor::main]
async fn main() {
    start
    (
        run, 
        "title".to_string(), 
        1920, 
        1080, 
        false, 
        true, 
        true,
        AntialiasingType::Lanczos3
    ).await;
}

