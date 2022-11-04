use crate::neuralnetwork;

pub struct Creature {
    pub network: neuralnetwork::Network,
    pub parts: [[u8; 8]; 8],
    pub x: f64,
    pub y: f64,
}

impl Creature {
    pub fn new(inputs: usize, outputs: usize) -> Creature {
        Creature {
            network: neuralnetwork::Network::new(inputs, outputs),
            parts: 
            [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            x: 0.0,
            y: 0.0,
        }
    }
}



