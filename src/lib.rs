#![no_std]
use smallvec::SmallVec;

pub type NodeIndex = usize;
pub type Generation = usize;
pub type NodeHandle = (NodeIndex, Generation);

pub enum GraphError {
    GenerationMismatch,
    Unexpected,
}

#[derive(Default)]
pub struct SmallGraph<T> {
    pub free: SmallVec<[NodeHandle; 128]>,
    pub nodes: SmallVec<[(Generation, Option<T>); 128]>,
    pub connections: SmallVec<[(NodeIndex, NodeIndex); 256]>,
}

impl<T> SmallGraph<T> {
    pub fn new() -> SmallGraph<T> {
        SmallGraph {
            free: SmallVec::new(),
            nodes: SmallVec::new(),
            connections: SmallVec::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> NodeHandle {
        if self.free.len() == 0 {
            let index = self.nodes.len();
            let gen = 0;
            self.nodes.push((gen, Some(value)));
            (index, gen)
        } else {
            let n = self.free.remove(0);
            let index = n.0;
            let gen = n.1;
            self.nodes[index] = (gen + 1, Some(value));
            (n.0, gen + 1)
        }
    }

    pub fn directed_connect(&mut self, parent: NodeHandle, child: NodeHandle) {
        self.connections.push((parent.0, child.0));
    }

    pub fn connect(&mut self, a: NodeHandle, b: NodeHandle) {
        self.connections.push((a.0, b.0));
        self.connections.push((b.0, a.0));
    }

    pub fn disconnect(&mut self, n: NodeHandle) {
        self.connections
            .retain(|&mut connection| (connection).0 != n.0 && (connection).1 != n.0);
    }

    pub fn is_connected(&mut self, a: NodeHandle, b: NodeHandle) -> bool {
        self.connections
            .iter()
            .find(|&connection| {
                (connection.0 == a.0 && connection.1 == b.0)
                    || (connection.1 == a.0 && connection.0 == b.0)
            })
            .is_some()
    }

    pub fn is_directed_connected(&mut self, parent: NodeHandle, child: NodeHandle) -> bool {
        self.connections
            .iter()
            .find(|&connection| connection.0 == parent.0 && connection.1 == child.0)
            .is_some()
    }

    pub fn remove(&mut self, n: NodeHandle) -> Result<T, GraphError> {
        let index = n.0;
        let gen = n.1;
        if self.nodes[index].0 == gen {
            self.disconnect(n);
            let mut r = (gen + 1, None);
            core::mem::swap(&mut self.nodes[index], &mut r);
            self.free.push(n);
            Ok(r.1.unwrap())
        } else {
            Err(GraphError::GenerationMismatch)
        }
    }

    pub fn get(&self, n: NodeHandle) -> Result<&T, GraphError> {
        let index = n.0;
        let gen = n.1;
        if self.nodes[index].0 == gen {
            if let Some(value) = &self.nodes[index].1 {
                Ok(value)
            } else {
                Err(GraphError::Unexpected)
            }
        } else {
            Err(GraphError::GenerationMismatch)
        }
    }

    pub fn get_mut(&mut self, n: NodeHandle) -> Result<&mut T, GraphError> {
        let index = n.0;
        let gen = n.1;
        if self.nodes[index].0 == gen {
            if let Some(value) = &mut self.nodes[index].1 {
                Ok(value)
            } else {
                Err(GraphError::Unexpected)
            }
        } else {
            Err(GraphError::GenerationMismatch)
        }
    }
}
