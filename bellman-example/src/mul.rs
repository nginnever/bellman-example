#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellperson;
extern crate paired;
extern crate rand;
extern crate ff;

// For randomness (during paramgen and proof generation)
use self::rand::{thread_rng};

// Bring in some tools for using pairing-friendly curves
use self::paired::{
    Engine
};

use self::ff::{Field,PrimeField};

// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use self::paired::bls12_381::{
    Bls12,
    Fr,
};

// We'll use these interfaces to construct our circuit.
use self::bellperson::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};

// We're going to use the Groth16 proving system.
use self::bellperson::groth16::{
    Proof,
    generate_random_parameters,
    prepare_verifying_key,
    create_random_proof,
    verify_proof,
};

// demo circuit
// proving that I know a such that a * 3 = 21
pub struct MultiplyDemo<E: Engine> {
    pub a: Option<E::Fr>,
    pub b: Option<E::Fr>,
    pub c: Option<E::Fr>
}

// create a demo circuit by using the `Circuit` trait which
/// is used during paramgen and proving in order to
/// synthesize the constraint system.
impl <E: Engine> Circuit<E> for MultiplyDemo<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self, 
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        
        // Allocate the first value (private)
        let a = cs.alloc(|| "a", || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate the second value (private)
        let b = cs.alloc(|| "b", || {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate the third value (public)
        // allocating a public input uses alloc_input
        let c = cs.alloc_input(|| "c", || {
            self.c.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // a * b = c?
        cs.enforce(
            || "mul",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c
        );
        
        Ok(())
    }
}

