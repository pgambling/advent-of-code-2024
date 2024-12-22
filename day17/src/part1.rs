const ADV: isize = 0;
const BXL: isize = 1;
const BST: isize = 2;
const JNZ: isize = 3;
const BXC: isize = 4;
const OUT: isize = 5;
const BDV: isize = 6;
const CDV: isize = 7;

struct Computer {
    register_a: isize,
    register_b: isize,
    register_c: isize,
    program: Vec<isize>,
    pointer: usize,
    output: Vec<isize>,
}

impl Computer {
    fn new() -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            program: vec![],
            pointer: 0,
            output: vec![],
        }
    }

    fn load_state_input(&mut self, input: &[String]) {
        let (mut register_a, mut register_b, mut register_c, mut program) = (0, 0, 0, vec![]);
        input.iter().for_each(|line| {
            if let Some((destination, value)) = line.split_once(": ") {
                if destination == "Program" {
                    program = value
                        .split(',')
                        .filter_map(|n| n.parse::<isize>().ok())
                        .collect();
                } else if let Ok(num) = value.parse::<isize>() {
                    match destination {
                        "Register A" => register_a = num,
                        "Register B" => register_b = num,
                        "Register C" => register_c = num,
                        _ => unreachable!(),
                    }
                }
            }
        });
        self.register_a = register_a;
        self.register_b = register_b;
        self.register_c = register_c;
        self.program = program;
        self.pointer = 0;
    }

    fn literal_operand(&mut self) -> isize {
        let operand = self.program[self.pointer];
        self.pointer += 1;
        operand
    }

    fn combo_operand(&mut self) -> isize {
        let operand = self.program[self.pointer];
        self.pointer += 1;
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }

    fn output(&mut self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn dv_instruction(&mut self) -> isize {
        self.register_a / 2_isize.pow(self.combo_operand() as u32)
    }

    fn combo_operand_modulo8(&mut self) -> isize {
        self.combo_operand().rem_euclid(8)
    }

    fn run(&mut self) -> String {
        while self.pointer < self.program.len() {
            let instruction = self.program[self.pointer];
            self.pointer += 1;
            match instruction {
                ADV => {
                    self.register_a = self.dv_instruction();
                }
                BXL => {
                    self.register_b ^= self.literal_operand();
                }
                BST => {
                    self.register_b = self.combo_operand_modulo8();
                }
                JNZ => {
                    let operand = self.literal_operand();
                    if self.register_a == 0 {
                        continue;
                    }
                    self.pointer = operand as usize;
                }
                BXC => {
                    self.register_b ^= self.register_c;
                    self.literal_operand(); // Read operand for legacy reasons
                }
                OUT => {
                    let output = self.combo_operand_modulo8();
                    self.output.push(output);
                }
                BDV => {
                    self.register_b = self.dv_instruction();
                }
                CDV => {
                    self.register_c = self.dv_instruction();
                }
                _ => unreachable!(),
            }
        }
        self.output()
    }
}

pub fn solve(input: &[String]) -> String {
    let mut computer = Computer::new();
    computer.load_state_input(input);
    computer.run()
}
