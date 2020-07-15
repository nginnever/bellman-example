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

    let rng = &mut thread_rng();

    println!("Creating parameters...");
    let load_parameters = false;
    let parameters_path = "parameters.dat";

    // Create parameters for our circuit
    let params = if load_parameters {
        let param_file = File::open(parameters_path).expect("Unable to open parameters file!");
        Parameters::<Bls12>::read(param_file, false /* false for better performance*/)
            .expect("Unable to read parameters file!")
    } else {
        // make option values as None for these variables, for paramgen
        // don't want to bake these nums into parameters
        let c = mul::MultiplyDemo::<Bls12> {
            a: None,
            b: None,
            c: None
        };

        let p = generate_random_parameters(c, rng).unwrap();
        let mut param_file = File::create(parameters_path).expect("Unable to create parameters file!");
        p.write(param_file).expect("Unable to write parameters file!");
        p
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");
    
    let public_input = Fr::from_str("21");

    // Create an instance of circuit
    let c = mul::MultiplyDemo::<Bls12> {
        a: Fr::from_str("7"),
        // when creating instance here, pass in Some of actual variables you're using
        b: Fr::from_str("3"),
        c: public_input
    };
    
    println!("Proving I know a and b such that a*b=21");
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();
    
    assert!(verify_proof(
        &pvk,
        &proof,
        &[public_input.unwrap()]
    ).unwrap());

    println!("Proof verified correctly!");
}
