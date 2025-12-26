use std::collections::HashMap;

fn dfs(node: String, graph: &HashMap<String, Vec<String>>, total_paths: &mut HashMap<String, u64>, visit: &str) -> u64 {
    if node == visit {
        return 1;
    }

    if total_paths.contains_key(&node) {
        return total_paths[&node];
    }

    let mut t = 0u64;

    if !graph.contains_key(&node) {
        return 0;
    }

    for next in &graph[&node] {
        t += dfs(next.clone(), graph, total_paths, visit);
    }

    total_paths.insert(node, t);
    t
}

pub fn part1(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut total_paths: HashMap<String, u64> = HashMap::new();

    for line in content.lines() {
        let (src, nodes) = line.split_once(": ").unwrap();

        for node in nodes.split(" ") {
            map.entry(src.to_string()).or_insert(Vec::new()).push(node.to_string());
        }
    }

    let paths = dfs("you".to_string(), &map, &mut total_paths, "out");

    println!("Day 11, part 1: {}", paths);
}

pub fn part2(input_file: &str) {
    let content = std::fs::read_to_string(input_file).unwrap();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut total_paths: HashMap<String, u64> = HashMap::new();

    for line in content.lines() {
        let (src, nodes) = line.split_once(": ").unwrap();

        for node in nodes.split(" ") {
            map.entry(src.to_string()).or_insert(Vec::new()).push(node.to_string());
        }
    }

    // Do it the stupid way

    let p1 = dfs("svr".to_string(), &map, &mut total_paths, "dac");
    total_paths.clear();
    let p2 = dfs("dac".to_string(), &map, &mut total_paths, "fft");
    total_paths.clear();
    let p3 = dfs("fft".to_string(), &map, &mut total_paths, "out");
    total_paths.clear();

    let m1 = p1 * p2 * p3;

    let p1 = dfs("svr".to_string(), &map, &mut total_paths, "fft");
    total_paths.clear();
    let p2 = dfs("fft".to_string(), &map, &mut total_paths, "dac");
    total_paths.clear();
    let p3 = dfs("dac".to_string(), &map, &mut total_paths, "out");
    total_paths.clear();

    let m2 = p1 * p2 * p3;

    println!("Day 11, part 2: {}", m1 + m2);
}