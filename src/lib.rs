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
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: None,
        }
    }
}

pub trait Evaluate {
    type Err;

    /// Assign variables the variables the given values.
    /// # Arguments
    /// * `values` - a slice of booleans to assign to the given struct in order
    /// # Errors
    /// * `VariableAssignmentError` - when the number does not match or a match can't be found
    /// # Example
    /// The following assigns the variable `0` to true for the BDD.
    /// ```
    /// use flow::bdd::BinaryDecisionDiagram;
    /// use flow::Evaluate;
    ///
    /// const SIMPLE_BDD: &str = "vars 1
    ///  nodes 3
    ///  0 1 2 0
    ///  1 -1 -1 1
    ///  2 -1 -1 0";
    ///
    /// let mut some_evaluate: BinaryDecisionDiagram = SIMPLE_BDD.parse().unwrap();
    ///
    /// some_evaluate.assign_vars(&vec![true])?;
    /// ```
    fn assign_vars(&mut self, values: &[bool]) -> Result<(), Self::Err>;

    /// Evaluate the current struct using currently assigned variables.
    /// # Errors
    /// * `VariableAssignmentError` - tried to valuate on a variable that has not been assigned
    /// * `EvaluationError`
    /// # Example
    /// The following assigns the variable `0` to true for the BDD and gets the result via `eval`.
    /// ```
    /// use flow::bdd::BinaryDecisionDiagram;
    /// use flow::Evaluate;
    ///
    /// const SIMPLE_BDD: &str = "vars 1
    ///  nodes 3
    ///  0 1 2 0
    ///  1 -1 -1 1
    ///  2 -1 -1 0";
    ///
    /// let mut some_evaluate: BinaryDecisionDiagram = SIMPLE_BDD.parse().unwrap();
    ///
    /// some_evaluate.assign_vars(&vec![true])?;
    /// some_evaluate.eval()?;
    /// ```
    fn eval(&self) -> Result<bool, Self::Err>;

    /// Get a list of booleans in order based on values of the variables
    /// # Errors
    /// * `VariableAssignmentError` - when the number does not match or a match can't be found
    /// * `EvaluationError`
    /// # Example
    /// The resulting vector of booleans is indexed 0 for variable 0 false and 1 for variable 0
    /// true.
    /// ```
    /// use flow::bdd::BinaryDecisionDiagram;
    /// use flow::Evaluate;
    ///
    /// const SIMPLE_BDD: &str = "vars 1
    ///  nodes 3
    ///  0 1 2 0
    ///  1 -1 -1 1
    ///  2 -1 -1 0";
    ///
    /// let mut some_evaluate: BinaryDecisionDiagram = SIMPLE_BDD.parse().unwrap();
    ///
    /// some_evaluate.truth_table()?;
    /// ```
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
