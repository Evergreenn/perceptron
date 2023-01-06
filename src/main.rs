use rand::distributions::{IndependentSample, Range};
use rand::Rng;

use crate::functions::heaviside::*;
use crate::synapse::dot::*;

mod functions;
mod synapse;

const ETA: f64 = 0.2;
const ITER: usize = 100;

struct TeachingData {
    input: (i8, i8, i8),
    expected: i8,
}

fn main() {
    let mut rng = rand::thread_rng();

    let teaching_data = [
        TeachingData {
            input: (0, 1, 1),
            expected: 1,
        },
        TeachingData {
            input: (1, 0, 1),
            expected: 1,
        },
        TeachingData {
            input: (1, 1, 1),
            expected: 0,
        },
        TeachingData {
            input: (0, 0, 1),
            expected: 1,
        },
    ];

    //Initiate the weight vector with random data between 0 and 1
    let range = Range::new(0., 1.);
    let mut w = (
        range.ind_sample(&mut rng),
        range.ind_sample(&mut rng),
        range.ind_sample(&mut rng),
    );

    println!("Starting teaching phase with {} iterations.", ITER);

    for _ in 0..ITER {
        //Choose random teaching sample
        let &TeachingData { input: x, expected } = rng.choose(&teaching_data).unwrap();

        //DOT product
        let result = dot(x, w);

        //Calculate the error
        let error = expected - result.heaviside();

        // println!("{} {}: {} --> {} \n({}, {}, {})", x.0, x.1, result, error, w.0, w.1, w.2);

        //Update the weights
        w.0 += ETA * error as f64 * x.0 as f64;
        w.1 += ETA * error as f64 * x.1 as f64;
        w.2 += ETA * error as f64 * x.2 as f64;
    }
    println!("End of the learning phase");

    //After the learning phase the perceptron should be ready
    for &TeachingData { input, .. } in &teaching_data {
        let result = dot(input, w);
        println!(
            "({} :: {}): {:.*} -> {}",
            input.0,
            input.1,
            8,
            result,
            result.heaviside()
        );
    }

    let input = (1, 0, 1);
    let result = dot(input, w);
    println!();

    println!(
        "({} :: {}): {:.*} -> {}",
        input.0,
        input.1,
        8,
        result,
        result.heaviside()
    );
}
