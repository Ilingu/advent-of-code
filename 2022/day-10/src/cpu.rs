pub struct Cpu {
    cycle: usize,
    register: i32,
    metrics: Vec<i32>,

    display_bus: Option<Box<dyn FnMut(usize, i32, bool) -> ()>>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            register: 1,
            metrics: vec![],

            display_bus: None,
        }
    }

    pub fn exec(&mut self, instructions: &Vec<&str>) {
        for instruction in instructions {
            self.new_cycle();

            let (cmd, arg) = Self::parse_instruction(instruction);
            if cmd == "addx" {
                self.new_cycle();
                self.register += arg.unwrap().parse::<i32>().unwrap();
            }
        }
    }

    pub fn link_display<F>(&mut self, cable: F)
    where
        F: FnMut(usize, i32, bool) -> () + 'static,
    {
        self.display_bus = Some(Box::new(cable));
    }

    pub fn show_screen(&mut self) {
        match self.display_bus.as_mut() {
            Some(call_display) => call_display(0, 0, true),
            None => {}
        }
    }

    fn parse_instruction(instruction: &str) -> (&str, Option<&str>) {
        let datas = instruction.split_whitespace().collect::<Vec<&str>>();
        return match datas.len() {
            1 => (datas[0], None),
            2 => (datas[0], Some(datas[1])),
            _ => panic!("invalid instruction"),
        };
    }

    fn new_cycle(&mut self) {
        self.cycle += 1;

        // calling display/crt
        match self.display_bus.as_mut() {
            Some(call_display) => call_display(self.cycle, self.register, false),
            None => {}
        }

        self.watch_cycle();
    }

    fn watch_cycle(&mut self) {
        if self.cycle % 40 == 20 {
            self.metrics.push((self.cycle as i32) * self.register);
        }
    }

    pub fn compute_p1(&self) -> i32 {
        if self.metrics.len() == 0 {
            panic!("nothing was executed!")
        }
        return self.metrics.iter().sum();
    }
}
