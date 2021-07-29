//! A rewrite of the petgraph impl_graph module
//! for practice and personal graph data structure needs
//! for other components of the Idle Chain
//!
use std::mem::size_of;
use std::{iter, slice, vec, ops, fmt, hash};

pub type NodeIx = usize;
pub type EdgeIx = usize;

#[derive( Clone)]
pub struct Node<N: Clone> {
    pub weight: N,
    edges: EdgeLink,
}

#[derive(Clone)]
pub struct Edge<E: Clone> {
    pub weight: E,
    node: NodeLink,
    next: EdgeLink,
}
#[derive(Clone)]
pub struct EdgeLink{
    outgoing: EdgeIx, 
    incoming: EdgeIx
}
#[derive(Clone)]
pub struct NodeLink {
    src: NodeIx,
    dest: NodeIx,
}
impl Default for NodeLink {
    fn default() -> Self {
        Self {
            src: EdgeIx::max_value(),
            dest: EdgeIx::max_value()
        }
    }
}
impl Default for EdgeLink {
    fn default() -> Self {
        Self {
            outgoing: EdgeIx::max_value(),
            incoming: EdgeIx::max_value()
        }
    }
}
impl EdgeLink {
    fn new(out: EdgeIx, inc: EdgeIx) -> Self {
        Self { outgoing: out, incoming: inc }
    }
    fn out(&self) -> EdgeIx { self.outgoing }
    fn inc(&self) -> EdgeIx { self.incoming }
    fn set_incoming(&mut self, inc: EdgeIx) {
        self.incoming = inc;
    }
    fn set_outgoing(&mut self, outgoing: EdgeIx) {
        self.outgoing = outgoing;
    }
    fn set_both(&mut self, src: EdgeIx) {
        self.incoming = src;
        self.outgoing = src;
    }
    fn reset_both(&mut self) {
        self.incoming = EdgeIx::max_value();
        self.outgoing = EdgeIx::max_value();
    }
    fn next(&self, dir: &Direction) -> NodeIx {
        match dir {
            &Direction::Incoming => self.incoming,
            &Direction::Outgoing => self.outgoing,
        }
    }
    fn next_mut(&mut self, dir: &Direction) -> &mut NodeIx {
        match dir {
            &Direction::Incoming => &mut self.incoming,
            &Direction::Outgoing => &mut self.outgoing,
        }
    }
}
impl NodeLink {
    fn new(src: NodeIx, dest: NodeIx) -> Self {
        Self { src, dest }
    }
    fn src(&self) -> NodeIx { self.src }
    fn dest(&self) -> NodeIx { self.dest }
    fn set_src(&mut self, src: NodeIx) {
        self.src = src;
    }
    fn set_dest(&mut self, dest: NodeIx) {
        self.dest = dest;
    }
    fn set_both(&mut self, src: NodeIx) {
        self.src = src;
        self.dest = src;
    }
    fn reset_both(&mut self) {
        self.src = NodeIx::max_value();
        self.dest = NodeIx::max_value();
    }

    fn next(&self, dir: &Direction) -> NodeIx {
        match dir {
            &Direction::Incoming => self.dest,
            &Direction::Outgoing => self.src,
        }
    }
    fn next_mut(&mut self, dir: &Direction) -> &mut NodeIx {
        match dir {
            &Direction::Incoming => &mut self.dest,
            &Direction::Outgoing => &mut self.src,
        }
    }
}

impl<N> Node<N> 
where
    N: Clone 
{
    #[inline]
    pub fn init(weight: N) -> Self {
        Self {  weight, edges: EdgeLink::default() }
    }
    #[inline]
    pub fn new(weight: N, outgoing: EdgeIx, incoming: EdgeIx) -> Self {
        Self { weight, edges:  EdgeLink::new(outgoing, incoming) }
    }
}
impl<N> ops::Deref for Node<N>
where
    N: Clone 
{
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.weight
    }
}

impl<E> Edge<E>
where
    E: Clone 
{
    #[inline]
    pub fn init(weight: E, a: NodeIx, b: NodeIx) -> Self {
        Self { 
            weight,
            node: NodeLink::new(a, b),
            next: EdgeLink::default(),
        }
    }
    #[inline]
    pub fn new(weight: E, 
               a: NodeIx, 
               b: NodeIx, 
               next_outgoing: EdgeIx,
               next_incoming: EdgeIx) -> Self  { 
        Self { 
            weight,
            node: NodeLink::new(a, b),
            next: EdgeLink::new(next_outgoing, next_incoming),
        }
    }
    #[inline]
    pub fn a(self) -> NodeIx { self.node.src }
    #[inline]
    pub fn b(self) -> NodeIx { self.node.dest }
    #[inline]
    pub fn node(&self, dir: &Direction) -> NodeIx {
        match dir {
            Direction::Outgoing => self.node.src(),
            Direction::Incoming => self.node.dest(),
        }
    }
}
pub trait Linked {
    fn next_in(&self) -> EdgeIx;
    fn next_in_mut(&mut self) -> &mut EdgeIx;
    fn next_out(&self) -> EdgeIx;
    fn next_out_mut(&mut self) -> &mut EdgeIx;

    fn next(&self, dir: &Direction) -> EdgeIx {
        match dir {
            Direction::Incoming => self.next_in(),
            Direction::Outgoing => self.next_out(),
        }
    }
    fn next_mut(&mut self, dir: &Direction) -> &mut EdgeIx {
        match dir {
            Direction::Incoming => self.next_in_mut(),
            Direction::Outgoing => self.next_out_mut()
        }
    }
}
impl<E> Linked for Edge<E> 
where
    E: Clone 
{
    fn next_out(&self) -> EdgeIx { self.next.outgoing }
    fn next_in(&self) -> EdgeIx { self.next.incoming }
    fn next_out_mut(&mut self) -> &mut EdgeIx { &mut self.next.outgoing }
    fn next_in_mut(&mut self) -> &mut EdgeIx { &mut self.next.incoming }
}
impl<N> Linked for Node<N> 
where
    N: Clone
{
    fn next_out(&self) -> EdgeIx { self.edges.outgoing }
    fn next_in(&self) -> EdgeIx { self.edges.incoming }
    fn next_out_mut(&mut self) -> &mut EdgeIx { &mut self.edges.outgoing }
    fn next_in_mut(&mut self) -> &mut EdgeIx { &mut self.edges.incoming }
}
impl<E> ops::Deref for Edge<E> 
where
    E: Clone
{
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.weight 
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction { 
    Outgoing = 0 ,
    Incoming = 1, 
}
impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::Outgoing => 0,
            Direction::Incoming => 1,
        }
    }
}
impl IntoIterator for Direction {
    type Item = Direction;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Outgoing => vec![Self::Outgoing, Self::Incoming].into_iter(),
            Self::Incoming => vec![Self::Incoming].into_iter()
        }
    }
}
impl Direction {

    pub fn iter() -> Direction {
        Direction::Outgoing
    }
}
pub struct Net<N, E>
where
    N: Clone,
    E: Clone, 
{
    edges: Vec<Edge<E>>,
    nodes: Vec<Node<N>>,
    directed: bool,
}
impl<N, E> Default for Net<N, E> 
where
    N: Clone,
    E: Clone
{
    fn default() -> Self {
        Self {
            directed: true,
            edges: Vec::new(),
            nodes: Vec::new()
        }
    }
}

impl<N, E> Net<N, E>
where
    N: Clone + fmt::Debug,
    E: Clone + fmt::Debug 
{

    pub fn new_directed() -> Self {
        Self { directed: true, ..Default::default() }
    }

    pub fn new_undirected() -> Self {
        Self { directed: false, ..Default::default() }
    }
    pub fn add(&mut self, weight: N) -> NodeIx {
        let node = Node::init(weight);
        let node_ix = self.nodes.len() as NodeIx;
        self.nodes.push(node);
        node_ix
    }
    pub fn edge_count(&self) -> usize { self.edges.len() }
    pub fn node_count(&self) -> usize { self.nodes.len() }

    pub fn add_edge(&mut self, a: NodeIx, b: NodeIx, weight: E) -> EdgeIx {
        let edge_ix = self.edges.len();
        let mut edge = Edge::init(weight, a, b);
        match index_twice(&mut self.nodes, a, b) {
            Pair::None => panic!("Graph::add_edge: OOB"),
            Pair::One(an) => {
                let anc = an.clone();
                edge.next = anc.edges;
                an.edges.set_both(edge_ix);
            }
            Pair::Both(an, bn) => {
                edge.next.set_outgoing(an.edges.outgoing);
                edge.next.set_incoming(bn.edges.incoming);
                an.edges.set_outgoing(edge_ix);
                bn.edges.set_incoming(edge_ix);
            }
        }
        self.edges.push(edge);
        return edge_ix;
    }

    pub fn weight(&self, a: NodeIx) -> Option<&N> {
        self.nodes.get(a).map(|n| &n.weight)
    }

    pub fn weight_mut(&mut self, a: NodeIx) -> Option<&mut N> {
        self.nodes.get_mut(a).map(|n| &mut n.weight)
    }

    pub fn edge_weight(&self, a: EdgeIx) -> Option<&E> {
        self.edges.get(a).map(|e| &e.weight)
    }
    pub fn edge_weight_mut(&mut self, a: EdgeIx) -> Option<&mut E> {
        self.edges.get_mut(a).map(|n| &mut n.weight)
    }
    pub fn edge_endpoints(&self, e: EdgeIx) -> Option<(NodeIx, NodeIx)> {
        match self.edges.get(e) {
            Some(e) => return Some((e.node.src(), e.node.dest())),
            _ => (),
        }
        None
    }

    pub fn remove(&mut self, a: NodeIx) -> Option<N> where N: fmt::Debug {
        let n = self.nodes.get(a)?;
        println!("REMOVING NODE {:?}", &n );
        // Remove all edges to/from this node
        for dir in Direction::iter() {
            println!("REMOVING {:?} NODES", &dir);
            loop {
                let next = self.nodes[a].next(&dir);
                println!("CHEKING {}", &next);
                if next == EdgeIx::max_value() { break;}
                let ret = self.remove_edge(next);
                println!("REMOVING CONN TO NODE IX {}: {}", a, next);
                let _ = ret;
            }
        }
        let node = self.nodes.swap_remove(a);
        let swap_edges = match self.nodes.get(a) {
            None => return Some(node.weight),
            Some(e) => &e.edges,
        };
        let old_ix = NodeIx::from(self.nodes.len());
        let new_ix = a;
        for dir in Direction::iter() { let mut edges = EdgesMut::new(&mut self.edges, swap_edges.next(&dir), dir);
            while let Some(curr) = edges.next_edge() {
                *curr.next_mut(&dir) = new_ix;
            }
        }
        Some(node.weight)
    }

    pub fn remove_edge(&mut self, eix: EdgeIx) -> Option<E> {
        let (e, e_node, e_next) = match self.edges.get_mut(eix) {
            None => return None,
            Some(e) => (
                e.clone(),
                NodeLink::new(e.node.src(), e.node.dest()),
                EdgeLink::new(e.next.out(), e.next.inc()),
            )
        };
        self._change_edge_links(e_node, e_next, eix);
        self._rm_edge_change_indices(eix)
    }

    /// For edge e with endpoints `edge_node`, replace
    /// links to it with links to `edge_next`
    fn _change_edge_links(&mut self, 
        e_node: NodeLink,
        e_edge: EdgeLink,
        eix: EdgeIx) 
    {
        for d in Direction::iter() {
            let node = match self.nodes.get_mut(e_node.next(&d)) {
                Some(n) => n,
                None => {
                    debug_assert!(false, "Edge endpoint
                        dir={:?} index={:?} not found",
                    &d, e_node.next(&d));
                    return;
                }
            };
            if node.next(&d) == eix {
                *node.next_mut(&d) = e_edge.next(&d);
            } else {
                let mut edges = EdgesMut::new(&mut self.edges, node.next(&d), d);
                while let Some(curr) = edges.next_edge() {
                    if curr.next(&d) == eix {
                        *curr.next_mut(&d) = e_edge.next(&d);
                        break;
                    }
                }

            }

        }
    }
    fn _rm_edge_change_indices(&mut self, eix: EdgeIx) -> Option<E>
    {
        let edge = self.edges.swap_remove(eix);
        let swap = match self.edges.get(eix) {
            None => return Some(edge.weight),
            Some(ed) => ed.clone().node,
        };
        let swapped_e = EdgeIx::from(self.edges.len());
        let e_edge = EdgeLink::new(eix, eix);
        self._change_edge_links(swap, e_edge, swapped_e);
        Some(edge.weight)
    }

    pub fn update_edge(&mut self, a: NodeIx, b: NodeIx, weight: E) -> EdgeIx {
        if let Some(ix) = self.get_edge(a, b) {
            if let Some(ed) = self.edge_weight_mut(ix) {
                *ed = weight;
                return ix;
            }
        }
        self.add_edge(a, b, weight)
        
    }
    pub fn get_edge(&self, a: NodeIx, b: NodeIx) -> Option<EdgeIx> where N: fmt::Debug {
        if let Some(n) = self.nodes.get(a) {
            println!("FOUND NODE {:?} LOOKING FOR {:?}", n, b);
            if !self.directed {
                if let Some((ix, _dir)) = self._edge_from_node_undir(n, b) {
                    return Some(ix);
                }
            } else {
                if let Some(ix) = self._edge_from_node_dir(n, b) {
                    return Some(ix);
                } 
            }
        }
        return None;
    }

    fn _edge_from_node_undir(&self, 
            n: &Node<N>, 
            b: NodeIx
        ) -> Option<(EdgeIx, Direction)> 
    {
        let (mut next_in, mut next_out) = (n.next_in(), n.next_out());
        let edges = &mut self.edges.clone();
        while let Some(e) = edges.get_mut(next_in) {
            if e.next_out() == b {
                return Some((next_in, Direction::Outgoing))
            }
            next_in = e.next_in();
        }
        while let Some(e) = edges.get_mut(next_out) {
            if e.next_in() == b {
                return Some((next_out, Direction::Incoming))
            }
            next_out = e.next_out();
        }
        return None;
    }

    fn _edge_from_node_dir(&self, 
            n: &Node<N>, 
            b: NodeIx
        ) -> Option<EdgeIx>
    {
        let mut eix = n.next(&Direction::Outgoing);
        while let Some(edge) = self.edges.get(eix) {
            if edge.node(&Direction::Incoming) == b {
                return Some(eix);
            }
            eix = edge.next(&Direction::Outgoing);
        }
        None
    }
    
    pub fn neighbors(&self, a: NodeIx) -> Neighbors<E> {
        self.neighbors_directed(a, Direction::Outgoing)
    }

    pub fn neighbors_directed(&self, a: NodeIx, dir: Direction) -> Neighbors<E> {
        let mut iter = self.neighbors_undirected(a);
        if self.directed {
            match dir {
                Direction::Outgoing => iter.next.set_incoming(EdgeIx::max_value()),
                Direction::Incoming => iter.next.set_outgoing(EdgeIx::max_value())
            }
            iter.src = NodeIx::max_value();
        }
        return iter;
    }

    pub fn neighbors_undirected(&self, a: NodeIx) -> Neighbors<E> {
        match self.nodes.get(a) {
            None => Neighbors::init(a, &self.edges),
            Some(n) => Neighbors::new(
                a, &self.edges, n.clone().edges)
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    pub fn clear_edges(&mut self) {
        self.edges.clear();
        for node in &mut self.nodes {
            node.edges = EdgeLink::default();
        }
    }

    pub fn capacity(&self) -> (usize, usize) {
        (self.nodes.capacity(), self.edges.capacity())
    }

    pub fn edges_log(&self) -> () where E: fmt::Debug {
        let mut out = String::from("");
        for e in &self.edges {
        let (outg, inco) = match (&e.next.out(), &e.next.inc()) {
            (&EdgeIx::MAX, &EdgeIx::MAX) => ("NONE".to_string(), "NONE".to_string()),
            (&EdgeIx::MAX, inc) => ("NONE".to_string(), inc.to_string()),
            (out, &EdgeIx::MAX) => (out.to_string(), "NONE".to_string()),
            (out, inc) => (out.to_string(), inc.to_string()),
        };
            out.push_str(&format!("Edge ( Data: {:?}, Nodes: [{} -> {}], Next: [O: {}, I: {}] ) \n",
                    &e.weight, &e.node.src(), &e.node.dest(), outg, inco))
        }
        println!("{}", out);
    }

    pub fn nodes_log(&self) -> () where N: fmt::Debug {
        let mut out = String::from("");
        for n in &self.nodes {
            let (outg, inco) = match (&n.edges.out(), &n.edges.inc()) {
                (&EdgeIx::MAX, &EdgeIx::MAX) => ("NONE".to_string(), "NONE".to_string()),
                (&EdgeIx::MAX, inc) => ("NONE".to_string(), inc.to_string()),
                (out, &EdgeIx::MAX) => (out.to_string(), "NONE".to_string()),
                (out, inc) => (out.to_string(), inc.to_string()),
            };
            out.push_str(&format!("Node ( Data: {:?}, Next: [O: {}, I: {}] ) \n",
                    &n.weight, outg, inco))
        }
        println!("{}", out);
    }
    pub fn contains_edge(&self, a: NodeIx, b: NodeIx) -> bool {
        self.get_edge(a, b).is_some()
    }

    pub fn first_edge(&self, a: NodeIx, dir: Direction) -> Option<EdgeIx> {
        match self.nodes.get(a) {
            None => None,
            Some(n) => {
                let eix = n.next(&dir);
                if eix == EdgeIx::max_value() {
                    None
                } else { Some(eix) }
            }
        }
    }
    pub fn next_edge(&self, e: EdgeIx, dir: Direction) -> Option<EdgeIx> {
        match self.edges.get(e) {
            None => None,
            Some(n) => {
                let eix = n.next(&dir);
                if eix == EdgeIx::max_value() {
                    None
                } else { Some(eix) }
            }
        }
    }

    pub fn nodes(&self) -> &[Node<N>] {
        &self.nodes
    }
    pub fn edges(&self) -> &[Edge<E>] {
        &self.edges
    }

}

#[derive(Debug)]
pub struct Neighbors<'a, E: 'a> 
where
    E: Clone 
{
    src: NodeIx,
    edges: &'a [Edge<E>],
    next: EdgeLink,
}
impl <'a, E> Neighbors<'a, E> 
where
    E: Clone
{
    pub fn detach(&self) -> WalkNeighbors {
        WalkNeighbors {
            src: self.src,
            next: EdgeLink::default()
        }
    }

    pub fn init(src: NodeIx, edges: &'a [Edge<E>]) -> Self {
        Self { 
            src, edges,
            next: EdgeLink::default()
        }
    }

    pub fn new(
        src: NodeIx, 
        edges: &'a [Edge<E>], 
        next: EdgeLink) -> Self
    {
        Self { src, edges,  next }

    }

}
struct EdgesMut<'a, E: 'a + Clone> {
    edges: &'a mut [Edge<E>],
    next: EdgeIx,
    dir: Direction
}
impl<'a, E> EdgesMut<'a, E> 
where
    E: 'a + Clone
{
    fn next_edge(&mut self) -> Option<&mut Edge<E>> {
        self.next().map(|t| t.1)
    } 

    fn new(edges: &'a mut [Edge<E>], next: EdgeIx, dir: Direction) -> Self {
        Self { edges, next, dir}
    }

    fn next(&mut self) -> Option<(EdgeIx, &mut Edge<E>)> {
        let ix = self.next;
        match self.edges.get_mut(self.next) {
            Some(edge) => { 
                self.next = edge.next(&self.dir);
                return Some((ix, edge));
            }
            None => None,
        } 
    }
}

#[derive(Debug)]
pub struct Edges<'a, E: 'a> where E: Clone {
    src: NodeIx,
    edges: &'a [Edge<E>],
    next: EdgeLink,
    dir: Direction,
}
impl <'a, E: 'a> Iterator for Edges<'a, E> where E: Clone {
    type Item = EdgeRef<'a, E>;
    fn next(&mut self) -> Option<EdgeRef<'a, E>> {
        let i = self.next.next(&self.dir);
        if let Some(Edge { node, weight, next}) = self.edges.get(i) {
            *self.next.next_mut(&self.dir) = next.next(&self.dir);
            return Some(
                    EdgeRef {  ix: i, node: node.clone(), weight }
                );
        }
        return None;
    }
}
#[derive(Debug)]
pub struct WalkNeighbors {
    src: NodeIx,
    next: EdgeLink,
}
impl Clone for WalkNeighbors {
    fn clone(&self) -> Self {
        WalkNeighbors { src: self.src, next: self.next.clone() }
    }
}
#[derive(Debug)]
pub struct EdgeRef<'a, E: 'a> {
    ix: EdgeIx,
    node: NodeLink,
    weight: &'a E,
}

impl WalkNeighbors {

    pub fn next<N, E>(&mut self,
        g: &Net<N, E>) -> Option<(EdgeIx, NodeIx)>  
    where
        N: Clone, E: Clone
    {
        match g.edges.get(self.next.next(&Direction::Outgoing)) {
            None => {  },
            Some(e) => {
                let ed = self.next.next(&Direction::Outgoing);
                *self.next.next_mut(&Direction::Outgoing) = e.next(&Direction::Outgoing);
                return Some((ed, e.node.next(&Direction::Incoming)));
            }
        }
        while let Some(edge) = g.edges.get(self.next.inc()) {
            let ed = self.next.inc();
            self.next.set_incoming(edge.next_out());
            if edge.node(&Direction::Incoming) != self.src {
                return Some((ed, edge.node(&Direction::Outgoing)));
            }
        }
        None
    }
    pub fn next_node<N, E>(&mut self, g: &Net<N, E>)
        -> Option<NodeIx> 
    where
        N: Clone, E: Clone
    {
        self.next(g).map(|t| t.1)
    }

    pub fn next_edge<N, E>(&mut self, g: &Net<N, E>)
        -> Option<NodeIx> 
    where
        N: Clone, E: Clone
    {
        self.next(g).map(|t| t.0)
    }


    /* pub fn next<N, E>(&mut self, g: &Graph<N, E>) -> Option<(EdgeIx, NodeIx)> {
        match g.edges
    } */
}

/* impl<'a, E> Iterator for Neighbors<'a, E> {
    type Item = NodeIx;

    fn next(&mut self) -> Option<NodeIx> {
        match self.edges.get(&self.next_out) {
            Some(edge) => {
                self.next_outgoing = edge
            }

        }
        if let Some(ed) = self.edges.get(&self.next_out) {
            self.next_out = ed.Err

        }

    }
} */

enum Pair<T> {
    Both(T, T),
    One(T),
    None,
}

use std::cmp::max;

/// Get mutable references at index `a` and `b`.
fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Pair<&mut T> {
    if max(a, b) >= slc.len() {
        Pair::None
    } else if a == b {
        Pair::One(&mut slc[max(a, b)])
    } else {
        // safe because a, b are in bounds and distinct
        unsafe {
            let ptr = slc.as_mut_ptr();
            let ar = &mut *ptr.add(a);
            let br = &mut *ptr.add(b);
            Pair::Both(ar, br)
        }
    }
}

impl<N, E> fmt::Debug for Net<N, E> 
where
    N: fmt::Debug + Clone,
    E: fmt::Debug + Clone,

{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_struct = f.debug_struct("Net");
        fmt_struct.field(&"# nodes", &self.node_count());
        fmt_struct.field(&"# edges", &self.edge_count());
        if size_of::<N>() != 0 && self.edge_count() > 0 {
            fmt_struct.field("Nodes", &self.nodes );
        }
        if size_of::<N>() != 0 && self.node_count() > 0 {
            fmt_struct.field("Edges", &self.edges );
        }
        fmt_struct.finish()
    }
}
impl<N> fmt::Debug for Node<N> 
where
    N: fmt::Debug + Clone 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut fmt_struct = f.debug_struct("Node");
        fmt_struct.field("weight", &self.weight);
        fmt_struct.field("edge", &self.edges);
        fmt_struct.finish()
    }
}
impl<E> fmt::Debug for Edge<E> 
where
    E: fmt::Debug + Clone 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut fmt_struct = f.debug_struct("Edge");
        fmt_struct.field("weight", &self.weight);
        fmt_struct.field("node", &self.node);
        fmt_struct.field("edge", &self.next);
        fmt_struct.finish()
    }
}
impl fmt::Debug for EdgeLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (out, inc) = match (&self.outgoing, &self.incoming) {
            (&EdgeIx::MAX, &EdgeIx::MAX) => ("NONE".to_string(), "NONE".to_string()),
            (&EdgeIx::MAX, inc) => ("NONE".to_string(), inc.to_string()),
            (out, &EdgeIx::MAX) => (out.to_string(), "NONE".to_string()),
            (out, inc) => (out.to_string(), inc.to_string()),
        };
        f.write_fmt(format_args!("outgoing [{}] -->> incoming: [{}]",
                &out, &inc))
    }
}
impl fmt::Debug for NodeLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("src [{}] -->> dest: [{}]",
                &self.src, &self.dest))
    }
}
