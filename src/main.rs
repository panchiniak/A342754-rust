extern crate num;

use num_bigint::{BigUint};
use num_traits::FromPrimitive;
use crate::num::ToPrimitive;

fn main() {
    println!("Sequence A342754!");

    const MAXSIZE: u64 = 500;
    let mut n: BigUint = BigUint::from_u64(0).unwrap();
    let mut a: [usize; 2500] = [0; 2500];

    for m in 1..MAXSIZE {
        let mut k = BigUint::from_u64(2).unwrap();
        let m_big_int = BigUint::from_u64(m).unwrap();

        while k<=m_big_int {
            let modulo: BigUint = &m_big_int % &k;
            if modulo == BigUint::from_u64(0).unwrap() {
                let mut xmod: BigUint = BigUint::from_u64(1).unwrap();
                let mut x: usize = 1;
                while xmod != BigUint::from_u64(0).unwrap() {
                    let pow: BigUint = BigUint::pow(&k, x.try_into().unwrap());
                    let diff_first: BigUint = pow - BigUint::from_u64(1).unwrap();
                    let diff_second: BigUint = m_big_int.clone() - BigUint::from_u64(1).unwrap();
                    xmod = &diff_first % &diff_second;
                    if xmod == BigUint::from_u64(0).unwrap() && x != 1 {
                        // println!("row: {m_value} - column: {k_value}", m_value=m, k_value=k);
                        // println!("{}", x);
                        let n_usize: usize = n.to_usize().unwrap();
                        a[n_usize] = x;
                        // println!("a[{number}]={value}", number=n_usize, value=a[n_usize]);
                        n += BigUint::from_u64(1).unwrap();
                    }
                    x += 1;
                }
            }
            k += BigUint::from_u64(1).unwrap();
        }
    }

    println!("{:?}", a);
}
