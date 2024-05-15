extern crate num;

use num_bigint::{BigUint};
use num_traits::FromPrimitive;

fn main() {
    println!("Sequence A342754!");

    const MAXSIZE: u64 = 1001;
    let mut n: BigUint = BigUint::from_u64(0).unwrap();

    for m in 1..MAXSIZE {
        let mut k = BigUint::from_u64(2).unwrap();
        let m_big_int = BigUint::from_u64(m).unwrap();

        while k<=m_big_int {
            let modulo: BigUint = &m_big_int % &k;
            if modulo == BigUint::from_u64(0).unwrap() {
                let mut xmod: BigUint = BigUint::from_u64(1).unwrap();
                let mut x: u32 = 1;
                while xmod != BigUint::from_u64(0).unwrap() {
                    let pow: BigUint = BigUint::pow(&k, x);
                    let diff_first: BigUint = pow - BigUint::from_u64(1).unwrap();
                    let diff_second: BigUint = m_big_int.clone() - BigUint::from_u64(1).unwrap();
                    xmod = &diff_first % &diff_second;
                    if xmod == BigUint::from_u64(0).unwrap() && x != 1 {
                        // println!("row: {m_value} - column: {k_value}", m_value=m, k_value=k);
                        println!("{}", x);
                        n += BigUint::from_u64(1).unwrap();
                    }
                    x += 1;
                }
            }
            k += BigUint::from_u64(1).unwrap();
        }
    }
}
