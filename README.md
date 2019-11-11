# smallgraph

A graph implementation based on [`smallvec`](https://github.com/servo/rust-smallvec) and generational indexes to increase memory cache locality of node members.

* `#![no_std]`
* [`smalltree`](https://github.com/richardanaya/smalltree) implementation is built on `smallgraph`


# Example

```rust
struct Foo{}

let g = SmallGraph::new();
let f1 = g.insert(Foo{});
let f2 = g.insert(Foo{});
g.connect(f1,f2);
```
