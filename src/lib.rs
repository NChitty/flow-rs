


pub mod bdd;

pub struct Variable {
    pub value: Option<bool>,
}

impl Default for Variable {
    fn default() -> Self {
        Self::new()
    }
}

impl Variable {
    pub fn new() -> Self {
        Self {
            value: None,
        }
    }
}

pub trait Evaluate {
    type Err;
    fn assign_vars(&mut self, values: &[bool]) -> Result<(), Self::Err>;
    fn eval(&self) -> Result<bool, Self::Err>;
    fn truth_table(&mut self) -> Result<Vec<bool>, Self::Err>;
}

pub(crate) fn convert_bits_to_bools(bits: usize, num_vars: usize) -> Vec<bool> {
    let mut bools = Vec::new();
    let mut cur_bits = bits;
    let mut tracker = num_vars;
    while tracker > 0 {
        bools.push((cur_bits & 1) == 1);
        tracker -= 1;
        cur_bits >>= 1;
    }
    bools
}
