use nalgebra::DMatrix;
use ark_bls12_381::Fq;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
// use ark_poly::MVPolynomial;
use ark_ff::fields::Field;
use crate::{core::UntypedValue, engine::Target};
use replace_with::replace_with_or_abort;
// use ark_ff::{Zero, One};


type GlobalStepCount = u64;
type VariableIndex = usize;
pub type VariableValue = Fq;
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct R1CS<T : Field>{
  pub variable_count : usize,
  pub global_step_count : u64,
  pub A : DMatrix<T>,
  pub B : DMatrix<T>,
  pub C:  DMatrix<T>,
  pub pc : TraceVariable<T>,
  pub variables : Vec<TraceVariable<T>>,
  pub inputs : Vec<TraceVariable<T>>,
  // pub trace : BTreeMap<VariableIndex, TraceVariable>,
  pub constraints : Vec<SparsePolynomial<VariableValue,SparseTerm>>
}

#[derive(Debug)]
pub enum TraceVariableKind
{
  PC,
  Other
}

#[derive(Debug)]
pub struct TraceVariable<T: Field>{
  pub kind : TraceVariableKind,
  pub global_step_count : GlobalStepCount,
  pub index : VariableIndex,
  pub value : T,
}

impl<T: Field> TraceVariable<T> {

  pub fn new(kind : TraceVariableKind, global_step_count : GlobalStepCount, index : Option<VariableIndex>, value : T) -> Self{
    let some_index : usize = match &kind {
      TraceVariableKind::PC => R1CS::<T>::INITIAL_VARIABLE_COUNT-1,
      TraceVariableKind::Other => index.unwrap()
    };
    TraceVariable { kind : kind, global_step_count: global_step_count, index: some_index, value: value }
  }
}

pub trait WebAssemblyR1CS<T : Field>{

  fn on_const(&mut self, bytes: UntypedValue);
  fn on_br(&mut self, target: Target);
  fn to_field(value : usize) -> T;
}

#[allow(non_snake_case)]
impl<T : Field> R1CS<T>
{

  pub const INITIAL_VARIABLE_COUNT : usize = 2;

  pub fn new() -> Self{
    // let mut trace: BTreeMap<VariableIndex, TraceVariable> = BTreeMap::new();
    // insert global variable counter
    // trace.insert(0, TraceVariable{global_step_count : 0, index:0, value: VariableValue::default()});
    // Initialize the proram counter
    // [1, pc,]
    let A : DMatrix<T> = DMatrix::from_vec(1, Self::INITIAL_VARIABLE_COUNT, vec![T::zero(), T::one()]);
    let B : DMatrix<T> = DMatrix::from_vec(1, Self::INITIAL_VARIABLE_COUNT, vec![T::one(), T::zero()]);
    let C : DMatrix<T> = DMatrix::from_vec(1, Self::INITIAL_VARIABLE_COUNT, vec![T::zero(), T::zero()]);

    //


    R1CS::<T>{
      variable_count: Self::INITIAL_VARIABLE_COUNT,
      global_step_count: 0,
      A : A,
      B : B,
      C : C,
      pc : TraceVariable { kind: TraceVariableKind::PC, global_step_count: 0, index: 1, value: T::zero()},
      variables : vec![],
      inputs : vec![],
      // trace : trace,
      constraints: vec![]
    }
  }

  // pub fn process_instruction(&mut self, instr : &Instruction, exec_ctx : &mut ExecutionContext<&mut impl AsContextMut>){
  //   use Instruction as Instr;

  //   match instr{

  //     Instr::Br(target) => {

  //     }
  //     Instr::BrIfEqz(target) => {

  //     }
  //     Instr::BrIfNez(target)=> {

  //     }

  //     _ => {}
  //   }
  // }

  // pub fn compile(&mut self){
  //   // Compile to constraints to matrix form
  //   for constraint in &self.constraints{
  //     for term in constraint.terms(){

  //     }
  //   }
  // }
}

impl<T> WebAssemblyR1CS<T> for R1CS<T> where T: Field{
  fn on_const(&mut self, bytes: UntypedValue){
    let variable: TraceVariable<T> = TraceVariable {
      kind: TraceVariableKind::Other, 
      global_step_count: 0, // TODO 
      index: self.variable_count, 
      value: Self::to_field(bytes.to_bits() as usize)
    };

    self.variables.push(variable);



    // Increment variable count
    self.variable_count += self.variable_count;
  }

  fn on_br(&mut self, target : Target){
    // Arithmetization for zkSNARKs
    // let variable =self.r1cs.trace.get_mut(&0).unwrap();
    // TODO: Set variable value
    let pc_variable : TraceVariable<T> = TraceVariable::new(
        TraceVariableKind::PC,
        self.global_step_count, 
        None, 
        Self::to_field(target.destination_pc().into_usize()) // TODO
    );
    let shape = self.A.shape();
    self.global_step_count += 1;

    // Modify A
    replace_with_or_abort(&mut self.A, |self_|{
        self_.insert_row(shape.0, T::zero())
    });
    self.A[(shape.0, pc_variable.index)] = T::one();

    // Modify B
    replace_with_or_abort(&mut self.B, |self_|{
        self_.insert_row(shape.0, T::zero())
    });
    self.B[(shape.0, 0)] = T::one();

    // Modify C
    replace_with_or_abort(&mut self.C, |self_|{
        self_.insert_row(shape.0, T::zero())
    });
    self.C[(shape.0, 0)] = Self::to_field(target.destination_pc().into_usize());    

    println!("r1cs shape: {:?}", &self.A.shape());
    // println!("r1cs: {:?}", &self.r1cs.A);
    // let A = self.r1cs.A.insert_row(num_row.0, VariableValue::zero());
    
    // let term = SparseTerm::new(vec![(0, 1)]);
    // // TODO: get the correct coeffecient
    // let polynomial : SparsePolynomial<VariableValue, SparseTerm> = SparsePolynomial{num_vars:1, terms:vec![(VariableValue::default(), term)]};
    // self.r1cs.constraints.push(polynomial);
  }

  fn to_field(value : usize) -> T {
    // To be implemented
    T::zero()
  }

}