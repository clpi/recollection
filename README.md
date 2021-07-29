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

```sh
cargo install recollection
```

## Usage

```rust
use recollection::Graph;

let g = Graph::<&'static str, usize>::new();

let p1 = g.add("Person 1");
let p2 = g.add("Person 2");

let p1_p2 = g.add_edge(p1, p2, 4 as usize);

g.remove_edge(p1_p2);
g.remove(p1);
```

## Author

👤 **Chris P**

* Website: https://clp.is
* Twitter: [@clp\_is](https://twitter.com/clp_is)
* Github: [@clpi](https://github.com/clpi)

