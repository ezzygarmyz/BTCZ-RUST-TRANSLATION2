use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::net::{IpAddr, Ipv4Addr};
use std::collections::HashSet;

/// Reads node addresses from a file and parses them into a vector of valid IP addresses.
fn read_node_list(file_path: &str) -> Result<HashSet<IpAddr>, io::Error> {
    let mut nodes = HashSet::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            if let Ok(ip) = line.trim().parse::<IpAddr>() {
                nodes.insert(ip);
            }
        }
    }
    Ok(nodes)
}

/// Generates a Rust-compatible array of valid seed nodes from the given node list.
fn generate_seed_array(nodes: &HashSet<IpAddr>) -> String {
    let mut seed_array = String::from("const SEEDS: &[IpAddr] = &[\n");
    for node in nodes {
        seed_array.push_str(&format!("    IpAddr::V4(Ipv4Addr::new({}, {}, {}, {})),\n", 
            match node {
                IpAddr::V4(ipv4) => (ipv4.octets()[0], ipv4.octets()[1], ipv4.octets()[2], ipv4.octets()[3]),
                _ => (0, 0, 0, 0), // Fallback for non-IPv4 (should not occur in this setup)
            }
        ));
    }
    seed_array.push_str("];");
    seed_array
}

/// Helper function to read lines from a file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Main function to process node files and generate seed arrays.
fn main() -> Result<(), io::Error> {
    // Process mainnet nodes
    let mainnet_nodes = read_node_list("nodes_main.txt")?;
    let mainnet_seeds = generate_seed_array(&mainnet_nodes);
    std::fs::write("seeds_main.rs", mainnet_seeds)?;

    // Process testnet nodes
    let testnet_nodes = read_node_list("nodes_test.txt")?;
    let testnet_seeds = generate_seed_array(&testnet_nodes);
    std::fs::write("seeds_test.rs", testnet_seeds)?;

    println!("Seed node arrays generated successfully!");
    Ok(())
}
