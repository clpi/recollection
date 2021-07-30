use std::fmt::Debug;
use recollection::prelude::*;

fn graph<N, E>(directed: bool) -> Graph<N, E> where
    N: Clone + Debug, E: Clone + Debug {
    match directed {
        true => Graph::<N, E>::new_directed(),
        false => Graph::<N, E>::new_undirected()
    }
}
fn sgraph(dir: bool) -> Graph<&'static str, &'static str> {
    graph::<&'static str, &'static str>(dir)
}
fn usgraph(dir: bool) -> Graph<usize, usize> {
    graph::<usize, usize>(dir)
}

#[test]
fn graph_adds_nodes() -> RecolResult<()> {
    let mut g = usgraph(true);
    for i in 0..20 {
        g.add(i);
    }
    assert_eq!(g.node_count(), 20);
    g.clear();
    assert_eq!(g.node_count(), 0);
    Ok(())
}

#[test]
fn graph_adds_nodes_and_edges() -> RecolResult<()> {
    let mut g = usgraph(true);
    for i in 0..20 {
        let n = g.add(i);
        for (j, node) in (0..i).enumerate() {
            let _edge = g.add_edge(n, node, i+j);
        }
        assert_eq!(g.node_count(), i+1);
        // assert_eq!(g.edge_count(), i*i);
    }
    assert_eq!(g.node_count(), 20);
    Ok(())
}

#[test]
fn graph_insert_remove() -> RecolResult<()> {
    let mut graph = sgraph(true);

    let chris = graph.add("Chris");
    let jazzy = graph.add("Jazzy");
    let baby = graph.add("Baby");
    let cat = graph.add("Cat");
    let man = graph.add("Man");
    println!("NODES: {} PRED {}", graph.node_count(), 5);
    assert_eq!(5, graph.node_count());

    let _jazzy_baby = graph.add_edge(jazzy, baby, "loves");
    let _baby_jazzy = graph.add_edge(baby, jazzy, "takes care of");
    let _chris_baby = graph.add_edge(chris, baby, "loves");
    let _jazzy_chris = graph.add_edge(jazzy, chris, "annoys");
    let _cat_jazzy = graph.add_edge(cat, jazzy, "loves");
    let _cat_baby = graph.add_edge(cat, baby, "annoys");
    let _chris_cat = graph.add_edge(chris, cat, "eats");
    let _man_cat = graph.add_edge(man, cat, "pets");
    let cat_man = graph.add_edge(cat, man, "meows");
    let _man_jazzy = graph.add_edge(man, jazzy, "annoys");
    println!("EDGES: {} PRED {}", graph.edge_count(), 10);
    assert_eq!(10, graph.edge_count());

    graph.remove_edge(cat_man);
    println!("EDGES: {} PRED {}", graph.edge_count(), 9);
    assert_eq!(9, graph.edge_count());

    graph.remove(man);
    println!("EDGES: {} PRED {}", graph.edge_count(), 7);
    println!("NODES: {} PRED {}", graph.node_count(), 4);
    assert_eq!(7, graph.edge_count());
    assert_eq!(4, graph.node_count());

    graph.remove(chris);
    println!("EDGES: {} PRED {}", graph.node_count(), 3);
    println!("NODES: {} PRED {}", graph.node_count(), 3);
    
    graph.clear_edges();
    println!("EDGES: {} PRED {}", graph.edge_count(), 0);
    println!("NODES: {} PRED {}", graph.node_count(), 3);
    assert_eq!(0, graph.edge_count());

    graph.clear();
    println!("EDGES: {} PRED {}", graph.edge_count(), 0);
    println!("NODES: {} PRED {}", graph.node_count(), 0);
    assert_eq!(0, graph.node_count());

    Ok(())
}
