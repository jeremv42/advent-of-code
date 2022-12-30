const _INPUT_EXAMPLE: &str = "addx 15
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
noop";
const _INPUT_PART1: &str = "addx 2
addx 3
addx -2
addx 3
noop
addx 6
addx -1
addx 4
addx 1
noop
addx 3
addx 1
addx 7
noop
noop
addx -1
addx 3
addx 2
noop
addx 4
addx 2
addx -25
addx -7
addx -4
addx 2
addx 2
addx 19
addx -8
addx -5
addx 2
addx -9
addx 16
addx 3
addx -2
addx 12
addx -5
addx 2
addx -15
noop
noop
noop
addx 5
addx 16
addx -22
addx -14
addx 5
noop
addx 29
noop
noop
noop
addx -21
addx 2
noop
noop
addx 5
addx -1
addx 1
noop
noop
addx 8
addx -2
addx 4
noop
addx -22
addx 29
noop
addx -36
noop
addx -2
addx 6
addx -2
addx 2
noop
noop
noop
addx 8
addx 2
addx 10
noop
addx -5
addx 3
addx -2
addx 9
addx -2
addx 2
addx -21
addx 10
addx 17
addx -38
noop
noop
noop
addx 34
addx -27
addx 2
addx -6
addx 7
addx 5
addx 2
addx 5
noop
noop
noop
addx 3
addx -2
addx 2
addx 5
addx 2
addx -29
addx 35
addx -3
addx -25
addx -8
addx 1
noop
addx 4
addx 3
addx -2
addx 5
noop
addx 8
addx -6
noop
addx -3
addx 10
noop
noop
addx 6
addx -1
addx -18
addx 21
addx -30
addx 37
addx 1
noop
noop
noop
noop";

struct CPU {
    cycle: i64,
    reg_x: i64,
    program_counter: usize,
    instruction_start_cycle: i64,
}
fn cpu() -> CPU {
    CPU {
        cycle: 0,
        reg_x: 1,
        program_counter: 0,
        instruction_start_cycle: 0,
    }
}
fn cpu_print_status(cpu: &CPU) {
    println!("{:04}: reg: X={}, PC={}", cpu.cycle, cpu.reg_x, cpu.program_counter);
}

fn noop(cpu: &mut CPU) -> bool {
    cpu.cycle >= cpu.instruction_start_cycle + 1
}
fn addx(cpu: &mut CPU, args: &str) -> bool {
    if cpu.cycle >= cpu.instruction_start_cycle + 2 {
        cpu.reg_x += args.parse::<i64>().unwrap();
        true
    }
    else {
        false
    }
}
fn run(cpu: &mut CPU, program: &Vec<&str>) {
    let (opcode, args) = program[cpu.program_counter].split_once(" ").unwrap_or((program[cpu.program_counter], ""));
    let next_instruction = match opcode {
        "addx" => addx(cpu, args),
        "noop" => noop(cpu),
        _ => panic!("unknown opcode {}", opcode),
    };
    if next_instruction {
        cpu.program_counter += 1;
        cpu.instruction_start_cycle = cpu.cycle;
    }
}
fn parse_program(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

fn part1() {
    let mut signal_strength: i64 = 0;

    let mut cpu = cpu();
    let program = parse_program(_INPUT_PART1);
    while cpu.program_counter < program.len() {
        run(&mut cpu, &program);
        cpu.cycle += 1;

        if (cpu.cycle - 20) % 40 == 0 {
            let signal = cpu.reg_x * cpu.cycle as i64;
            signal_strength += signal;
            println!("{:04}: signal strength = {} (total {})", cpu.cycle, signal, signal_strength);
            cpu_print_status(&cpu);
        }
    }
    cpu_print_status(&cpu);
}

fn part2() {
    let mut cpu = cpu();
    let program = parse_program(_INPUT_PART1);
    while cpu.program_counter < program.len() {
        run(&mut cpu, &program);

        let pos = cpu.cycle % 40;
        print!("{}", if cpu.reg_x - 1 <= pos && pos <= cpu.reg_x + 1 { "#" } else { " " });
        if (cpu.cycle + 1) % 40 == 0 {
            println!("");
        }

        cpu.cycle += 1;
    }
    cpu_print_status(&cpu);
}

fn main() {
    println!("part1");
    part1();
    println!("part2");
    part2();
}
