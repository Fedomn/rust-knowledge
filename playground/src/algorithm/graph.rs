#[cfg(test)]
mod graph_test {
    use petgraph::algo::dijkstra;
    use petgraph::graph::{NodeIndex, UnGraph};
    use petgraph::{stable_graph::StableDiGraph, Direction};

    #[test]
    fn test_shortest() {
        // Create an undirected graph with `i32` nodes and edges with `()` associated data.
        let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        // Find the shortest path from `1` to `4` using `1` as the cost for every edge.
        let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
        assert_eq!(&1i32, node_map.get(&NodeIndex::new(4)).unwrap());
    }

    #[test]
    fn test_graph_remove_node() {
        let mut g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        let children: Vec<_> = g
            .neighbors_directed(1.into(), Direction::Outgoing)
            .collect();
        assert_eq!(children, vec![4.into(), 2.into()]);

        g.remove_node(2.into());

        let children: Vec<_> = g
            .neighbors_directed(1.into(), Direction::Outgoing)
            .collect();
        // The graph will invalidate unrelated node or edge indices when items are removed.
        assert_eq!(children, vec![2.into()]);
    }

    #[test]
    fn test_stable_graph_remove_node() {
        let mut g = StableDiGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

        let children: Vec<_> = g
            .neighbors_directed(1.into(), Direction::Outgoing)
            .collect();
        assert_eq!(children, vec![4.into(), 2.into()]);

        g.remove_node(2.into());

        let children: Vec<_> = g
            .neighbors_directed(1.into(), Direction::Outgoing)
            .collect();
        // The graph **does not invalidate** any unrelated node or edge indices when items are removed.
        assert_eq!(children, vec![4.into()]);
    }

    #[test]
    fn test_stable_graph_add_node() {
        let a = "a".to_string();
        let b = "b".to_string();
        let mut g = StableDiGraph::<String, (), usize>::default();

        let idx1 = g.add_node(a);
        assert_eq!(idx1, NodeIndex::new(0));

        let idx2 = g.add_node(b);
        assert_eq!(idx2, NodeIndex::new(1));

        g.add_edge(idx1, idx2, ());
        let children = g
            .neighbors_directed(idx1, Direction::Outgoing)
            .collect::<Vec<_>>();
        assert_eq!(children, vec![1.into()]);
    }
}
