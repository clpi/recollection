use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fmt::Debug;
use recollection::{
    RecolResult, RecolError,
    data::graph::{Direction, Node, Edge, Graph},
};

fn graph<N, E>(directed: bool) -> Graph<N, E> where
    N: Clone + Debug, E: Clone + Debug
{
    match directed {
        true => Graph::<N, E>::new_directed(),
        false => Graph::<N, E>::new_undirected()
    }
}

fn add_nodes(n: usize) {
    let mut g = graph::<usize, usize>(true);
    for i in 0..n {
        g.add(i);
    }
}

fn add_nodes_edges(nodes: usize) {
    let mut g = graph::<usize, usize>(true);
    let mut prev_nodes: Vec<usize> = vec![];
    for i in 0..nodes {
        let n = g.add(i);
        for (j, node) in (0..i).enumerate() {
            let _edge = g.add_edge(n, node, i+j);
        }
        prev_nodes.push(n);
    }
}


fn bench(c: &mut Criterion) {
    c.bench_function("graph_add_nodes 10", |b| b.iter(|| add_nodes(black_box(10))));
    c.bench_function("graph_add_nodes 20", |b| b.iter(|| add_nodes(black_box(20))));
    c.bench_function("graph_add_nodes 40", |b| b.iter(|| add_nodes(black_box(40))));

    c.bench_function("graph_add_nodes_edges 10", |b| b.iter(|| add_nodes_edges(black_box(10))));
    c.bench_function("graph_add_nodes_edges 20", |b| b.iter(|| add_nodes_edges(black_box(20))));
    c.bench_function("graph_add_nodes_edges 40", |b| b.iter(|| add_nodes_edges(black_box(40))));
}

criterion_group!(benches, bench);
criterion_main!(benches);
