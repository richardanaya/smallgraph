#![no_std]
use smallvec::SmallVec;

pub type NodeIndex = usize;
pub type Generation = usize;
pub type NodeHandle = (NodeIndex, Generation);

pub struct SmallGraph<T> {
    pub(crate) free: SmallVec<[NodeHandle; 128]>,
    pub(crate) nodes: SmallVec<[(Generation, Option<T>); 128]>,
    pub(crate) connections: SmallVec<[(NodeIndex, NodeIndex); 256]>,
}

impl<T> SmallGraph<T> {
    /// Create a new SmallGraph
    pub fn new() -> SmallGraph<T> {
        SmallGraph {
            free: SmallVec::new(),
            nodes: SmallVec::new(),
            connections: SmallVec::new(),
        }
    }

    /// Insert a value into graph
    pub fn insert(&mut self, value: T) -> NodeHandle {
        if self.free.len() == 0 {
            let index = self.nodes.len();
            let gen = 0;
            self.nodes.push((gen, Some(value)));
            (index, gen)
        } else {
            let n = self.free.remove(0);
            let (index, gen) = n;
            self.nodes[index] = (gen + 1, Some(value));
            (n.0, gen + 1)
        }
    }

    /// Create a directed connection between parent and child
    pub fn connect_to(&mut self, parent: NodeHandle, child: NodeHandle) {
        self.connections.push((parent.0, child.0));
    }

    /// Get nodes this node has an edge to
    pub fn neighbors(&self, node: NodeHandle) -> SmallVec<[NodeHandle; 8]> {
        let mut children = SmallVec::new();
        for i in 0..self.connections.len() {
            if self.connections[i].0 == node.0 {
                let child_handle = (
                    self.connections[i].1,
                    self.nodes[self.connections[i].1].0,
                );
                children.push(child_handle);
            }
        }
        children
    }

    /// Get nodes that have an edge that can reach node
    pub fn nodes_with_neighbor(&self, node: NodeHandle) -> SmallVec<[NodeHandle; 8]> {
        let mut children = SmallVec::new();
        for i in 0..self.connections.len() {
            if self.connections[i].1 == node.0 {
                let child_handle = (
                    self.connections[i].1,
                    self.nodes[self.connections[i].1].0,
                );
                children.push(child_handle);
            }
        }
        children
    }

    /// Create a two way connection between two nodes
    pub fn connect(&mut self, a: NodeHandle, b: NodeHandle) {
        self.connections.push((a.0, b.0));
        self.connections.push((b.0, a.0));
    }

    /// Disconnect all connections a node has
    pub fn disconnect_all(&mut self, n: NodeHandle) {
        self.connections
            .retain(|&mut connection| (connection).0 != n.0 && (connection).1 != n.0);
    }

    /// Disconnect all connections between two nodes
    pub fn disconnect(&mut self, a: NodeHandle, b: NodeHandle) {
        self.connections.retain(|&mut connection| {
            !((connection.0 == a.0 && connection.1 == b.1)
                && (connection.0 == b.0 && connection.1 == a.1))
        });
    }

    /// Disconnect edge connection between source and destination
    pub fn disconnect_from(&mut self, source: NodeHandle, destination: NodeHandle) {
        self.connections
            .retain(|&mut connection| !(connection.0 == source.0 && connection.1 == destination.1));
    }

    /// Determine if there is a connection connection between a source and destination node
    pub fn is_connected_to(&mut self, source: NodeHandle, destination: NodeHandle) -> bool {
        self.connections
            .iter()
            .find(|&connection| connection.0 == source.0 && connection.1 == destination.0)
            .is_some()
    }

    /// Remove a node and it's connections from graph
    pub fn remove(&mut self, n: NodeHandle) -> Option<T> {
        let (index, gen) = n;
        if self.nodes[index].0 == gen {
            self.disconnect_all(n);
            let mut r = (gen + 1, None);
            core::mem::swap(&mut self.nodes[index], &mut r);
            self.free.push(n);
            return Some(r.1.unwrap());
        }
        None
    }

    /// Returns the count of nodes
    pub fn node_count(&self) -> usize {
        let mut c = 0;
        for i in 0..self.nodes.len() {
            if self.nodes[i].1.is_some() {
                c += 1;
            }
        }
        c
    }

    /// Get the value of a node
    pub fn get(&self, n: NodeHandle) -> Option<&T> {
        let (index, gen) = n;
        if self.nodes[index].0 == gen {
            if let Some(value) = &self.nodes[index].1 {
                return Some(value);
            }
        }
        None
    }

    /// Get a mutable value of a node
    pub fn get_mut(&mut self, n: NodeHandle) -> Option<&mut T> {
        let (index, gen) = n;
        if self.nodes[index].0 == gen {
            if let Some(value) = &mut self.nodes[index].1 {
                return Some(value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo {
        v: u8,
    }

    #[test]
    fn test_basic_0() {
        let g = SmallGraph::<Foo>::new();
        assert_eq!(0, g.nodes.len());
        assert_eq!(0, g.free.len());
        assert_eq!(0, g.connections.len());
    }

    #[test]
    fn test_basic_1() {
        let mut g = SmallGraph::<Foo>::new();
        let f1 = g.insert(Foo { v: 24 });
        let f2 = g.insert(Foo { v: 42 });
        assert_eq!(2, g.nodes.len());
        assert_eq!(0, f1.0);
        assert_eq!(0, f1.1);
        assert_eq!(1, f2.0);
        assert_eq!(0, f2.1);
    }

    #[test]
    fn test_basic_2() {
        let mut g = SmallGraph::<Foo>::new();
        let f1 = g.insert(Foo { v: 42 });
        let r = g.remove(f1).expect("could not remove");
        assert_eq!(42, r.v);
        let f2 = g.insert(Foo { v: 55 });
        assert_eq!(0, f2.0);
        assert_eq!(1, f2.1);
        assert_eq!(None, g.get(f1));
        assert_eq!(55, g.get(f2).unwrap().v);
    }

    #[test]
    fn test_basic_3() {
        let mut g = SmallGraph::<Foo>::new();
        let f1 = g.insert(Foo { v: 24 });
        let f2 = g.insert(Foo { v: 42 });
        g.connect(f1, f2);
        assert_eq!(2, g.connections.len());
        assert_eq!((0, 1), g.connections[0]);
        assert_eq!((1, 0), g.connections[1]);
    }

    #[test]
    fn test_basic_4() {
        let mut g = SmallGraph::<Foo>::new();
        let f1 = g.insert(Foo { v: 24 });
        let f2 = g.insert(Foo { v: 42 });
        let f3 = g.insert(Foo { v: 33 });
        g.connect_to(f1, f2);
        g.connect_to(f2, f3);
        g.connect_to(f1, f3);
        assert_eq!(3, g.connections.len());
        assert_eq!((0, 1), g.connections[0]);
        assert_eq!((1, 2), g.connections[1]);
        g.remove(f2);
        assert_eq!(1, g.connections.len());
        assert_eq!(true, g.is_connected_to(f1, f3));
        assert_eq!(false, g.is_connected_to(f1, f2));
        assert_eq!(false, g.is_connected_to(f2, f3));
    }

    #[test]
    fn test_basic_5() {
        let mut g = SmallGraph::<Foo>::new();
        g.insert(Foo { v: 24 });
        g.insert(Foo { v: 42 });
        let f3 = g.insert(Foo { v: 33 });
        assert_eq!(3, g.node_count());
        g.remove(f3);
        assert_eq!(2, g.node_count());
    }
}
