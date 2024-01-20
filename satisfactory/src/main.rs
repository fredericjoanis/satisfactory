use std::collections::HashMap;

use nalgebra::{DMatrix, DVector};
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Resource {
    IronIngot,
    IronOre,
    IronPlate,
    IronRod,
    ModularFrame,
    ReinforcedIronPlate,
    Screw,
    Rotor,
    SmartPlating,
    CopperIngot,
    CopperSheet,
    Wire,
    Cable,
    Limestone,
    Concrete,
    CopperOre,
    SteelIngot,
    Coal,
    SteelBeam,
    SteelPipe,
    VersatileNetwork,
}

fn main() {
    // Define your target production rates here
    let target_rates = HashMap::from([(Resource::VersatileNetwork, 2.0)]);

    // Create a directed graph
    let mut graph: DiGraph<(Resource, f64), f64> = DiGraph::new();

    // Add nodes (resources and their production rates)
    let iron_ore = graph.add_node((Resource::IronOre, 30.0));
    let iron_ingot = graph.add_node((Resource::IronIngot, 30.0));
    let iron_plate = graph.add_node((Resource::IronPlate, 20.0));
    let iron_rod = graph.add_node((Resource::IronRod, 15.0));
    let screw = graph.add_node((Resource::Screw, 40.0));
    let reinforced_iron_plate = graph.add_node((Resource::ReinforcedIronPlate, 5.0));
    let modular_frame = graph.add_node((Resource::ModularFrame, 2.0));
    let rotor = graph.add_node((Resource::Rotor, 4.0));
    let smart_plating = graph.add_node((Resource::SmartPlating, 2.0));
    let copper_ingot = graph.add_node((Resource::CopperIngot, 30.0));
    let copper_sheet = graph.add_node((Resource::CopperSheet, 10.0));
    let wire = graph.add_node((Resource::Wire, 30.0));
    let cable = graph.add_node((Resource::Cable, 30.0));
    let limestone = graph.add_node((Resource::Limestone, 30.0)); // Validate
    let concrete = graph.add_node((Resource::Concrete, 30.0));
    let copper_ore = graph.add_node((Resource::CopperOre, 30.0)); // Validate
    let steel_ingot = graph.add_node((Resource::SteelIngot, 45.0));
    let coal = graph.add_node((Resource::Coal, 60.0)); // Validate
    let steel_beam = graph.add_node((Resource::SteelBeam, 15.0));
    let steel_pipe = graph.add_node((Resource::SteelPipe, 20.0));
    let versatile_framework = graph.add_node((Resource::VersatileNetwork, 2.0));

    // Add edges (dependencies between resources and their input rates)
    graph.add_edge(iron_plate, reinforced_iron_plate, 30.0);
    graph.add_edge(screw, reinforced_iron_plate, 60.0);
    graph.add_edge(reinforced_iron_plate, modular_frame, 3.0);
    graph.add_edge(iron_rod, modular_frame, 12.0);
    graph.add_edge(iron_rod, rotor, 20.0);
    graph.add_edge(screw, rotor, 100.0);
    graph.add_edge(reinforced_iron_plate, smart_plating, 2.0);
    graph.add_edge(rotor, smart_plating, 2.0);
    graph.add_edge(iron_ingot, iron_plate, 30.0);
    graph.add_edge(iron_ingot, iron_rod, 15.0);
    graph.add_edge(iron_rod, screw, 10.0);
    graph.add_edge(copper_ingot, copper_sheet, 20.0);
    graph.add_edge(copper_ingot, wire, 15.0);
    graph.add_edge(wire, cable, 60.0);
    graph.add_edge(limestone, concrete, 45.0);
    graph.add_edge(iron_ore, iron_ingot, 30.0);
    graph.add_edge(copper_ore, copper_ingot, 30.0);
    graph.add_edge(iron_ore, steel_ingot, 45.0);
    graph.add_edge(coal, steel_ingot, 45.0);
    graph.add_edge(steel_ingot, steel_beam, 60.0);
    graph.add_edge(steel_ingot, steel_pipe, 60.0);
    graph.add_edge(modular_frame, versatile_framework, 12.0);
    graph.add_edge(steel_beam, versatile_framework, 60.0);

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

        let number_of_factories = x_vector[index].ceil() as i64;

        if number_of_factories > 0 {
            println!("{resource:?}, Factories: {number_of_factories}");
        }
    }
}

#[allow(clippy::expect_used)]
fn solve_system(a_matrix: DMatrix<f64>, b_vector: &DVector<f64>) -> DVector<f64> {
    // Use nalgebra or another linear algebra library to solve the system
    a_matrix
        .lu()
        .solve(b_vector)
        .expect("Cannot solve the system.")
}
