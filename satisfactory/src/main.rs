use std::collections::HashMap;

use nalgebra::{DMatrix, DVector};
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Resource {
    IronOre,
    IronIngot,
    IronPlate,
    IronRod,
    Screw,
    ReinforcedIronPlate,
    // ... Add other resources
}

fn main() {
    // Create a directed graph
    let mut graph: DiGraph<(Resource, f64), f64> = DiGraph::new();

    // Add nodes (resources and their production rates)
    let iron_ore = graph.add_node((Resource::IronOre, 30.0));
    let iron_ingot = graph.add_node((Resource::IronIngot, 30.0));
    let iron_plate = graph.add_node((Resource::IronPlate, 20.0));
    let iron_rode = graph.add_node((Resource::IronRod, 15.0));
    let screw = graph.add_node((Resource::Screw, 40.0));
    let reinforced_iron_plate = graph.add_node((Resource::ReinforcedIronPlate, 5.0));

    // Add edges (dependencies between resources and their input rates)
    graph.add_edge(iron_ore, iron_ingot, 30.0);
    graph.add_edge(iron_ingot, iron_plate, 30.0);
    graph.add_edge(iron_ingot, iron_rode, 15.0);
    graph.add_edge(iron_ingot, screw, 40.0);
    graph.add_edge(iron_plate, screw, 4.0);
    graph.add_edge(iron_plate, reinforced_iron_plate, 30.0);
    graph.add_edge(screw, reinforced_iron_plate, 60.0);

    // Define your target production rates
    let target_rates = HashMap::from([(Resource::ReinforcedIronPlate, 5.0)]);

    // Visualize the graph (optional)
    println!("{:?}", Dot::with_config(&graph, &[]));

    calculate_factory_requirements(&graph, &target_rates);
}

fn calculate_factory_requirements(
    graph: &DiGraph<(Resource, f64), f64>,
    target_rates: &HashMap<Resource, f64>,
) {
    // Determine the size of the matrix
    let size = graph.node_count();

    // Create matrices for the system of equations
    let mut a_matrix = DMatrix::zeros(size, size); // Coefficients matrix
    let mut b_vector = DVector::zeros(size); // Constants vector (production goals)

    // Map to keep track of node indices
    let mut node_indices = HashMap::new();
    for node_index in graph.node_indices() {
        let resource = graph[node_index].0;
        let index = node_index.index();
        node_indices.insert(resource, index);

        // Fill the b_vector with the target production rates
        if let Some(&rate) = target_rates.get(&resource) {
            b_vector[index] = rate;
        }

        a_matrix[(index, index)] = graph[node_index].1;
    }

    // Fill the matrix and vector based on the production data
    for edge_ref in graph.edge_references() {
        let (source, target) = (edge_ref.source(), edge_ref.target());
        let source_index = node_indices[&graph[source].0];
        let target_index = node_indices[&graph[target].0];

        // Existing logic...
        a_matrix[(source_index, target_index)] = -edge_ref.weight();
    }

    // Solve the system of equations
    let x_vector = solve_system(a_matrix, &b_vector);

    // Output the number of factories needed for each resource
    for node_index in graph.node_indices() {
        let index = node_index.index();
        let resource = graph[node_index].0;
        println!("Resource: {:?}, Factories: {}", resource, x_vector[index]);
    }
}

fn solve_system(a_matrix: DMatrix<f64>, b_vector: &DVector<f64>) -> DVector<f64> {
    println!("A Matrix:\n{a_matrix}");
    println!("B Vector:\n{b_vector}");

    // Use nalgebra or another linear algebra library to solve the system
    a_matrix
        .lu()
        .solve(b_vector)
        .expect("Cannot solve the system.")
}
