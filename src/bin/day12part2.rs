use aoc_lib::utils::parsing_input;
use core::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge(String, String);

impl FromStr for Edge {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('-');
        let n1 = iter.next().ok_or("no items in string")?;
        let n2 = iter.next().ok_or("no second item in string")?;
        iter.next()
            .is_none()
            .then(move || Edge(n1.to_string(), n2.to_string()))
            .ok_or("more than 2 items in string")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum CaveType {
    Small,
    Big,
    Terminal,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Node {
    id: usize,
    cave_type: CaveType,
}

pub const NODE_START: Node = Node {
    id: 0,
    cave_type: CaveType::Terminal,
};
pub const NODE_END: Node = Node {
    id: 1,
    cave_type: CaveType::Terminal,
};

type GraphRef<'a> = &'a [Vec<Node>];

impl Node {
    fn make_builder() -> impl (FnMut(String) -> Self) {
        let mut node_id_lookup = std::collections::HashMap::<String, Node>::new();
        node_id_lookup.insert("start".to_string(), NODE_START);
        node_id_lookup.insert("end".to_string(), NODE_END);

        move |s| {
            if let Some(&node) = node_id_lookup.get(&s) {
                node
            } else {
                let node = Self {
                    id: node_id_lookup.len(),
                    cave_type: if s.to_uppercase() == s {
                        CaveType::Big
                    } else {
                        CaveType::Small
                    },
                };
                println!("node mapping: {:?} == {:?}", s, node.id);
                node_id_lookup.insert(s, node);

                node
            }
        }
    }

    pub fn build_graph<I: Iterator<Item = Edge>>(iter: I) -> Vec<Vec<Self>> {
        let mut builder = Self::make_builder();

        iter.fold(vec![Vec::new(), Vec::new()], |mut result, Edge(s1, s2)| {
            let n1 = builder(s1);
            if n1.id >= result.len() {
                result.push(Vec::new());
            }

            let n2 = builder(s2);
            if n2.id >= result.len() {
                result.push(Vec::new());
            }

            result[n1.id].push(n2);
            result[n2.id].push(n1);

            result
        })
    }
}

#[derive(Debug, Clone)]
pub struct PathStreamIter<'a> {
    graph: GraphRef<'a>,
    path: Vec<Node>,
    node_exits: Vec<usize>,
    visited: std::collections::HashMap<Node, usize>,
    visited_small_twice: bool,
}

impl<'a> PathStreamIter<'a> {
    pub fn new(graph: GraphRef<'a>) -> PathStreamIter<'a> {
        let mut s = Self {
            graph,
            path: Vec::new(),
            node_exits: Vec::new(),
            visited: std::collections::HashMap::new(),
            visited_small_twice: false,
        };
        s.push_if_valid(NODE_START, 0);

        s
    }

    fn push_if_valid(&mut self, new_head: Node, last_exit: usize) -> bool {
        println!("maybe push {:?} {:?}", new_head, last_exit);
        if match new_head.cave_type {
            CaveType::Big => true,
            CaveType::Small => {
                *self.visited.get(&new_head).unwrap_or(&0) < {
                    if self.visited_small_twice {
                        1
                    } else {
                        2
                    }
                }
            }
            CaveType::Terminal => *self.visited.get(&new_head).unwrap_or(&0) == 0,
        } {
            println!("\tyes push");
            self.path.push(new_head);
            self.node_exits.push(last_exit);

            let new_count = 1 + self.visited.get(&new_head).unwrap_or(&0);
            self.visited.insert(new_head, new_count);
            println!(
                "\tvisited {:?} times",
                *self.visited.get(&new_head).unwrap()
            );

            match (new_head.cave_type, new_count) {
                (CaveType::Small, 2) => {
                    self.visited_small_twice = true;
                    println!("\tvisited small cave twice");
                }
                (CaveType::Small, 1) => {}
                (CaveType::Small, _) => unreachable!(),
                _ => {}
            }

            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<(Node, usize)> {
        let path_head = self.path.pop()?;
        let last_exit = self
            .node_exits
            .pop()
            .expect("`path` and `node_exits` should be the same length");
        println!("pop {:?} {:?}", path_head, last_exit);
        let count = self.visited.remove(&path_head).unwrap();
        if count > 1 {
            self.visited.insert(path_head, count - 1);
        } else if count < 1 {
            unreachable!();
        }

        Some((path_head, last_exit))
    }

    fn head(&self) -> Option<(&Node, &usize)> {
        let path_head = (!self.path.is_empty()).then(|| &self.path[self.path.len() - 1])?;
        let last_exit = &self.node_exits[self.path.len() - 1];

        Some((path_head, last_exit))
    }

    pub fn next_ref(&mut self) -> Option<&Vec<Node>> {
        println!("start!");
        loop {
            let (&path_head, &last_exit) = self.head()?;
            println!("path head: {:?} last exit: {:?}", path_head, last_exit);

            if path_head == NODE_END || last_exit == self.graph[path_head.id].len() {
                println!("pop {:?}", path_head);
                self.pop();
                continue;
            }

            for exit in last_exit..self.graph[path_head.id].len() {
                self.node_exits[self.path.len() - 1] = exit + 1;
                let next_node = self.graph[path_head.id][exit];

                if self.push_if_valid(next_node, 0) {
                    if next_node == NODE_END {
                        return Some(&self.path);
                    }

                    break;
                }
            }
        }
    }

    /*
    fn iter<'b>(&'b mut self) -> impl 'b + Iterator<Item = &'b Vec<Node>> {
        std::iter::repeat(()).filter_map(move |_| self.next())
    }
    */
}

pub fn count_paths(graph: GraphRef) -> usize {
    let mut path_iterlike = PathStreamIter::new(graph);

    let mut result = 0;
    while let Some(_path_ref) = path_iterlike.next_ref() {
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_graph_eg1() {
        let cave_edges = vec![
            Edge("start".to_string(), "A".to_string()),
            Edge("start".to_string(), "b".to_string()),
            Edge("A".to_string(), "c".to_string()),
            Edge("A".to_string(), "b".to_string()),
            Edge("b".to_string(), "d".to_string()),
            Edge("A".to_string(), "end".to_string()),
            Edge("b".to_string(), "end".to_string()),
        ];
        let graph = Node::build_graph(cave_edges.into_iter());

        let nodes = vec![
            NODE_START,
            NODE_END,
            Node {
                id: 2,
                cave_type: CaveType::Big,
            },
            Node {
                id: 3,
                cave_type: CaveType::Small,
            },
            Node {
                id: 4,
                cave_type: CaveType::Small,
            },
            Node {
                id: 5,
                cave_type: CaveType::Small,
            },
        ];
        assert_eq!(
            graph,
            vec![
                vec![nodes[2], nodes[3]],
                vec![nodes[2], nodes[3]],
                vec![nodes[0], nodes[4], nodes[3], nodes[1]],
                vec![nodes[0], nodes[2], nodes[5], nodes[1]],
                vec![nodes[2]],
                vec![nodes[3]],
            ]
        );
    }

    #[test]
    fn test_count_paths_eg1() {
        let cave_edges = vec![
            Edge("start".to_string(), "A".to_string()),
            Edge("start".to_string(), "b".to_string()),
            Edge("A".to_string(), "c".to_string()),
            Edge("A".to_string(), "b".to_string()),
            Edge("b".to_string(), "d".to_string()),
            Edge("A".to_string(), "end".to_string()),
            Edge("b".to_string(), "end".to_string()),
        ];
        let graph = Node::build_graph(cave_edges.into_iter());
        println!("graph: {:?}", graph);

        let result = count_paths(&graph);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_count_paths_eg2() {
        let cave_edges = vec![
            Edge("dc".to_string(), "end".to_string()),
            Edge("HN".to_string(), "start".to_string()),
            Edge("start".to_string(), "kj".to_string()),
            Edge("dc".to_string(), "start".to_string()),
            Edge("dc".to_string(), "HN".to_string()),
            Edge("LN".to_string(), "dc".to_string()),
            Edge("HN".to_string(), "end".to_string()),
            Edge("kj".to_string(), "sa".to_string()),
            Edge("kj".to_string(), "HN".to_string()),
            Edge("kj".to_string(), "dc".to_string()),
        ];
        let graph = Node::build_graph(cave_edges.into_iter());
        println!("graph: {:?}", graph);

        let result = count_paths(&graph);
        assert_eq!(result, 103);
    }

    #[test]
    fn test_count_paths_eg3() {
        let cave_edges = vec![
            Edge("fs".to_string(), "end".to_string()),
            Edge("he".to_string(), "DX".to_string()),
            Edge("fs".to_string(), "he".to_string()),
            Edge("start".to_string(), "DX".to_string()),
            Edge("pj".to_string(), "DX".to_string()),
            Edge("end".to_string(), "zg".to_string()),
            Edge("zg".to_string(), "sl".to_string()),
            Edge("zg".to_string(), "pj".to_string()),
            Edge("pj".to_string(), "he".to_string()),
            Edge("RW".to_string(), "he".to_string()),
            Edge("fs".to_string(), "DX".to_string()),
            Edge("pj".to_string(), "RW".to_string()),
            Edge("zg".to_string(), "RW".to_string()),
            Edge("start".to_string(), "pj".to_string()),
            Edge("he".to_string(), "WI".to_string()),
            Edge("zg".to_string(), "he".to_string()),
            Edge("pj".to_string(), "fs".to_string()),
            Edge("start".to_string(), "RW".to_string()),
        ];
        let graph = Node::build_graph(cave_edges.into_iter());
        println!("graph: {:?}", graph);

        let result = count_paths(&graph);
        assert_eq!(result, 3509);
    }
}

fn main() {
    println!("Enter input:");
    let stdin = std::io::stdin();
    let parsed_inputs = parsing_input::<_, Edge>(stdin.lock());

    let graph = Node::build_graph(parsed_inputs);
    let result = count_paths(&graph);
    println!("paths count: {:?}", result);
}
