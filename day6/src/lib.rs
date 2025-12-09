mod part1;
mod part2_by_column;
mod part2_transpose;

pub use part1::*;
pub use part2_by_column::*;
pub use part2_transpose::*;

#[derive(Debug, Clone)]
enum Op {
    None,
    Add,
    Mult,
}

#[derive(Debug)]
struct Calculation {
    pub operands: Vec<usize>,
    pub operation: Op,
}

impl Calculation {
    pub fn total(&self) -> usize {
        match self.operation {
            Op::None => unreachable!("invalid None operation"),
            Op::Add => self.operands.iter().sum::<usize>(),
            Op::Mult => self.operands.iter().product::<usize>(),
        }
    }
}
