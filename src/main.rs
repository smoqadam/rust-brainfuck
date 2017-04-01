
#[derive(Debug)]
struct Brainfuck<'a> {
    code: Vec<&'a str>,
    pointer: u8,
    mem: Vec<u8>,
}
impl<'a> Brainfuck<'a> {
    fn new(code: &str) -> Brainfuck {
        let c: Vec<&str> = code.split("").collect();
        Brainfuck {
            code: c,
            pointer: 0,
            mem: vec![0],
        }
    }

    fn dot(&mut self) {
        print!("{}", self.mem[self.pointer as usize] as char);
    }

    fn lt(&mut self) {
        if self.pointer != 0 {
            self.pointer -= 1;
        }
    }

    fn gt(&mut self) {
        self.mem.push(0);
        self.pointer += 1;
    }

    fn plus(&mut self) {
        self.mem[self.pointer as usize] += 1;
    }
    fn minus(&mut self) {
        self.mem[self.pointer as usize] -= 1;
    }
    fn clean(&mut self) {
        self.code.retain(|&c| {
                             c == "+" || c == "-" || c == "." || c == "!" || c == "," ||
                             c == "[" || c == "]" ||
                             c == "<" || c == ">"
                         });
    }

    fn compile(&mut self) {
        println!("{:?}", self.mem);

        self.clean();
        let mut begins: Vec<u8> = Vec::new();
        let mut ends: Vec<u8> = Vec::new();
        let mut i: usize = 0;
        let op = self.code.clone();
        while op.len() > i {
            if op[i] == "+" {
                self.plus();
            }
            if op[i] == "-" {
                self.minus();
            }
            if op[i] == ">" {
                self.gt();
            }
            if op[i] == "<" {
                self.lt();
            }
            if op[i] == "." {
                self.dot();
            }
            if op[i] == "[" {
                if self.mem[self.pointer as usize] != 0 {
                    begins.push(i as u8);
                } else {
                    match ends.first() {
                        Some(&index) => i = index as usize,
                        None => println!(""),
                    }
                    begins.pop();
                }
            }
            if op[i] == "]" {
                ends.push(i as u8);
                match begins.last() {
                    Some(&index) => i = index as usize - 1,
                    None => println!(""),
                };
            }
            i += 1;
        }
        println!("{:?}", self.mem);
    }
}


fn main() {
    let input = "++++
[
> ++++++++++
+++++++++
++++++
<-
]";
    let mut bf = Brainfuck::new(input);
    bf.compile();
}
