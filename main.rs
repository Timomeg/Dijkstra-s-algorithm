use text_io::read;
use std::collections::HashMap;
#[derive(Debug)]
// Graph structure
struct Graph{
    num_of_nodes: usize,
    matrix: Vec<Vec<u32>>
}
impl Graph{
    // Matrix builder
    fn matrix_build(&mut self){
        println!("Enter the paths between nodes");
        println!("If there is no path between nodes");
        println!("Enter 0");

        for hor in 1..=self.num_of_nodes{
            let node:Vec<u32> = Vec::new();
            self.matrix.push(node);

            for vert in 1..=self.num_of_nodes{
                if hor == vert{
                    self.matrix[hor-1].push(0);
                    continue;
                }
                println!("Enter the paths between nodes {hor} and {vert}");
                let path:u32 = read!();
                self.matrix[hor-1].push(path);
            }
        }
    }
    // Algorithm itself
    fn dijkstra(&self, node_number:usize) -> HashMap<usize, u32>{

        let mut map_of_shortest_paths: HashMap<usize, u32> = HashMap::new();
        let mut map_of_visited_nodes: HashMap<usize, bool> = HashMap::new();
        // first step of algorithm
        for i in 0..self.num_of_nodes{
            map_of_visited_nodes.insert(i, false);
            if i == node_number-1{
                map_of_shortest_paths.insert(i, 0);
                continue;
            }
            map_of_shortest_paths.insert(i, 2^32);
        }
        let mut current_node = node_number - 1;
        let mut min_node:usize = 0;
        // second step of algorithm
        while !map_of_visited_nodes.is_empty(){
            let mut min_path:u32 = 2^32;
            for (finish, _state) in map_of_visited_nodes.iter(){
                if map_of_shortest_paths[finish] > (self.matrix[current_node][*finish] + map_of_shortest_paths[&current_node])
                    && self.matrix[current_node][*finish] != 0{

                    map_of_shortest_paths.insert(*finish,self.matrix[current_node][*finish] + map_of_shortest_paths[&current_node]);
                }
                if self.matrix[current_node][*finish] < min_path
                    && self.matrix[current_node][*finish] != 0{

                    min_path = self.matrix[current_node][*finish];
                    min_node = *finish;
                }
            }
            map_of_visited_nodes.remove(&current_node);
            current_node = min_node;
        }
        map_of_shortest_paths
    }
}
fn main() {
    println!("Enter the number of nodes:");
    let num_of_nodes:usize = read!();
    let matrix:Vec<Vec<u32>> = Vec::new();
    let mut graph = Graph{
        num_of_nodes,
        matrix
    };
    graph.matrix_build();
    let input:usize = read!();
    let shortest_paths:HashMap<usize, u32> = graph.dijkstra(input);
    println!("{:?}", shortest_paths);

}
