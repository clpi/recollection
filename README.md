<h1 align="center">recollection</h1>
<p>
  <a href="https://www.npmjs.com/package/recollection" target="_blank">
    <img alt="Version" src="https://img.shields.io/npm/v/recollection.svg">
  </a>
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

> implementations and re-implementations of common data structures and algorithms in rust
> very much so a work in progress in its infant stage

### 🏠 [Homepage](https://clp.is/projects/recollection)

## Install
- Add to your `Cargo.toml` dependencies:

```toml
[dependencies]
recollection = "^0.1"
```

## Usage
- Example for graph (currently the only implemented, or part-implemented, data structure)
```rust
use recollection::Graph;

let g = Graph::<&'static str, usize>::new();

let p1 = g.add("Person 1");
let p2 = g.add("Person 2");

let p1_p2 = g.add_edge(p1, p2, 4 as usize);

g.remove_edge(p1_p2);
g.remove(p1);
```

## Further Info
- Crate can be found on [crates.io](crates.io/crates/recollection) or [lib.rs](lib.rs/crates/recollection).
- Documentation can be found on [docs.rs](https://docs.rs/recollection)


### Author
👤 **Chris P**

* Website: https://clp.is
* Twitter: [@clp\_is](https://twitter.com/clp_is)
* Github: [@clpi](https://github.com/clpi)

