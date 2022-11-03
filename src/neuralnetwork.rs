use std::fmt;
use rand::Rng;

#[derive(Debug)]
pub struct Neuron {
    value: f64,
    function: String,
    conections: Vec<usize>,
    multiplayers: Vec<f64>,
    mutable: bool,
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

#[derive(Debug)]
pub struct Network {
    neurons: Vec<Neuron>,
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
            neurons.push(Neuron::new());
            i += 1;
        }

        return Network{neurons};
    }
    pub fn mutate(&mut self) {
        
    }
}
