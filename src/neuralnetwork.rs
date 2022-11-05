//use std::fmt;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub value: f64,
    pub function: String,
    pub conections: Vec<usize>,
    pub multiplayers: Vec<f64>,
    pub mutable: bool,
}

/*impl fmt::Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut first_conect: String = String::from("0");
        if &self.conections.len() > &1 {
            if &self.conections[0].to_owned() < &10 {
                first_conect.push_str(&self.conections[0].to_string());
            }

            else {
                first_conect = self.conections[0].to_string();
            }
        }
        else {
            first_conect.push_str("0");
        }

        let mut last_conect: String = String::from("0");
        if &self.conections.len() > &0 {
            if &self.conections.last().unwrap().to_owned() < &10 {
                last_conect.push_str(&self.conections.last().unwrap().to_string());
            }

            else {
                last_conect = self.conections.last().unwrap().to_string();
            }
        }
        else {
            last_conect.push_str("0");
        }

        let x = "";

        f.debug_struct("Neuron")
         .field(
            &format!(

"════════{}════════
 [{}]━━┓
  .. ━━╋━━[{}]━━ ..
 [{}]━━┛
"
,
&self.function, 
first_conect, 
&self.value, 
last_conect,
            
), 
            &x)
         .finish()
    }
}*/

impl Neuron {
    pub fn new() -> Neuron {
        let functions: Vec<String> = vec![
        String::from("ADD"),
        String::from("SUB"),
        String::from("NADD"),
        String::from("NSUB"),
        String::from("ABSLT"),
        String::from("SIN"),
        String::from("COS"),
        String::from("TAN"),
        ];

        let mut rng = rand::thread_rng();

        Neuron {
            value: 0.0,
            function: functions[rng.gen_range(0..functions.len())].clone(),
            conections: vec![],
            multiplayers: vec![],
            mutable: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Network {
    pub neurons: Vec<Neuron>,
}

impl Network {
    pub fn new(inputs: usize, outputs: usize) -> Network {
        let mut neurons: Vec<Neuron> = Vec::new();
        let mut i = 0;

        while i < inputs {
            let mut neuron = Neuron::new();
            neuron.mutable = false;
            neurons.push(neuron);
            i += 1;
        }
        i = 0;
        while i < outputs {
            let mut neuron = Neuron::new();
            neuron.function = "ADD".to_owned();
            neurons.push(neuron);
            i += 1;
        }

        return Network{neurons};
    }
    pub fn mutate(&mut self, num_of_mutations: usize) {
        let mut iter = 0;
        while iter < num_of_mutations {
            let mut rng = rand::thread_rng();

            if 15 > rng.gen_range(0..100) as u8 {
                self.neurons.push(Neuron::new());
            }

            let n_count = self.neurons.len();

            let mut i = 0;
            while i < n_count {
                if self.neurons[i].mutable {
                    if 5 > rng.gen_range(0..100) as u8 {
                        let mut index = rng.gen_range(0..n_count);
                        while index == i {
                            index = rng.gen_range(0..n_count);
                        }
                        self.neurons[i].conections.push(index);
                        self.neurons[i].multiplayers.push(1.0);
                    }
                    if 5 > rng.gen_range(0..100) as u8 {
                        let mutiplayer = rng.gen_range(-0.5..0.5);
                        let len = self.neurons[i].multiplayers.len();
                        if len > 0 {
                            self.neurons[i].multiplayers[rng.gen_range(0..len)] += mutiplayer;
                        }
                    }
                    if 5 > rng.gen_range(0..100) as u8 {
                        let len = self.neurons[i].conections.len();
                        if len > 0 {
                            let index = rng.gen_range(0..len);
                            self.neurons[i].conections.remove(index);
                            self.neurons[i].multiplayers.remove(index);
                        }
                    }
                }
                i += 1;
            }
            iter += 1
        }
    }
    pub fn update(&mut self) {
        let fake_n = self.neurons.clone();

        for neuron in &mut self.neurons {
            match neuron.function.as_str() {
                "ADD" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += fake_n[neuron.conections[i]].value * neuron.multiplayers[i];
                        i += 1;
                    }
                }
                "NADD" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += fake_n[neuron.conections[i]].value * neuron.multiplayers[i];
                        i += 1;
                    }
                    neuron.value *= -1.0;
                }
                "SUB" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += fake_n[neuron.conections[i]].value * neuron.multiplayers[i];
                        i += 1;
                    }
                }
                "NSUB" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += fake_n[neuron.conections[i]].value * neuron.multiplayers[i];
                        i += 1;
                    }
                    neuron.value *= -1.0;
                }
                "ABSLT" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += fake_n[neuron.conections[i]].value * neuron.multiplayers[i];
                        i += 1;
                    }
                    if neuron.value < 0.0 {
                        neuron.value *= -1.0;
                    }
                }
                "SIN" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += (fake_n[neuron.conections[i]].value * neuron.multiplayers[i]).sin();
                        i += 1;
                    }
                }
                "COS" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += (fake_n[neuron.conections[i]].value * neuron.multiplayers[i]).cos();
                        i += 1;
                    }
                }
                "TAN" => {
                    neuron.value = 0.0;
                    let mut i = 0;
                    while i < neuron.conections.len() {
                        neuron.value += (fake_n[neuron.conections[i]].value * neuron.multiplayers[i]).tan();
                        i += 1;
                    }
                }

                _ => {}
            }
        }
    }
}
