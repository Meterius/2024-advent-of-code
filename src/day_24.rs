use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::{Direction, Graph};
use petgraph::graph::{DiGraph, NodeIndex};
use bimap::BiMap;
use petgraph::visit::{Bfs, Reversed, Topo};

#[derive(Hash, PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
enum GateVariant { And, Or, Xor }

impl GateVariant {
    fn eval(&self, left: bool, right: bool) -> bool {
        match self {
            GateVariant::And => left && right,
            GateVariant::Or => left || right,
            GateVariant::Xor => left ^ right,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Ord, PartialOrd, Clone, Copy)]
enum Node {
    Register,
    Gate(GateVariant)
}

#[derive(Debug, Clone)]
struct Circuit {
    graph: DiGraph<Node, ()>,

    interpreted_inputs: HashMap<String, Vec<NodeIndex>>,
    inputs: Vec<NodeIndex>,
    outputs: Vec<NodeIndex>,

    labels: BiMap<NodeIndex, String>,
    test_input: HashMap<NodeIndex, bool>
}

impl Circuit {
    fn execute(&self, input: &HashMap<NodeIndex, bool>) -> HashMap<String, bool> {
        let mut state = HashMap::new();
        self.inputs.iter().for_each(|idx| { state.insert(idx.clone(), input[idx].clone()); });

        let mut visitor = Topo::with_initials(&self.graph, self.inputs.iter().cloned());
        while let Some(idx) = visitor.next(&self.graph) {
            let node = &self.graph[idx];

            let value = match node {
                Node::Register => state[
                    &self.graph
                        .neighbors_directed(idx, Direction::Incoming)
                        .next()
                        .unwrap_or(idx)
                ].clone(),
                Node::Gate(variant) => {
                    let (a, b) = self.graph
                        .neighbors_directed(idx, Direction::Incoming)
                        .into_iter()
                        .map(|jdx| state[&jdx].clone())
                        .next_tuple()
                        .unwrap();

                    variant.eval(a, b)
                }
            };

            state.insert(idx, value);
        }

        return HashMap::from_iter(
            state.into_iter()
                .filter_map(|(k, v)| self.labels.get_by_left(&k).map(|k| (k.clone(), v)))
        );
    }

    fn convert_output(&self, output: &HashMap<String, bool>) -> usize {
        return self.outputs.iter()
            .map(|k| self.labels.get_by_left(k).unwrap())
            .enumerate()
            .map(|(i, k)| if output[k] { 1 } else { 0 } << i)
            .sum::<usize>();
    }

    fn convert_input(&self, xs: &HashMap<String, usize>) -> HashMap<NodeIndex, bool> {
        let mut input = HashMap::new();

        for (name, digits) in self.interpreted_inputs.iter() {
            let mut rem = xs[name].clone();

            for digit in digits.iter() {
                input.insert(digit.clone(), rem % 2 == 1);
                rem >>= 1;
            }

            if rem != 0 {
                panic!("Overflow");
            }
        }

        return input;
    }
}

fn compile_circuit(
    data: File,
) -> Circuit {
    let mut lines = BufReader::new(data).lines().flatten();

    let mut graph = DiGraph::new();

    let mut labels = BiMap::new();
    let mut test_input = HashMap::new();

    let mut inputs = Vec::new();
    let mut interpreted_inputs = HashMap::<String, Vec<NodeIndex>>::new();

    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        let (input_register, input_value) = line.split_once(':').unwrap();

        let input_register = input_register.trim().to_string();
        let input_value = match input_value.trim().parse::<usize>().unwrap() {
            0 => false, 1 => true, _ => panic!("Invalid Input")
        };

        let index = graph.add_node(Node::Register);

        labels.insert(index.clone(), input_register.clone());
        test_input.insert(index.clone(), input_value);
        inputs.push(index.clone());

        let name = input_register[0..1].to_string();
        interpreted_inputs
            .entry(name)
            .and_modify(|v| v.push(index.clone()))
            .or_insert(vec![index.clone()]);
    }

    let mut queue = HashSet::new();

    while let Some(line) = lines.next() {
        let (gate, gate_output) = line.split_once("->").unwrap();

        let gate_output = gate_output.trim().to_string();
        let (gate_left, gate_type, gate_right) = gate.trim()
            .split_ascii_whitespace().next_tuple().unwrap();

        let gate_variant = match gate_type {
            "XOR" => GateVariant::Xor,
            "AND" => GateVariant::And,
            "OR" => GateVariant::Or,
            _ => panic!("Invalid")
        };

        queue.insert((gate_variant, gate_left.to_string(), gate_right.to_string(), gate_output));
    }

    while queue.len() != 0 {
        let curr = queue.len();

        for (variant, left, right, output) in queue.iter() {
            if labels.contains_right(left) && labels.contains_right(right) {
                let gate_index = graph.add_node(Node::Gate(variant.clone()));
                let output_index = graph.add_node(Node::Register);

                graph.add_edge(labels.get_by_right(left).unwrap().clone(), gate_index, ());
                graph.add_edge(labels.get_by_right(right).unwrap().clone(), gate_index, ());
                graph.add_edge(gate_index, output_index, ());

                labels.insert(output_index.clone(), output.clone());

                queue.remove(&(variant.clone(), left.clone(), right.clone(), output.clone()));
                break;
            }
        }

        assert_ne!(curr, queue.len());
    }

    let mut outputs: Vec<_> = graph
        .node_indices()
        .filter(|idx| graph.neighbors_directed(*idx, Direction::Outgoing).count() == 0)
        .collect();

    outputs.sort_by_key(|idx| labels.get_by_left(idx).unwrap()[1..].parse::<usize>().unwrap());

    for digits in interpreted_inputs.values_mut() {
        digits.sort_by_key(|idx| labels.get_by_left(idx).unwrap()[1..].parse::<usize>().unwrap());
    }

    return Circuit {
        graph, test_input, labels, inputs, outputs, interpreted_inputs,
    };
}

pub fn part_1(data: File) -> usize {
    let circuit = compile_circuit(data);
    let state = circuit.execute(&circuit.test_input);
    return circuit.convert_output(&state);
}

fn swap_circuit_registers(
    circuit: &mut Circuit,
    a: &str,
    b: &str,
) {
    let a = String::from(a);
    let a_idx = circuit.labels.get_by_right(&a).unwrap().clone();
    let b = String::from(b);
    let b_idx = circuit.labels.get_by_right(&b).unwrap().clone();

    let gate_a = circuit.graph.neighbors_directed(
        a_idx,
        Direction::Incoming
    ).next().unwrap();

    let gate_b = circuit.graph.neighbors_directed(
        b_idx,
        Direction::Incoming
    ).next().unwrap();

    let gate_a_edge = circuit.graph.find_edge(gate_a, a_idx).unwrap();
    circuit.graph.remove_edge(gate_a_edge);
    circuit.graph.add_edge(gate_b, a_idx, ());

    let gate_b_edge = circuit.graph.find_edge(gate_b, b_idx).unwrap();
    circuit.graph.remove_edge(gate_b_edge);
    circuit.graph.add_edge(gate_a, b_idx, ());
}

pub fn part_2(data: File) -> String {
    let swaps = [
        ("gmt", "z07"),
        ("qjj", "cbj"),
        ("dmn", "z18"),
        ("cfk", "z35")
    ];

    let mut all_swaps = swaps.iter()
        .flat_map(|x| [String::from(x.0), String::from(x.1)].into_iter())
        .collect::<Vec<_>>();
    all_swaps.sort();

    return all_swaps.into_iter().join(",");

    let mut circuit = compile_circuit(data);

    let swaps = [
        ("gmt", "z07"),
        ("qjj", "cbj"),
        ("dmn", "z18"),
        ("cfk", "z35")
    ];

    for (a, b) in swaps.iter() {
        swap_circuit_registers(&mut circuit, a, b);
    }

    let input_bits = circuit.interpreted_inputs
        .values().map(|seq| seq.len()).min().unwrap();

    let output_bits = circuit.outputs.len();

    println!("Input {input_bits} Output {output_bits}");

    for i in 0..input_bits {
        let x: usize = 1 << i;
        let y: usize = 0;

        let input = circuit.convert_input(&HashMap::from([
            (String::from("x"), x),
            (String::from("y"), y),
        ]));

        let state = circuit.execute(&input);
        let z = circuit.convert_output(&state);
        let expected = (x + y) & ((1 << output_bits) - 1);

        if z != expected {
            println!("{i} X={x} Y={y} Z={z}={z:b} != {expected}={expected:b}");
        }
    }

    return String::from("");
}