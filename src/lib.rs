// > 	Increment the data pointer (to point to the next cell to the right).
// < 	Decrement the data pointer (to point to the next cell to the left).
// + 	Increment (increase by one) the byte at the data pointer.
// - 	Decrement (decrease by one) the byte at the data pointer.
// . 	Output the byte at the data pointer.
// , 	Accept one byte of input, storing its value in the byte at the data pointer.
// [ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ] 	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

use std::{collections::VecDeque, str::FromStr};

#[derive(Clone, Debug)]
pub struct BrainFuck(Vec<Command>);

#[derive(Clone, Debug)]
pub enum Command {
    IncData,
    DecData,
    IncPtr,
    DecPtr,
    OutputByte,
    AcceptByte,
    Loop(Vec<Command>),
}

impl BrainFuck {
    pub fn run(&self) {
        let mut machine = Machine::default();
        exec_comands(&self.0, &mut machine);
    }
}

impl FromStr for BrainFuck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let commands = parse_body(&mut it);
        Ok(Self(commands))
    }
}

fn exec_comands(commands: &[Command], memory: &mut Machine){
    for cmd in commands {
        match cmd {
            Command::IncPtr => memory.inc_ptr(),
            Command::DecPtr => memory.dec_ptr(),
            Command::IncData => memory.inc_data(),
            Command::DecData => memory.dec_data(),
            Command::OutputByte => print!("{}", memory.get_data() as char),
            Command::AcceptByte => unimplemented!(),
            Command::Loop(commands) => {
                while memory.get_data() != 0 {
                    exec_comands(commands, memory);
                }
            }
        }
    }
}

fn parse_body(it: &mut impl Iterator<Item = char>) -> Vec<Command> {
    let mut program = Vec::new();
    loop {
        let Some(c) = it.next()else {break;};
        match c {
            '>' | '<' | '+' | '-' | '.' | ',' => program.push(parse_command(c)),
            '[' => program.push(Command::Loop(parse_body(it))),
            ']' => break,
            _ => continue,
        }
    }
    program
}

fn parse_command(c: char) -> Command {
    match c {
        '>' => Command::IncPtr,
        '<' => Command::DecPtr,
        '+' => Command::IncData,
        '-' => Command::DecData,
        '.' => Command::OutputByte,
        ',' => Command::AcceptByte,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    ptr: usize,
    data: VecDeque<u8>,
}


impl Machine {
    fn inc_ptr(&mut self) {
        self.ptr += 1;
        if self.ptr == self.data.len() {
            self.data.push_back(0);
        }
    }

    fn dec_ptr(&mut self) {
        if self.ptr == 0 {
            self.data.push_front(0);
        } else {
            self.ptr -= 1;
        }
    }
    fn get_data(&self) -> u8 {
        self.data[self.ptr]
    }
    fn inc_data(&mut self)  {
        self.data[self.ptr] = u8::wrapping_add(self.data[self.ptr] , 1);
    }
    fn dec_data(&mut self) {
        self.data[self.ptr] = u8::wrapping_sub(self.data[self.ptr] , 1);

    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            ptr: 0,
            data: [0].into_iter().collect(),
        }
    }
}
