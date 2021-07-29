pub mod data;
pub mod config;
pub mod error;

pub use error::{IchaError, IchaResult};
pub use config::Config;

pub fn init() -> IchaResult<()> {
    let config = Config::from_file()?;
    println!("{}", config.to_string());
    graph_test();
    Ok(())
}

use crate::data::graph::{Net, Node, Edge};

pub fn graph_test() {
    let mut graph = Net::<&'static str, &'static str>::new_directed();

    let chris = graph.add("Chris");
    let jazzy = graph.add("Jazzy");
    let baby = graph.add("Baby");
    let cat = graph.add("Cat");
    let man = graph.add("Man");

    let jazzy_baby = graph.add_edge(jazzy, baby, "loves");
    let baby_jazzy = graph.add_edge(baby, jazzy, "takes care of");
    let chris_baby = graph.add_edge(chris, baby, "loves");
    let jazzy_chris = graph.add_edge(jazzy, chris, "annoys");
    let cat_jazzy = graph.add_edge(cat, jazzy, "loves");
    let cat_baby = graph.add_edge(cat, baby, "annoys");
    let chris_cat = graph.add_edge(chris, cat, "eats");
    let man_cat = graph.add_edge(man, cat, "pets");
    let cat_man = graph.add_edge(cat, man, "meows");
    let man_jazzy = graph.add_edge(man, jazzy, "annoys");



    println!("R1: EDGES {:#?} NODES {:#?} exp: (10, 5)", 
        graph.edge_count(), graph.node_count());
    graph.edges_log();
    graph.nodes_log();
    graph.remove_edge(cat_man);

    println!("R2: EDGES {:#?} NODES {:#?} exp: (9, 5)", 
        graph.edge_count(), graph.node_count());
    graph.edges_log();
    graph.nodes_log();
    graph.remove(man);

    println!("R3: EDGES {:#?} NODES {:#?} exp: (7, 4)", 
        graph.edge_count(), graph.node_count());
    graph.edges_log();
    graph.nodes_log();
    graph.remove(chris);

    println!("R4: EDGES {:#?} NODES {:#?} exp: (4, 3)", 
        graph.edge_count(), graph.node_count());
    graph.edges_log();
    graph.nodes_log();


    println!("FIND BABY{}-JAZZy{} EDGE: {:?}", baby, jazzy, graph.get_edge(baby, jazzy));
    // println!("FIND BABY NODE: {:?}", graph.node(baby, jazzy));


    println!("{:#?}", graph);


}
