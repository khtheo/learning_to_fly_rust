#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

// impl Network {
//     pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {

//         for layer in &self.layers {
//             inputs = layer.propagate(inputs);
//         }

//         inputs
//     }
// }

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs)) // for each layer, we apply the propagate function, passing passsing step by step outputs of previous layer to the next one
    } 
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

// impl Layer {
//     fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
//         let mut outputs = Vec::new();

//         for neuron in &self.neurons {
//             let output = neuron.propagate(&inputs);
//             outputs.push(output);
//         }

//         outputs
//     }
// }

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter() // we iterate over each neuron in the layer
            .map(|neuron| neuron.propagate(&inputs)) // For each neuron, we apply the propagate function
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> Result<f32> {

        if inputs.len() != self.weights.len() {
            return Err(Error::MismatchedInputSize {
                got: inputs.len(),
                expected: self.weights.len(),
            });
        }

        let mut output = 0.0;

        // for i in 0..inputs.len() { // it is how you write a for loop in rust.
        //     output += inputs[i] * self.weights[i];
        // }

        // for (&input, &weight) in inputs.iter().zip(&self.weights) { // zip is a function that associates inputs and weights together instead of using indexes.
        //     output += input * weight;
        // }

        let mut output = input
            .iter()
            .zip(&self.weights) // Here zip function associates inputs and weights together
            .map(|(input, weight)| input * weight) // map function takes input and weight in parameters and returns its product.
            .sum::<f32>(); // we end by summing all of these products to get the output

        (self.bias + output).max(0.0) // Returning the sum between bias & output + ReLU activation
    }
}




pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("got {got} inputs, but {expected} inputs were expected")]
    MismatchedInputSize {
        got: usize,
        expected: usize,
    },
}

