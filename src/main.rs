extern crate num;

use num_bigint::{BigUint};
use num_traits::FromPrimitive;

fn main() {
    println!("Sequence A342754!");

    const MAXSIZE: u64 = 13;
    let mut n: BigUint = BigUint::from_u64(0).unwrap();
    let mut k: BigUint;

    for m in 1..MAXSIZE {
        k = BigUint::from_u64(2).unwrap();
        // Convert m to bigInt
        let m_big_int = BigUint::from_u64(m).unwrap();

        while k<=m_big_int {
            k += BigUint::from_u64(1).unwrap();
            // println!("row: {m_value} - column: {k_value}", m_value=m, k_value=k);
            let modulo = &m % &k;
            // println!("modulo: {modulo_value}", modulo_value=modulo);
            if modulo == BigUint::from_u64(0).unwrap() {
                // println!("here");
                let mut xmod: BigUint = BigUint::from_u64(1).unwrap();
                let mut x: u32 = 1;
                while xmod != BigUint::from_u64(0).unwrap() {
                    let pow: BigUint = BigUint::pow(&k, x);
                    // println!("pow: {pow_value}", pow_value=pow);
                    let diff_first: BigUint = pow - BigUint::from_u64(1).unwrap();
                    // println!("diff_first: {diff_first_value}", diff_first_value=diff_first);
                    let diff_second: BigUint = m_big_int.clone() - BigUint::from_u64(1).unwrap();
                    // println!("diff_second: {diff_second_value}", diff_second_value=diff_second);
                    xmod = &diff_first % &diff_second;
                    // println!("xmod: {xmod_value}", xmod_value=xmod);
                    if xmod == BigUint::from_u64(0).unwrap() && x != 1 {
                        println!("{x_value}", x_value=x);
                        n += BigUint::from_u64(1).unwrap();
                    }
                    x += 1;
                }
            }
            k += BigUint::from_u64(1).unwrap();
        }
    }
}
