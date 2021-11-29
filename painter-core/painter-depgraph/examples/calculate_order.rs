use painter_depgraph::{compute_execution, debug_executor, DepGraph};

fn main() {
    let mut graph = DepGraph::default();

    graph.insert('A', vec!['B', 'G']);
    graph.insert('B', vec!['C', 'H']);
    graph.insert('C', vec!['D', 'F']);
    graph.insert('D', vec!['E']);
    graph.insert('E', vec![]);
    graph.insert('F', vec![]);
    graph.insert('G', vec!['F', 'M']);
    graph.insert('H', vec!['I']);
    graph.insert('I', vec!['J']);
    graph.insert('J', vec!['K', 'D']);
    graph.insert('K', vec!['L']);
    graph.insert('L', vec![]);
    graph.insert('M', vec!['N', 'P']);
    graph.insert('N', vec!['O']);
    graph.insert('O', vec!['P']);
    graph.insert('P', vec!['Q']);
    graph.insert('Q', vec![]);

    println!("{}", graph.generate_dotgraph(&|x| format!("{}", x)));

    let stages = compute_execution(&graph, vec!['A'], 10).expect("Calculating Order Failed");
    let stages_str: Vec<String> = stages.iter().map(|s| format!("{:?}", s)).collect();
    println!("{}", stages_str.join("\n"));
    debug_executor(stages, 10).expect("Execution Failed");
}
