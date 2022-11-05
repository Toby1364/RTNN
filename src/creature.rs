use crate::neuralnetwork;
use rand::Rng;

#[derive(Debug, Clone)]
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
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            x: rng.gen_range(200..300) as f64,
            y: rng.gen_range(200..300) as f64,
            energy: 9.0,
        }
    }
    pub fn mutate(&mut self, num_of_mutations: usize) {
        self.network.mutate(num_of_mutations);
        let mut rng = rand::thread_rng();

        let mut i = 0;
        while i < num_of_mutations {
            if 5 > rng.gen_range(0..100) as u8 {
                self.body[rng.gen_range(0..8)][rng.gen_range(0..8)] = rng.gen_range(0..6);
            }
            i += 1;
        }
    }
}



