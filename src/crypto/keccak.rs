use anyhow::{Result, anyhow};
use bitvec::prelude as bv;
use bitvec::ptr as bitptr;

use std::collections::HashSet;

use crate::errors::KecPInitError;

//Width candidates for Keccak-p permutations
const B_CANDS: [usize; 7] = [25, 50, 100, 200, 400, 800, 1600];

//KECCAK-F permutation definition
struct KecF {
    //width
    b: usize,
    //# of rounds
    n_r: usize,
    //b/25
    w: usize,
    //log_2(b/25)
    l: usize,
    //b bit State String
    state: StateStr,
}

impl KecF {
    pub fn new(b: usize) -> Result<Self> {
        if !B_CANDS.contains(&b) {
            return Err(anyhow!(KecPInitError));
        }

        let w = b / 25;
        let l = w.ilog(2) as usize;

        //Keccak-p special family condition for Keccak-f
        let n_r = 12 + 2 * l;

        let mut state: bv::BitVec = bv::BitVec::new();
        for _ in 0..b {
            state.push(false);
        }

        let state = StateStr(state);

        Ok(Self {
            b: b,
            n_r: n_r,
            w: w,
            l: l,
            state: state,
        })
    }

    fn step_theta(&mut self) {
        let A = &mut self.state.array_view(self.w);

        //Coordinate transform for 1d array representation
        let coord = |x: usize, z: usize| -> usize { 5 * x + z };

        let modulo = |x: isize, m: usize| -> usize { x.rem_euclid(m as isize) as usize };

        //Construct C 2d array as bit vector
        let mut C: bv::BitVec = bv::BitVec::new();

        for _ in 0..5 * self.w {
            C.push(false);
        }

        for x in 0..5 {
            for z in 0..self.w {
                let c_result = (1..5).fold(A.get(x, 0, z), |acc, elem| acc ^ A.get(x, elem, z));
                *C.get_mut(coord(x, z)).unwrap() = c_result;
            }
        }

        //Construct D 2d array as bit vector
        let mut D: bv::BitVec = bv::BitVec::new();

        for _ in 0..5 * self.w {
            D.push(false);
        }

        for x in 0..5 {
            for z in 0..self.w {
                let d_result = *(&C).get(coord(modulo(x as isize - 1, 5), z)).unwrap()
                    ^ *(&C).get(coord((x + 1) % 5, modulo(z as isize - 1, self.w))).unwrap();

                *D.get_mut(coord(x, z)).unwrap() = d_result;
            }
        }

        for x in 0..5 {
            for y in 0..5 {
                for z in 0..self.w {
                    *A.get_mut(x, y, z) = A.get(x, y, z) ^ *(&D).get(coord(x, z)).unwrap();
                }
            }
        }
    }
}

struct StateStr(bv::BitVec);

impl StateStr {
    pub fn array_view(&mut self, w: usize) -> ArrayView<'_> {
        ArrayView {
            slice: &mut self.0,
            w,
        }
    }
}

struct ArrayView<'a> {
    slice: &'a mut bv::BitSlice<usize, bv::Lsb0>,
    w: usize,
}

impl<'a> ArrayView<'a> {
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> BitMut<'_> {
        let idx = self.w * (5 * y + x) + z;
        self.slice.get_mut(idx).unwrap()
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> bool {
        let idx = self.w * (5 * y + x) + z;
        *self.slice.get(idx).unwrap()
    }
}

type BitMut<'a> = bv::BitRef<'a, bitptr::Mut, usize>;
