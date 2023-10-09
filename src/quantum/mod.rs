use std::vec::Vec;
use std::collections::HashSet;

use crate::Complex;

#[derive(Debug)]
pub struct State
{
    qubits: u64,
    mapping: Vec<(u64, Complex)>
}

impl State
{
    pub fn new(qubits: u64, init_state: u64) -> State
    {
        // create state
        let mut s = State{qubits: qubits, mapping: Vec::new()};
        // set initial state
        s.mapping.push((init_state, Complex::new(1.0, 0.0)));
        // return
        return s;
    }

    pub fn pretty_amplitudes(&self)
    {
        for i in 0..self.mapping.len()
        {
            let tup = &self.mapping[i];
            println!("|{}> {:?}", tup.0, tup.1);
        }
    }

    pub fn pretty_probabilities(&self)
    {
        for i in 0..self.mapping.len()
        {
            let tup = self.mapping[i].clone();
            println!("|{}> {:?}", tup.0, tup.1.abs_squared());
        }
    }

    pub fn state_index(&mut self, state: u64) -> usize
    {
        for i in 0..self.mapping.len()
        {
            if self.mapping[i].0 == state {
                return i;
            }
        }

        // insert searched entry to be able to return its index
        self.mapping.push((state, Complex::zero()));

        return self.mapping.len() - 1;
    }

    pub fn contains_state(&self, state: u64) -> bool
    {
        for i in 0..self.mapping.len()
        {
            if self.mapping[i].0 == state {
                return true;
            }
        }
        return false;
    }

    pub fn cnot(&mut self, control: u64, target: u64)
    {
        for i in 0..self.mapping.len()
        {
            let (mut ste, ref amp) = self.mapping[i].clone();

            if (ste & (1 << control)) > 0
            {
                ste = ste ^ (1 << target);
            }

            self.mapping[i] = (ste, amp.clone());
        }
    }

    pub fn toffoli(&mut self, control0: u64, control1: u64, target: u64)
    {
        for i in 0..self.mapping.len()
        {
            let (mut ste, ref amp) = self.mapping[i].clone();

            if (ste & (1 << control0)) > 0
            {
                if (ste & (1 << control1)) > 0
                {
                    ste = ste ^ (1 << target);       
                }
            }

            self.mapping[i] = (ste, amp.clone());
        }
    }

    // Swap bits in state
    pub fn swap(&mut self, fst: i64, snd: i64)
    {
        for i in 0..self.mapping.len()
        {
            let tup = self.mapping[i].clone();
            let mut state = tup.0;

            let fb = (state & (1 << fst)) > 0 as u64;
            let sb = (state & (1 << snd)) > 0 as u64;

            if fb != sb
            {
                state = state ^ (1 << fst);
                state = state ^ (1 << snd);

                self.mapping[i] = (state, tup.1);
            }
        }
    }

    pub fn gate_x(&mut self, target: u64)
    {
        for i in 0..self.mapping.len()
        {
            let (mut ste, ref amp) = self.mapping[i].clone();

            ste = ste ^ (1 << target);

            self.mapping[i] = (ste, amp.clone());
        }
    }

    pub fn gate_y(&mut self, target: u64)
    {
        for i in 0..self.mapping.len()
        {
            let (mut ste, ref amp) = self.mapping[i].clone();

            ste = ste ^ (1 << target);

            let new_amp: Complex;
            if ste & (1 << target) > 0
            {
                new_amp = amp.clone().mul(Complex::new(0.0, 1.0));
            }
            else
            {
                new_amp = amp.clone().mul(Complex::new(0.0, -1.0));
            }

            self.mapping[i] = (ste, new_amp);
        }
    }

    pub fn gate_z(&mut self, target: u64)
    {
        for i in 0..self.mapping.len()
        {
            let (mut ste, ref amp) = self.mapping[i].clone();

            ste = ste ^ (1 << target);

            let mut new_amp: Complex = amp.clone();
            if ste & (1 << target) > 0
            {
                new_amp = amp.clone().mul(Complex::new(-1.0, 0.0));
            }

            self.mapping[i] = (ste, new_amp);
        }
    }

    pub fn conditional_phase_shift(&mut self, control: i64, target: i64)
    {
        let phi = std::f64::consts::PI / ((1 << (control - target).abs()) as f64);
        let p = Complex::new(0.0, phi).exp();

        for i in 0..self.mapping.len()
        {
            let (ste, ref amp) = self.mapping[i].clone();

            let mut amp_new: Complex = amp.clone();
            if (ste & (1 << control)) > 0
            {
                if (ste & (1 << target)) > 0
                {
                    amp_new = amp_new.mul(p.clone());
                    self.mapping[i] = (ste, amp_new);
                }
            }
        }
    }

    pub fn conditional_phase_shift_inverse(&mut self, control: i64, target: i64)
    {
        let phi = (-std::f64::consts::PI) / ((1 << (control - target).abs()) as f64);
        let p = Complex::new(0.0, phi).exp();

        for i in 0..self.mapping.len()
        {
            let (ste, ref amp) = self.mapping[i].clone();

            let mut amp_new: Complex = amp.clone();
            if (ste & (1 << control)) > 0
            {
                if (ste & (1 << target)) > 0
                {
                    amp_new = amp_new.mul(p.clone());
                    self.mapping[i] = (ste, amp_new);
                }
            }
        }
    }

    // Apply 2x2 matrix on qubit
    pub fn gate(&mut self, target: u64, mat: [Complex; 4])
    {
        for i in 0..(1 << self.qubits)
        {
            let normal_index = self.state_index(i as u64);
            let normal_state = self.mapping[normal_index].0;
            let normal_amp   = self.mapping[normal_index].1.clone();

            let comple_state = normal_state ^ (1 << target);
            let comple_index = self.state_index(comple_state);
            let comple_amp   = self.mapping[comple_index].1.clone();

            if (normal_state & (1 << target)) == 0
            {
                let update_zero = mat[0].mul(normal_amp.clone()).add(mat[1].mul(comple_amp.clone()));
                let update_one = mat[2].mul(normal_amp.clone()).add(mat[3].mul(comple_amp.clone()));

                self.mapping[normal_index] = ((normal_state as u64), update_zero);
                self.mapping[comple_index] = ((comple_state as u64), update_one);   
            }
        }
    }

    pub fn hadamard(&mut self, target: u64)
    {
        let matrix = [Complex::new(0.70710678118, 0.0), Complex::new(0.70710678118, 0.0), Complex::new(0.70710678118, 0.0), Complex::new(-0.70710678118, 0.0)];

        self.gate(target, matrix);
    }

    pub fn qft(&mut self, span: i64, offset: i64)
    {
        for i in (0..span).rev()
        {
            self.hadamard((offset + i) as u64);

            for j in (0..i).rev()
            {
                self.conditional_phase_shift(offset + j, offset + i);
            }
        }

        // Swap bits
        let mut i = 0;
        let mut j = span - 1;
        loop
        {
            if (j - i) >= 1 {
                self.swap(offset + i, offset + span - 1 - i);
            } else {
                break;
            }

            i += 1;
            j -= 1;
        }
    }

    pub fn iqft(&mut self, span: i64, offset: i64)
    {
        for i in (0..span).rev()
        {
            self.hadamard((offset + i) as u64);

            for j in (0..i).rev()
            {
                self.conditional_phase_shift_inverse(offset + j, offset + i);
            }
        }

        // Swap bits
        let mut i = 0;
        let mut j = span - 1;
        loop
        {
            if (j - i) >= 1 {
                self.swap(offset + i, offset + span - 1 - i);
            } else {
                break;
            }

            i += 1;
            j -= 1;
        }
    }

    pub fn pow_x_mod_n(&mut self, x: u64, n: u64)
    {
    }

    pub fn measure(&self)
    {
    }

    pub fn measure_and_project(&mut self, qubit: u64)
    {
    }
}