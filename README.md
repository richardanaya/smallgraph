# smallgraph

<a href="https://docs.rs/smallgraph"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

```toml
[dependencies]
executor = "0.0"
```

A graph implementation based on [`smallvec`](https://github.com/servo/rust-smallvec) and generational indexes to increase memory cache locality of node members.

* `#![no_std]`
* [`smalltree`](https://github.com/richardanaya/smalltree) implementation is built on `smallgraph`


# Example

```rust
struct Foo;

fn main(){
  let g = smallgraph::SmallGraph::new();
  let f1 = g.insert(Foo{});
  let f2 = g.insert(Foo{});
  g.connect(f1,f2);
}
```


# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `smallgraph` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
