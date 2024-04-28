use std::collections::HashMap;
use std::fmt;


use itertools::Itertools;

use crate::core::*;
use crate::node::*;
use crate::stats::*;

pub struct Dag {
    root: Pointer<Node>,
}
impl Dag {
    pub fn new() -> Self {
        Dag {
            root: Pointer(None),
        }
    }

    pub fn from_file(file: &str) -> Result<Self, FileError> {
        let file_result: Result<ExistingFile, FileError> = ExistingFile::new(file);
        match file_result {
            Ok(file) => {
                let dag_file_data: DagFileData = file.into();
                Ok(Dag::from(dag_file_data))
            }
            Err(err) => {
                println!("Error opening file: {:?}", err);
                Err(err)
            }
        }
    }
    fn set_root(&mut self, root: Node) {
        self.root = Pointer::new(root);
    }
}
impl fmt::Display for Dag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.root.as_ref())
    }
}
impl DAGStats for Dag {
    fn calculate(&self) -> DAGStatsData {
        let mut builder = DAGStatsBuilder::new();
        builder.set_average_depth(self.get_average_depth());
        builder.set_average_nodes_per_depth(self.get_average_nodes_per_depth());
        builder.set_average_in_references(self.get_average_in_references());
        builder.set_max_depth(self.get_max_depth());
        builder.build()
    }
    fn get_average_depth(&self) -> f64 {
        fn depth_recursive(
            pointer: &Pointer<Node>,
            current_depth: usize,
            total: &mut usize,
            count: &mut usize,
        ) {
            *total += current_depth;
            *count += 1;

            if !pointer.as_ref().left.is_none() {
                depth_recursive(&pointer.as_ref().left, current_depth + 1, total, count);
            }
            if !pointer.as_ref().right.is_none() {
                depth_recursive(&pointer.as_ref().right, current_depth + 1, total, count);
            }
        }

        let mut total_depth = 0;
        let mut node_count = 0;
        depth_recursive(&self.root, 0, &mut total_depth, &mut node_count);

        if node_count == 0 {
            0.0 // Avoid division by zero
        } else {
            total_depth as f64 / node_count as f64
        }
    }

    fn get_average_nodes_per_depth(&self) -> f64 {
        fn count_nodes(pointer: &Pointer<Node>, depth: usize, counts: &mut Vec<usize>) {
            if depth >= counts.len() {
                counts.resize(depth + 1, 0);
            }
            counts[depth] += 1;

            if !pointer.as_ref().left.is_none() {
                count_nodes(&pointer.as_ref().left, depth + 1, counts);
            }
            if !pointer.as_ref().right.is_none() {
                count_nodes(&pointer.as_ref().right, depth + 1, counts);
            }
        }

        let mut counts = Vec::new();
        count_nodes(&self.root, 0, &mut counts);

        let total_depths = counts.len() as f64;
        let total_nodes: usize = counts.iter().sum();
        let average_nodes = if total_depths > 0.0 {
            total_nodes as f64 / total_depths
        } else {
            0.0
        };

        average_nodes
    }

    fn get_average_in_references(&self) -> f64 {
        fn count_references(node: &Pointer<Node>) -> usize {
            let mut count = 1; 
            if !node.as_ref().left.is_none() {
                count += count_references(&node.as_ref().left);
            }
            if !node.as_ref().right.is_none() {
                count += count_references(&node.as_ref().right);
            }
            count
        }
        fn count_nodes(node: &Pointer<Node>) -> usize {
            fn count_recursive(node: &Pointer<Node>) -> usize {
                let mut count = 1;
                if !node.as_ref().left.is_none() {
                    count += count_recursive(&node.as_ref().left);
                }
                if !node.as_ref().right.is_none() {
                    count += count_recursive(&node.as_ref().right);
                }
                count
            }
            count_recursive(node)
        }
        let total_references = count_references(&self.root);
        let total_nodes = count_nodes(&self.root);

        if total_nodes == 0 {
            0.0 // Avoid division by zero
        } else {
            (total_references as f64) / (total_nodes as f64)
        }
    }
    
    fn get_max_depth(&self) -> usize {
        fn max_depth_recursive(node: &Pointer<Node>, current_depth: usize, max_depth: &mut usize) {
            // Update max_depth if the current depth is greater
            *max_depth = (*max_depth).max(current_depth);

            if !node.as_ref().left.is_none() {
                max_depth_recursive(&node.as_ref().left, current_depth + 1, max_depth);
            }
            if !node.as_ref().right.is_none() {
                max_depth_recursive(&node.as_ref().right, current_depth + 1, max_depth);
            }
        }

        let mut max_depth = 0;
        max_depth_recursive(&self.root, 1, &mut max_depth); // Start depth at 1 for the root node
        max_depth
    }
    
    fn get_nodes_count(&self) -> usize {
        0
    }
    
    fn get_edges_count(&self) -> usize {
        0
    }
    
    fn get_nodes_with_no_incoming_edges(&self) -> usize {
        0
    }
    
    fn get_isolated_nodes_count(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct DagFileData {
    num_nodes: i128,
    nodes: Vec<(i128, i128, i128)>, // (node_val, left_val, right_val)
}

impl From<ExistingFile> for DagFileData {
    fn from(file: ExistingFile) -> Self {
        let mut nodes = Vec::new();
        let lines = file.read_lines().collect_vec();
        let mut ne: (i128, i128, i128) = (-1, -1, -1);
        let mut n: i128 = -1;
        lines.iter().enumerate().for_each(|(i, line)| {
            let vals: Vec<i128> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            match i {
                0 => {
                    n = vals[0];
                    nodes.push((1 as i128, -1, -1));
                }
                _ if i + 1 == n as usize => {
                    ne = (n, vals[0], vals[1]);
                }
                _ => {
                    nodes.push(((i + 1) as i128, vals[0], vals[1]));
                }
            }
        });
        if ne.0 != -1 {
            nodes.push(ne);
        }
        // println!("nodes: {:?}", nodes);
        DagFileData {
            num_nodes: n,
            nodes,
        }
    }
}

impl From<DagFileData> for Dag {
    fn from(data: DagFileData) -> Self {
        let mut dag = Dag::new();
        let mut nodes_map: HashMap<i128, Pointer<Node>> = HashMap::new();

        // Create nodes without child references
        for (node_val, _, _) in &data.nodes {
            nodes_map
                .entry(*node_val)
                .or_insert_with(|| Pointer::new(Node::new(*node_val)));
        }

        fn get_node(nodes_map: &HashMap<i128, Pointer<Node>>, val: i128) -> Option<&Pointer<Node>> {
            nodes_map.get(&val)
        }

        for i in (0..data.nodes.len()).rev() {
            let (node_val, left_val, right_val) = data.nodes[i];
            //println!("val: {:?} left: {:?} right: {:?}", node_val, left_val, right_val);
            if left_val != -1 {
                if let Some(left_node) = get_node(&nodes_map, left_val) {
                    let pt: &Pointer<Node> = get_node(&nodes_map, node_val).unwrap();
                    pt.as_mut_ref().left = left_node.clone();
                }
            }
            if right_val != -1 {
                if let Some(right_node) = get_node(&nodes_map, right_val) {
                    let pt: &Pointer<Node> = get_node(&nodes_map, node_val).unwrap();
                    pt.as_mut_ref().right = right_node.clone();
                }
            }
        }
        if let Some(root) = nodes_map.get(&data.num_nodes) {
            dag.set_root(root.as_ref().clone());
        }
        dag
    }
}
