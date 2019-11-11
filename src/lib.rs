//#![no_std]
use smallvec::SmallVec;

pub type NodeIndex = usize;
pub type Generation = usize;
pub type NodeHandle = (NodeIndex, Generation);

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
            let (index,gen) = n;
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

    pub fn remove(&mut self, n: NodeHandle) -> Option<T> {
        let (index,gen) = n;
        if self.nodes[index].0 == gen {
            self.disconnect(n);
            let mut r = (gen + 1, None);
            core::mem::swap(&mut self.nodes[index], &mut r);
            self.free.push(n);
            return Some(r.1.unwrap());
        }
        None
    }

    pub fn get(&self, n: NodeHandle) -> Option<&T> {
        let (index,gen) = n;
        if self.nodes[index].0 == gen {
            if let Some(value) = &self.nodes[index].1 {
                return Some(value);
            }
        }
        None
    }

    pub fn get_mut(&mut self, n: NodeHandle) -> Option<&mut T> {
        let (index,gen) = n;
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

    #[derive(Debug,PartialEq)]
    struct Foo{
        v:u8
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
        let f1 = g.insert(Foo {v:24});
        let f2 = g.insert(Foo {v:42});
        assert_eq!(2, g.nodes.len());
        assert_eq!(0, f1.0);
        assert_eq!(0, f1.1);
        assert_eq!(1, f2.0);
        assert_eq!(0, f2.1);
    }

    #[test]
    fn test_basic_2() {
        let mut g = SmallGraph::<Foo>::new();
        let f1 = g.insert(Foo {v:42});
        let r = g.remove(f1).expect("could not remove");
        assert_eq!(42, r.v);
        let f2 = g.insert(Foo {v:55});
        assert_eq!(0, f2.0);
        assert_eq!(1, f2.1);
        assert_eq!(None, g.get(f1));
        assert_eq!(55, g.get(f2).unwrap().v);
    }

    #[test]
    fn test_basic_3() {
        let mut g = SmallGraph::<Foo>::new();
        let f1 = g.insert(Foo {v:24});
        let f2 = g.insert(Foo {v:42});
        g.connect(f1,f2);
        assert_eq!(2, g.connections.len());
        assert_eq!((0,1), g.connections[0]);
        assert_eq!((1,0), g.connections[1]);
    }

    #[test]
    fn test_basic_4() {
        let mut g = SmallGraph::<Foo>::new();
        let f1 = g.insert(Foo {v:24});
        let f2 = g.insert(Foo {v:42});
        let f3 = g.insert(Foo {v:33});
        g.directed_connect(f1,f2);
        g.directed_connect(f2,f3);
        assert_eq!(2, g.connections.len());
        assert_eq!((0,1), g.connections[0]);
        assert_eq!((1,2), g.connections[1]);
        g.remove(f2);
        assert_eq!(0, g.connections.len());
    }
}
