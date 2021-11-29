use painter_depgraph::DepGraph;
use std::collections::HashMap;

fn main() {
    let mut d = DepGraph::default();

    let mut names = HashMap::new();

    names.insert(1, "Test".to_string());
    names.insert(2, "Asdf".to_string());
    names.insert(3, "Qwer".to_string());

    d.insert(1, vec![2, 3]);
    d.insert(2, vec![3]);
    d.insert(3, vec![]);

    println!(
        "{}",
        d.generate_dotgraph(&|x| format!("{}", names.get(x).unwrap()))
    )
}
