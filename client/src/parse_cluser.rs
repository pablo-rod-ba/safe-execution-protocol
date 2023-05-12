use std::process::Command;

#[derive(Debug)]
struct DataNode {
    name: String,
    hostname: String,
    dfs_used: u64,
    dfs_remaining: u64,
    dfs_used_percentage: f32,
    dfs_remaining_percentage: f32,
    num_of_blocks: u32,
}

fn main() {
    // Ejecutar el comando 'hdfs dfsadmin -report' y obtener la salida como un String
    let output = Command::new("hdfs")
        .arg("dfsadmin")
        .arg("-report")
        .output()
        .expect("Error al ejecutar el comando hdfs dfsadmin -report");

    let output_str =
        String::from_utf8(output.stdout).expect("Error al convertir la salida a UTF-8");

    let datanodes_info = extract_datanodes_info(&output_str);

    for datanode in datanodes_info {
        println!("{:?}", datanode);
    }
}

fn extract_datanodes_info(output: &str) -> Vec<DataNode> {
    let mut datanodes = Vec::new();
    let mut current_datanode = None;

    for line in output.lines() {
        if line.starts_with("Name:") {
            current_datanode = Some(DataNode {
                name: line.split_whitespace().nth(1).unwrap().to_string(),
                hostname: String::new(),
                dfs_used: 0,
                dfs_remaining: 0,
                dfs_used_percentage: 0.0,
                dfs_remaining_percentage: 0.0,
                num_of_blocks: 0,
            });
        } else if let Some(datanode) = current_datanode.as_mut() {
            if line.starts_with("Hostname:") {
                datanode.hostname = line.split_whitespace().nth(1).unwrap().to_string();
            } else if line.starts_with("DFS Used:") {
                datanode.dfs_used = line.split_whitespace().nth(2).unwrap().parse().unwrap();
            } else if line.starts_with("DFS Remaining:") {
                datanode.dfs_remaining = line.split_whitespace().nth(2).unwrap().parse().unwrap();
            } else if line.starts_with("DFS Used%:") {
                datanode.dfs_used_percentage = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .trim_end_matches('%')
                    .parse()
                    .unwrap();
            } else if line.starts_with("DFS Remaining%:") {
                datanode.dfs_remaining_percentage = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .trim_end_matches('%')
                    .parse()
                    .unwrap();
            } else if line.starts_with("Num of Blocks:") {
                datanode.num_of_blocks = line.split_whitespace().nth(3).unwrap().parse().unwrap();
                datanodes.push(current_datanode.take().unwrap());
            }
        }
    }

    datanodes
}
