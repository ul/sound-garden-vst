//! # Audio graph
use crate::module::Module;
use crate::sample::{Frame, Sample};
use fixedbitset::FixedBitSet;
use petgraph::algo::{toposort, DfsSpace};
use petgraph::prelude::*;

pub type Node = Box<Module + Send>;

/// Structure which manages network of Modules.
pub struct AudioGraph {
    channels: usize,
    /// Nodes are boxed Modules and edges represent source->sink connections.
    graph: StableGraph<Node, ()>,
    /// `sample` writes output from source nodes into this buffer and then passes it to sink.
    /// Buffer is reused during graph traversal and between samples to avoid memory allocations.
    input: Vec<Sample>,
    /// `sample` walks graph in topological order which is cached here.
    order: Vec<NodeIndex>,
    space: DfsSpace<NodeIndex, FixedBitSet>,
}

/// Maximum number of sources to connect to sink.
/// This number is required because input buffer is allocated during AudioGraph initialization and
/// then re-used across all nodes sampling. There is no real need to have it hardcoded though.
/// It can be made an argument of AudioGraph::new.
const MAX_SOURCES: usize = 16;

impl AudioGraph {
    pub fn new(channels: usize) -> Self {
        let graph = StableGraph::default();
        let space = DfsSpace::new(&graph);
        let input_len = channels * MAX_SOURCES;
        AudioGraph {
            channels,
            graph,
            input: vec![0.0; input_len],
            order: Vec::new(),
            space,
        }
    }

    /// Compute and return the next frame of AudioGraph's sound stream.
    /// Frame slice contains one sample for each channel.
    pub fn sample(&mut self, input: &Frame) -> &Frame {
        let channels = self.channels;
        for idx in &self.order {
            let idx = *idx;
            let g = &mut self.graph;
            if g[idx].inputs() > 0 {
                // NOTE neighbors_directed walks edges starting from the most recently added (is it
                // guaranteed?). This is the reason why connection methods (connect, set_sources,
                // chain etc.) call clear_sources first and reverse sources. Always resetting
                // sources instead of finer-grained manipulation reduces risk of confusing their
                // order. We might want to consider to name edges and pass HashMap instead instead
                // of Vec as input. But it implies non-neglegible performance hit.
                //
                // Ref `Module::sample` doc for an example of input layout.
                for (i, source) in g.neighbors_directed(idx, Incoming).enumerate() {
                    let offset = i * channels;
                    self.input[offset..(offset + channels)].clone_from_slice(g[source].output());
                }
            } else {
                // If node does not have any inputs it might be waiting for external input.
                // Yes, black magic.
                self.input[..channels].clone_from_slice(input);
            }
            g[idx].sample(&self.input);
        }
        if self.order.is_empty() {
            // This might be a garbage, we just don't care what to return from empty graph
            // and don't want to allocate any extra resources for such case.
            &self.input
        } else {
            &self.graph[self.order[self.order.len() - 1]].output()
        }
    }

    pub fn node(&self, idx: NodeIndex) -> &Node {
        &self.graph[idx]
    }

    /// Add node to the graph and return index assigned to the node.
    /// This index is stable and could be used to reference the node when building connections.
    pub fn add_node(&mut self, n: Node) -> NodeIndex {
        self.graph.add_node(n)
    }

    /// Connect nodes in a chain, from left to right.
    /// It clears nodes' sources before connecting, except for the first one.
    pub fn chain(&mut self, nodes: &[NodeIndex]) {
        for i in 0..(nodes.len() - 1) {
            self.clear_sources(nodes[i + 1]);
            self.graph.update_edge(nodes[i], nodes[i + 1], ());
        }
        self.update_order();
    }

    /// Set node `a` as a single source of node `b`.
    /// It clears `b`'s sources before connecting, to set multiple sources use `set_sources`.
    pub fn connect(&mut self, a: NodeIndex, b: NodeIndex) {
        self.clear_sources(b);
        self.graph.update_edge(a, b, ());
        self.update_order();
    }

    /// Set multiple sources for the `sink` node.
    /// It clears `sink`'s sources before connecting.
    /// `source`s' outputs are layouted in `sink` input buffer in the provided order.
    /// Ref `Module::sample` doc for an example of input layout.
    pub fn set_sources(&mut self, sink: NodeIndex, sources: &[NodeIndex]) {
        self.clear_sources(sink);
        // ref `sample` method comments for the reason of reversing sources
        for source in sources.iter().rev() {
            self.graph.update_edge(*source, sink, ());
        }
        self.update_order();
    }

    pub fn set_sources_rev(&mut self, sink: NodeIndex, sources: &[NodeIndex]) {
        self.clear_sources(sink);
        // ref `sample` method comments for the reason of reversing sources
        for source in sources.iter() {
            self.graph.update_edge(*source, sink, ());
        }
        self.update_order();
    }

    pub fn clear(&mut self) {
        self.order.clear();
        self.graph.clear();
    }

    /// Update node traversal order.
    /// It must be called after any connection change.
    pub fn update_order(&mut self) {
        self.order = toposort(&self.graph, Some(&mut self.space)).unwrap_or_else(|_| vec![]);
    }

    /// Remove all incoming connections of the node.
    fn clear_sources(&mut self, sink: NodeIndex) {
        while let Some(edge) = self
            .graph
            .neighbors_directed(sink, Incoming)
            .detach()
            .next_edge(&self.graph)
        {
            self.graph.remove_edge(edge);
        }
    }
}
