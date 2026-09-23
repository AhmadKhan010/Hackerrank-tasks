use std::io::{self, BufRead};
use std::collections::BinaryHeap ;
use std::cmp::Reverse ;

/*
 * Complete the 'getCost' function below.
 *
 * The function accepts WEIGHTED_INTEGER_GRAPH g as parameter.
 */

/*
 * For the weighted graph, <name>:
 *
 * 1. The number of nodes is <name>_nodes.
 * 2. The number of edges is <name>_edges.
 * 3. An edge exists between <name>_from[i] and <name>_to[i]. The weight of the edge is <name>_weight[i].
 *
 */

fn getCost(g_nodes: i32, g_from: &[i32], g_to: &[i32], g_weight: &[i32]) {
    // Print your answer within the function and return nothing
    
    let mut priority_queue: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new() ;
    
    priority_queue.push(Reverse((0,1))) ;
    
    // building graph first
    let mut graph: Vec<Vec<(i32, i32)>> = vec![Vec::new(); (g_nodes + 1) as usize] ;
    
    for i in 0..g_from.len() {
        let x = g_from[i] ;
        let y = g_to[i] ;
        let weight = g_weight[i] ;
        
        // because graph is bi directional
        graph[x as usize].push((y,weight)) ;
        graph[y as usize].push((x,weight)) ;
    }
    
    let mut minimum_cost: Vec<i32> = vec![i32::MAX; (g_nodes+1) as usize] ;
    minimum_cost[1] = 0 ;
    
    while let Some(Reverse((current_cost, current_node))) = priority_queue.pop() {
        
        if current_node == g_nodes {
            println!("{}", current_cost);
            return ;
        }
        
        // for optimization, we can remove it
        if current_cost > minimum_cost[current_node as usize] {
            continue ; 
        }
        
        for &(neighbour, edge_weight) in &graph[current_node as usize] {
            
            let difference = edge_weight - current_cost;
            let mut additional_cost = 0 ;
            if difference > 0 {
                additional_cost = difference;
            }
            
            let new_cost = current_cost + additional_cost ;
            
            if new_cost < minimum_cost[neighbour as usize] {
                minimum_cost[neighbour as usize] = new_cost;
                priority_queue.push(Reverse((new_cost, neighbour))) ;
            }
        }
    }
    
    println!("NO PATH EXISTS")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let g_nodes_edges: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let g_nodes = g_nodes_edges[0].trim().parse::<i32>().unwrap();
    let g_edges = g_nodes_edges[1].trim().parse::<i32>().unwrap();

    let mut g_from: Vec<i32> = Vec::with_capacity(g_edges as usize);
    let mut g_to: Vec<i32> = Vec::with_capacity(g_edges as usize);
    let mut g_weight: Vec<i32> = Vec::with_capacity(g_edges as usize);

    for _ in 0..g_edges {
        let g_from_to: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let g_from_temp = g_from_to[0].trim().parse::<i32>().unwrap();
        let g_to_temp = g_from_to[1].trim().parse::<i32>().unwrap();
        let g_weight_temp = g_from_to[2].trim().parse::<i32>().unwrap();

        g_from.push(g_from_temp);
        g_to.push(g_to_temp);
        g_weight.push(g_weight_temp);
    }

    getCost(g_nodes, &g_from, &g_to, &g_weight);
}
