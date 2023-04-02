use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Component {
    Gate(String, String, String),
    WireSignal(String),
}

struct Circuit {
    gates: HashMap<String, Component>,
    wires: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            gates: HashMap::new(),
            wires: HashMap::new(),
        }
    }

    fn build(&mut self, filename: &String) {
        let input = read_to_string(&filename).expect("Unable to read puzzle file.");

        for line in input.lines() {
            let mut tokens: Vec<&str> = line.split(' ').collect();
            let out_wire = tokens.pop().unwrap().to_string();
            tokens.pop(); // Get rid of "->"
            let component = match tokens.len() {
                1 => Component::WireSignal(tokens[0].to_string()),
                2 => Component::Gate(tokens[1].to_string(), "XOR".to_string(), u16::max_value().to_string(),),
                3 => Component::Gate(tokens[0].to_string(), tokens[1].to_string(), tokens[2].to_string(),),
                _ => panic!("Unexpected number of tokens in expression '{}'", tokens.concat()),
            };
            self.gates.insert(out_wire, component);
        };
    }

    fn get_wire_value(&mut self, wire_name: &str) -> (u16, u16) {
        let component = self.gates[wire_name].clone();
        let first_value = self.evaluate(&component);
        // For part 2
        self.wires.clear();
        self.wires.insert("b".to_string(), first_value);
        (first_value, self.evaluate(&component))
    }

    fn evaluate(&mut self, component: &Component) -> u16 {
        match *component {
            Component::Gate(ref in1, ref op, ref in2) => Circuit::compute(
                op,
                self.evaluate(&Component::WireSignal(in1.to_owned())),
                self.evaluate(&Component::WireSignal(in2.to_owned())),
            ),
            Component::WireSignal(ref val) => match val.parse::<u16>() {
                Ok(v) => v,
                _ => {
                    if !self.wires.contains_key(val) {
                        let comp = self.gates[val].clone();
                        let res = self.evaluate(&comp);
                        self.wires.insert(val.to_owned(), res);
                    }
                    self.wires[val]
                }
            },
        }
    }

    fn compute(op: &str, in1: u16, in2: u16) -> u16 {
        match op {
            "XOR" => in1 ^ in2,
            "AND" => in1 & in2,
            "OR" => in1 | in2,
            "LSHIFT" => in1 << in2,
            "RSHIFT" => in1 >> in2,
            _ => panic!("Unknown gate type: '{}'.", op),
        }
    }
}

fn main() {
    let filename = String::from("puzzle_input.txt");

    let mut circuit = Circuit::new();
    circuit.build(&filename);
    let result = circuit.get_wire_value("a");
    println!("Part One:");
    println!("  Wire 'a' signal: {}", result.0);
    println!("\nPart Two: [override signal of wire 'b' with 'a' and compute 'a' again]");
    println!("  Wire 'a' signal: {}", result.1);
}
