use crate::neuralnetwork;
use rand::Rng;

#[derive(Debug)]
pub struct Creature {
    pub network: neuralnetwork::Network,
    pub body: [[u8; 8]; 8],
    pub x: f64,
    pub y: f64,
    pub energy: f64,
}

impl Creature {
    pub fn new(inputs: usize, outputs: usize) -> Creature {
        let mut rng = rand::thread_rng();
        Creature {
            network: neuralnetwork::Network::new(inputs, outputs),
            body: 
            [
                [1, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 1],
            ],
            x: rng.gen_range(0..50) as f64,
            y: rng.gen_range(0..30) as f64,
            energy: 20.0,
        }
    }
}



