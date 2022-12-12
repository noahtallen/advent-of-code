use crate::helpers;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    #[allow(dead_code)]
    original_val: char, // Include for debug purposes.
    height: i32,
    dist_from_start: Option<i32>,
    maybe_total_dist: Option<i32>,
}

impl Coord {
    fn same_location(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Note: equality and hash are both based only on x,y, and not other properties.
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

// The priority queue depends on `Ord`. Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .maybe_total_dist
            .cmp(&self.maybe_total_dist)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn path_finding(input: &String, part_two: bool) -> String {
    // let input = "Sabqponm
    //              abcryxxl
    //              accszExk
    //              acctuvwj
    //              abdefghi"
    //     .to_string();

    // Get the coordinate grid!
    let (raw_map, start_coord, end_coord) = get_map(&input);
    let shared_map = Arc::new(raw_map);

    let shortest_path = if part_two {
        // For each lowland starting point, find the shortest path to the end.

        // Start by beginning A* concurrently on each input. This is about 4x faster than running synchronously.
        let handles: Vec<_> = get_coords_of_height(&shared_map, 1)
            .into_iter()
            .map(|starting_coord| {
                let map = Arc::clone(&shared_map);
                thread::spawn(move || find_node_path(map, starting_coord, end_coord))
            })
            .collect();

        // Join the handles and find the shortest result. Note that results of 0
        // length indicate that no path was found, so we must filter those out.
        handles
            .into_iter()
            .filter_map(|handle| handle.join().unwrap())
            .min_by_key(|path| path.len())
    } else {
        find_node_path(Arc::clone(&shared_map), start_coord, end_coord)
    };

    if let Some(path) = shortest_path {
        // The shortest path is length minus one, because it includes the start and
        // end nodes -- if go from cell one to cell three, you only take two steps
        // (1 to 2, and then 2 to 3) to get there.
        (path.len() - 1).to_string()
    } else {
        "No path found!".to_string()
    }
}

// Finds the shortest path from start_coord to end_coord in the map using A*.
fn find_node_path(
    map: Arc<Vec<Vec<Coord>>>,
    start_coord: Coord,
    end_coord: Coord,
) -> Option<Vec<Coord>> {
    // Create the A* data structures.
    let mut pending_nodes: BinaryHeap<Coord> = BinaryHeap::new();
    let mut came_from: HashMap<Coord, (Coord, i32)> = HashMap::new(); // Include the cost for this pair.
    let mut score_from_start: HashMap<Coord, i32> = HashMap::new();

    // Initialize data with the start coordinate.
    pending_nodes.push(Coord {
        maybe_total_dist: Some(distance_heuristic(start_coord, end_coord)),
        ..start_coord
    });
    score_from_start.insert(start_coord, 0);

    // Loop until we run out of nodes to check.
    while let Some(current_coord) = pending_nodes.pop() {
        if current_coord.same_location(&end_coord) {
            return Some(reconstruct_path(came_from, current_coord));
        }
        // println!("\nCurrent node: {:?}", current_coord);

        // If we had previously found a better path to this node, we don't need
        // to process this entry, since it will be processed again in the future.
        // (It's possible to insert the same node multiple times into the queue.)
        if let Some(&(_, best_dist_from_start)) = came_from.get(&current_coord) {
            if let Some(current_dist_from_start) = current_coord.dist_from_start {
                if current_dist_from_start > best_dist_from_start {
                    continue;
                }
            }
        }

        // Check each neighboring cell that we can actually step to to see if
        // it might become a better path to the end.
        for neighbor in get_nearby_coords(&map, current_coord)
            .into_iter()
            .filter(|coord| current_coord.height + 1 >= coord.height)
        {
            // println!("  Neighbor: {:?}", neighbor);
            // The distance from start to the neighbor through the current node.
            // The distance between nodes is always 1 in this set.
            let new_dist_from_start = current_coord.dist_from_start.unwrap_or(0) + 1;
            let best_dist_from_start = score_from_start.get(&neighbor);

            // If the path to the neighbor via current is better than the previous
            // best distance to neighbor, let's change the route to use current!
            if best_dist_from_start == None || new_dist_from_start < *best_dist_from_start.unwrap()
            {
                came_from.insert(neighbor, (current_coord, new_dist_from_start));
                score_from_start.insert(neighbor, new_dist_from_start);

                let possible_end_distance =
                    Some(new_dist_from_start + distance_heuristic(neighbor, end_coord));

                // Indicate we want to check this node again in the future. Note
                // its priority in the queue using the distance heuristic.
                pending_nodes.push(Coord {
                    dist_from_start: Some(new_dist_from_start),
                    maybe_total_dist: possible_end_distance,
                    ..neighbor
                });
            }
        }
        // println!("  Next node: {:?}", pending_nodes.peek());
        // println!("  Pending nodes: {:?}", pending_nodes.len());
        // helpers::pause();
    }

    // No path was found :(
    None
}

// Reconstructs the path to the given node via the parent HashMap.
fn reconstruct_path(came_from: HashMap<Coord, (Coord, i32)>, current: Coord) -> Vec<Coord> {
    let mut total_path = vec![current];
    let mut parent_node = current;
    while let Some((parent, _)) = came_from.get(&parent_node) {
        total_path.push(*parent);
        parent_node = *parent;
    }

    // Reverse path, since we want the start node to be at the beginning.
    total_path.into_iter().rev().collect()
}

// Returns all coordinates of the given height in the map.
fn get_coords_of_height(map: &Vec<Vec<Coord>>, height: i32) -> Vec<Coord> {
    map.iter()
        .flat_map(|row| row.iter().filter(|coord| coord.height == height))
        .cloned()
        .collect()
}

// Our heuristic for A* is the naive number of steps to get to the end, not taking into account the height.
fn distance_heuristic(coord: Coord, end_coord: Coord) -> i32 {
    (coord.x - end_coord.x).abs() + (coord.y - end_coord.y).abs()
}

// Gets nearby coordinates, making sure we don't index outside of the map.
fn get_nearby_coords(map: &Vec<Vec<Coord>>, coord: Coord) -> Vec<Coord> {
    let mut coords = vec![];

    if coord.x - 1 >= 0 {
        coords.push(map[coord.y as usize][coord.x as usize - 1]);
    }
    if coord.x + 1 < map[0].len() as i32 {
        coords.push(map[coord.y as usize][coord.x as usize + 1]);
    }
    if coord.y - 1 >= 0 {
        coords.push(map[coord.y as usize - 1][coord.x as usize]);
    }
    if coord.y + 1 < map.len() as i32 {
        coords.push(map[coord.y as usize + 1][coord.x as usize]);
    }

    coords
}

// Translates the input into a 2D array of coordinates, and finds the start and end coordinates.
fn get_map(input: &String) -> (Vec<Vec<Coord>>, Coord, Coord) {
    let mut start_coord = None;
    let mut end_coord = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(c_num, c)| match c {
                    'E' | 'S' => {
                        let coord = Coord {
                            x: c_num as i32,
                            y: line_num as i32,
                            height: if c == 'E' { 26 } else { 1 },
                            original_val: c,
                            dist_from_start: if c == 'E' { None } else { Some(0) },
                            maybe_total_dist: None,
                        };
                        if c == 'E' {
                            end_coord = Some(coord);
                        } else {
                            start_coord = Some(coord);
                        }
                        coord
                    }
                    c => Coord {
                        x: c_num as i32,
                        y: line_num as i32,
                        original_val: c,
                        height: c as i32 - 96,
                        dist_from_start: None,
                        maybe_total_dist: None,
                    },
                })
                .collect()
        })
        .collect();

    // It's ok to panic -- if there is no start or end coord, something bad happened.
    (map, start_coord.unwrap(), end_coord.unwrap())
}
