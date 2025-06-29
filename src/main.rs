use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use cargo_lock::{Dependency, Lockfile};
use clap::Parser;
use petgraph::Graph;
use petgraph::dot::{Config, Dot};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct CliArgs {
    #[clap(short, long, help = "path to Cargo.lock file")]
    pub file: Option<String>,

    #[clap(short, long, help = "path to output dot file")]
    pub output: Option<String>,
}

fn generate_dot(lockfile_path: &str, output_path: &str) {
    let lockfile = Lockfile::load(Path::new(lockfile_path)).expect("Can't load Cargo.lock");

    let mut graph = Graph::new();
    let mut nodes = BTreeMap::new();

    for package in &lockfile.packages {
        let node_index = graph.add_node(package.name.as_str());
        nodes.insert(Dependency::from(package), node_index);
    }

    for package in &lockfile.packages {
        let parent_index = nodes[&Dependency::from(package)];

        for dependency in &package.dependencies {
            if let Some(node_index) = nodes.get(dependency) {
                graph.add_edge(parent_index, *node_index, dependency.clone());
            }
        }
    }

    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    let dot_str = format!("{}", dot);

    fs::write(output_path, dot_str).expect("Can't dump result dot file");
}

fn main() {
    let args = CliArgs::parse();

    let lockfile_path = args.file.unwrap_or("Cargo.lock".to_string());
    let output_path = args.output.unwrap_or("Cargo.dot".to_string());
    generate_dot(&lockfile_path, &output_path);
}
