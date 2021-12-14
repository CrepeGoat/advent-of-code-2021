#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Node {
    id: usize,
    is_big: bool,
}

const NODE_START: Node = Node {
    id: 0,
    is_big: false,
};
const NODE_END: Node = Node {
    id: 1,
    is_big: true,
};

type Graph = Vec<Vec<Node>>;

impl Node {
    fn make_builder<'a>() -> impl (FnMut(&'a str) -> Self) {
        let mut node_id_lookup = std::collections::HashMap::new();
        node_id_lookup.insert("start", NODE_START);
        node_id_lookup.insert("end", NODE_END);

        move |s| {
            if let Some(&node) = node_id_lookup.get(&s) {
                node
            } else {
                let node = Self {
                    id: node_id_lookup.len(),
                    is_big: s.to_lowercase() == s,
                };
                node_id_lookup.insert(&s, node);

                node
            }
        }
    }

    fn build_graph(inputs: &Vec<(String, String)>) -> Vec<Vec<Self>> {
        let builder = Self::make_builder();

        inputs.iter().fold(Vec::new(), |result, (s1, s2)| {
            let n1 = builder(s1);
            let n2 = builder(s2);

            if n1.id >= result.len() {
                result.push(Vec::new());
            }
            result[n1.id].push(n2);

            result
        })
    }
}

#[derive(Debug, Clone)]
struct PathStreamIter<'a> {
    graph: &'a Graph,
    path: Vec<Node>,
    node_exits: Vec<usize>,
    visited: std::collections::HashSet<Node>,
}

impl<'a> PathStreamIter<'a> {
    pub fn new(graph: &'a Graph) -> PathStreamIter<'a> {
        let mut s = Self {
            graph,
            path: vec![NODE_START],
            node_exits: vec![0],
            visited: std::collections::HashSet::new(),
        };
        s.visited.insert(NODE_START);

        s
    }

    pub fn next<'b>(&'b mut self) -> Option<&'b Vec<Node>> {
        loop {
            let path_head = self.path.pop()?;
            let last_exit = self
                .node_exits
                .pop()
                .expect("`path` and `node_exits` should be the same length");

            if path_head == NODE_END {
                continue;
            }

            for (i, exit) in (last_exit + 1..self.graph[path_head.id].len()).enumerate() {
                let next_node = &self.graph[path_head.id][exit];
                if next_node.is_big || !self.visited.contains(&next_node) {
                    self.path.push(path_head);
                    self.node_exits.push(last_exit + i + 1);

                    self.path.push(*next_node);
                    self.node_exits.push(0);
                    break;
                }
            }

            if self.path[self.path.len() - 1] == NODE_END {
                return Some(&self.path);
            }
        }
    }

    /*
    fn iter<'b>(&'b mut self) -> impl 'b + Iterator<Item = &'b Vec<Node>> {
        std::iter::repeat(()).filter_map(move |_| self.next())
    }
    */
}
