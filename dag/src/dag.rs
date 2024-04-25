use std::fmt;

use crate::core::*;
use crate::node::Node;
use crate::stats::*;

pub struct Dag {
    root: Option<Box<Node>>,
}
impl Dag {
    pub fn new() -> Self {
        Dag { root: None }
    }

    pub fn from_file(file: &str) -> Result<Self,FileError> {
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
        self.root = Some(Box::new(root));
    }

    fn get_root(&self) -> Option<&Node> {
        self.root.as_ref().map(|boxed_node| &**boxed_node)
    }
}

impl DAGStats for Dag {
    fn calculate(&self) -> DAGStatsData {
        let builder = DAGStatsBuilder::new();

        builder.build()
    }
}
impl fmt::Display for Dag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        fn display_node(node: &Node, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
            let indent = "  ".repeat(level);

            if let Some(ref left) = node.left {
                write!(f, "{}{}--", indent, node.val)?;
                if left.val == node.val {
                    write!(f, "||\n")?;
                } else {
                    write!(f, "//\n")?;
                }
                display_node(left, f, level + 1)?;
            }

            write!(f, "{}{}\n", indent, node.val)?;

            if let Some(ref right) = node.right {
                write!(f, "{}{}--", indent, node.val)?;
                if right.val == node.val {
                    write!(f, "||\n")?;
                } else {
                    write!(f, "\\\\\n")?;
                }
                display_node(right, f, level + 1)?;
            }

            Ok(())
        }

        if let Some(ref root) = self.root {
            display_node(root, f, 0)
        } else {
            write!(f, "Empty DAG")
        }
    }
}

#[derive(Debug)]
struct DagFileData {
    nodes: Vec<(i32, i32, i32)>, // (node_val, left_val, right_val)
}

impl From<ExistingFile> for DagFileData {
    fn from(file: ExistingFile) -> Self {
        let mut nodes = Vec::new();
        for line in file.read_lines() {
            let vals: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            nodes.push((vals[0], vals[1], vals[2]));
        }

        DagFileData { nodes }
    }
}
impl From<DagFileData> for Dag {
    fn from(data: DagFileData) -> Self {
        let mut dag = Dag::new();
        let mut nodes: Vec<Option<Node>> = vec![None; data.nodes.len() + 1];
        for (node_val, left_val, right_val) in data.nodes {
            nodes[node_val as usize] = Some(Node::new(node_val));

            if left_val != -1 {
                if let Some(node) = nodes[left_val as usize].take() {
                    nodes[node_val as usize].as_mut().unwrap().left = Some(Box::new(node));
                }
            }

            if right_val != -1 {
                if let Some(node) = nodes[right_val as usize].take() {
                    nodes[node_val as usize].as_mut().unwrap().right = Some(Box::new(node));
                }
            }
        }

        dag.set_root(nodes[1].take().unwrap()); // Set the root node for the DAG
        dag
    }
}
