use anyhow::{Result, anyhow};

use crate::errors::KecPInitError;

//Width candidates for Keccak-p permutations
const B_CANDS: [usize; 7] = [25, 50, 100, 200, 400, 800, 1600];

//KECCAK-P permutation definition
struct KecP {
    //width
    b: usize,
    //# of rounds
    n_r: usize,
    //b/25
    w: usize,
    //log_2(b/25)
    l: usize,
}

impl KecP {
    pub fn new(b: usize, n_r: usize) -> Result<Self> {
        if !B_CANDS.contains(&b) {
            return Err(anyhow!(KecPInitError));
        }

        let w = b / 25;
        let l = w.ilog(2) as usize;
        Ok(Self {
            b: b,
            n_r: n_r,
            w: w,
            l: l,
        })
    }
}
