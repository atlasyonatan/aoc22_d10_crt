pub struct Instruction<V> {
    value: V,
    cycles: usize,
}

impl Instruction<isize> {
    pub fn noop() -> Self {
        Self {
            value: 0,
            cycles: 1,
        }
    }

    pub fn addx(value: isize) -> Self {
        Self { value, cycles: 2 }
    }
}

pub struct Interpreter<I: Iterator, V> {
    iter: I,
    reg_x: V,
    instruction: Option<Instruction<V>>,
}

impl<I> Iterator for Interpreter<I, isize>
where
    I: Iterator<Item = String>,
{
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        while let None = &self.instruction {
            match self.iter.next() {
                Some(line) => {
                    let mut words = line.split_ascii_whitespace();
                    match words.next() {
                        Some("addx") => match words.next() {
                            Some(s) => match s.parse::<isize>() {
                                Ok(value) => self.instruction = Some(Instruction::addx(value)),
                                Err(error) => {
                                    panic!("Failed to parse '{}' into isize.\nError: {}", &s, error)
                                }
                            },
                            None => panic!("addx instruction missing 1 positional parameter"),
                        },
                        Some("noop") => self.instruction = Some(Instruction::noop()),
                        Some(other) => panic!("Unknown instruction '{}'", other),
                        None => panic!("Invalid empty line"),
                    }
                }
                None => return None,
            }
        }
        let instruction = self.instruction.as_mut().unwrap();
        let current = self.reg_x;
        if instruction.cycles <= 1 {
            self.reg_x += instruction.value;
            self.instruction = None;
        } else {
            instruction.cycles -= 1;
        }
        Some(current)
    }
}

pub trait InterpretIter: Iterator + Sized {
    fn interpret(self) -> Interpreter<Self, isize>;
}

impl<I> InterpretIter for I
where
    I: Iterator<Item = String>,
{
    fn interpret(self) -> Interpreter<Self, isize> {
        Interpreter {
            instruction: None,
            iter: self,
            reg_x: 1,
        }
    }
}
