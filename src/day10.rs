use std::{error::Error, fmt, str::FromStr};

#[allow(dead_code)]
static INPUT: &str = r#"
    addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop
"#;

pub fn run() {
    let input = include_str!("../input/day10/input");
    dbg!(first(input));
    print!("{}", second(input));
}

#[derive(Debug, Default, Clone)]
pub struct CPU {
    reg_x: isize,
    cycle: usize,
    ins_cycle: usize,
    cycle_state: CycleState,
    pc: usize,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Instruction {
    #[default]
    Noop,
    AddX(isize),
}

#[derive(Debug, Default, Clone, Copy)]
pub enum CycleState {
    Running,
    #[default]
    End,
}

impl Instruction {
    #[inline]
    pub fn cycle_count(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

/// An error returned when parsing a `bool` using [`from_str`] fails
///
/// [`from_str`]: super::FromStr::from_str
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not a instruction".fmt(f)
    }
}

impl Error for ParseInstructionError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to parse motion"
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited: Vec<&str> = s.split_ascii_whitespace().collect();
        match splited[0] {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::AddX(splited[1].parse().unwrap())),
            _ => Err(ParseInstructionError),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CRT {
    wide: usize,
    pixels: Vec<bool>,
}

impl CRT {
    pub fn new(wide: usize, high: usize) -> Self {
        Self {
            wide,
            pixels: vec![false; wide * high],
        }
    }

    pub fn draw_at(&mut self, pos: usize) {
        self.pixels[pos] = true;
    }
}

impl fmt::Display for CRT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, pixel) in self.pixels.iter().enumerate() {
            if *pixel {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            if (idx + 1) % self.wide == 0 {
                "\n".fmt(f)?;
            }
        }
        Ok(())
    }
}

impl CPU {
    fn with_instructions(instructions: Vec<Instruction>) -> Self {
        Self {
            reg_x: 1,
            cycle: 1,
            cycle_state: CycleState::End,
            ins_cycle: 1,
            pc: 0,
            instructions,
        }
    }

    #[allow(non_snake_case)]
    #[inline]
    fn X(&self) -> isize {
        self.reg_x
    }

    #[inline]
    fn instruction(&self) -> Instruction {
        self.instructions.get(self.pc).copied().unwrap_or_default()
    }

    fn run_once(&mut self) {
        if self.pc >= self.instructions.len() {
            println!("It's stoped already!");
            return;
        }
        self.cycle += 1;
        let instruction = self.instruction();
        self.cycle_state = if self.ins_cycle == instruction.cycle_count() {
            CycleState::End
        } else {
            CycleState::Running
        };
        match self.cycle_state {
            CycleState::Running => self.ins_cycle += 1,
            CycleState::End => {
                match instruction {
                    Instruction::Noop => (),
                    Instruction::AddX(n) => {
                        self.reg_x += n;
                    }
                };
                self.pc += 1;
                self.ins_cycle = 1;
            }
        }
    }

    fn run_until_cycle(&mut self, cycle: usize) {
        for _ in (self.cycle + 1)..=cycle {
            self.run_once();
        }
    }

    #[inline]
    fn signal_strength(&self) -> isize {
        self.cycle as isize * self.reg_x
    }

    fn get_all_signal_strength(
        &mut self,
        start: usize,
        step_by: usize,
        count: usize,
    ) -> Vec<isize> {
        let mut result = vec![];
        for cycle in (start..).step_by(step_by).take(count) {
            self.run_until_cycle(cycle);
            if self.cycle == cycle {
                result.push(self.signal_strength());
            } else {
                break;
            }
        }
        result
    }
}

fn to_instuctions(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|i| i.trim())
        .filter(|i| !i.is_empty())
        .map(|i| i.parse().unwrap())
        .collect()
}

fn first(input: &str) -> isize {
    let instructions = to_instuctions(input);
    let mut cpu = CPU::with_instructions(instructions);
    cpu.get_all_signal_strength(20, 40, 6).into_iter().sum()
}

fn second(input: &str) -> String {
    let instructions = to_instuctions(input);
    let mut cpu = CPU::with_instructions(instructions);
    let mut crt = CRT::new(40, 6);
    for i in 0..crt.pixels.len() {
        let pos = (i % crt.wide) as isize;
        let x = cpu.X();
        if [x - 1, x, x + 1].contains(&pos) {
            crt.draw_at(i);
        }
        cpu.run_once();
    }
    format!("{crt}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_signal_strength() {
        let instructions = to_instuctions(INPUT);
        let mut cpu = CPU::with_instructions(instructions);

        cpu.run_until_cycle(20);
        assert_eq!(cpu.signal_strength(), 420);
        cpu.run_until_cycle(60);
        assert_eq!(cpu.signal_strength(), 1140);
        cpu.run_until_cycle(100);
        assert_eq!(cpu.signal_strength(), 1800);
        cpu.run_until_cycle(140);
        assert_eq!(cpu.signal_strength(), 2940);
        cpu.run_until_cycle(180);
        assert_eq!(cpu.signal_strength(), 2880);
        cpu.run_until_cycle(220);
        assert_eq!(cpu.signal_strength(), 3960);
    }

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT), 13140);
    }

    #[test]
    fn test_second() {
        let output = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;
        assert_eq!(second(INPUT).trim(), output.trim().to_string());
    }
}
