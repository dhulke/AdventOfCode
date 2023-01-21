pub mod input;

pub enum Command {
    ADDX(isize),
    NOOP
}

/**
    A module dedicated to parsing command input from text. There could be other methods in this
    module for parsing from strings, web addresses or other sources. There could also be other
    dedicated modules for parsing commands from other formats.
*/
mod command_text_parser {
    use super::*;

    pub fn parse_line(line: &str) -> Command {
        match line.split_once(' ') {
            Some((_, n)) => Command::ADDX(n.parse()
                .expect("Expect addx command to be followed by a number")),
            None => Command::NOOP
        }
    }
}

/**
    I modeled the problem in a OO fashion, creating a SimpleCpu object that keeps track of cycles,
    register and an interrupt that is a callback that allows for a more dynamic way of computing
    the problem's result.
*/
pub struct SimpleCpu<F: FnMut(usize, isize)> {
    register_x: isize,
    cycle: usize,
    interrupt: F,
}

impl<F: FnMut(usize, isize)> SimpleCpu<F> {
    pub fn from_interrupt(interrupt: F) -> Self {
        Self {
            register_x: 1,
            cycle: 0,
            interrupt
        }
    }
    
    pub fn addx(&mut self, n: isize) {
        self.tick();
        self.tick();
        self.register_x += n;
    }
    
    pub fn noop(&mut self) {
        self.tick();
    }
    
    fn tick(&mut self) {
        self.cycle += 1;
        (self.interrupt)(self.cycle, self.register_x);
    }

    pub fn get_cycle(&self) -> usize {
        self.cycle
    }
}

/// Response to the first part
pub fn get_sum_signal_strengths_at_6_intervals(lines: impl Iterator<Item=String>) -> isize {
    let mut signal_strength_sum = 0;
    let mut simple_cpu = SimpleCpu::from_interrupt(|cycle, register_x| {
        if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180
            || cycle == 220 {
            signal_strength_sum += cycle as isize * register_x;
        }
    });
    for line in lines {
        match command_text_parser::parse_line(&line) {
            Command::ADDX(n) => simple_cpu.addx(n),
            Command::NOOP => simple_cpu.noop()
        }
        // stop computation sooner since we only need 220 cycles
        if simple_cpu.get_cycle() == 220 {
            break;
        }
    }
    signal_strength_sum
}

/// Response to the second part
pub fn render_crt_output(lines: impl Iterator<Item=String>) -> String {
    let mut crt_output = String::new();
    let mut simple_cpu = SimpleCpu::from_interrupt(|cycle, register_x| {
        /*
            Cycle goes from 0 to 39. We're decrementing cycle here because in the problem
            statement cycle 1 refers to the first pixel while register_x 0 refers to the first
            pixel. We need to either decrement cycle or increment register_x and the former seems
            less cumbersome and more in line wiht the domain of the problem.
         */
        let cycle = (cycle - 1) % 40;
        if register_x - 1 <= cycle as isize && register_x + 1 >= cycle as isize {
            crt_output.push_str("#");
        } else {
            crt_output.push_str(".");
        }
        if cycle == 39 {
            crt_output.push_str("\n");
        }
    });

    for line in lines {
        match command_text_parser::parse_line(&line) {
            Command::ADDX(n) => simple_cpu.addx(n),
            Command::NOOP => simple_cpu.noop()
        }
        // stop computation sooner since we only need 240 cycles
        if simple_cpu.get_cycle() == 240 {
            break;
        }
    }
    crt_output
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
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
noop";

    #[test]
    fn test_get_sum_signal_strengths_at_6_intervals() {
        assert_eq!(get_sum_signal_strengths_at_6_intervals(INPUT.lines().map(String::from)), 13140);
    }

    #[test]
    fn test_render_crt_output() {
        assert_eq!(render_crt_output(INPUT.lines().map(String::from)), "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
".to_string());
    }
}