extern crate bellperson;
extern crate paired;
extern crate rand;
extern crate ff;
use bellperson::{Circuit, ConstraintSystem, SynthesisError};
use bellperson::groth16::{Parameters};
use paired::{Engine};
use ff::{Field, PrimeField};

use std::fs::File;
use std::io::prelude::*;

mod mul;

fn main() {
	//env_logger::init();
	use paired::bls12_381::{Bls12, Fr};
	use rand::thread_rng;

    use bellperson::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    };

    println!("Hello, world!");
}
