use std::collections::HashSet;
use std::process;

#[derive(Copy, Clone)]
enum Dir {
    R,
    L,
    U,
    D,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    // Moves a coordinate one step in the given direction.
    fn move_towards(&mut self, direction: Dir) {
        // Grab coordinate to move.
        let val_ptr = match direction {
            Dir::R | Dir::L => &mut self.x,
            Dir::U | Dir::D => &mut self.y,
        };

        // The amount to move.
        let by_num = match direction {
            Dir::U | Dir::R => 1,
            Dir::D | Dir::L => -1,
        };

        *val_ptr += by_num;
    }

    // Moves a coordinate closer to the given coord, following "rope rules."
    fn follow(&mut self, to_coord: Coord) {
        // Scenario one: touching coordinates do not move.
        if self.touching(to_coord) {
            return;
        }

        // We could just use +/-1 here, but why not use the fun direction movement
        // function we created above!

        // Move towards the other's x coord if not equal.
        if self.x > to_coord.x {
            self.move_towards(Dir::L);
        } else if self.x < to_coord.x {
            self.move_towards(Dir::R);
        }

        // Move towards the other's y coord if not equal.
        if self.y > to_coord.y {
            self.move_towards(Dir::D);
        } else if self.y < to_coord.y {
            self.move_towards(Dir::U);
        }
    }

    // Returns true if the two coordinates are within one x,y of each other in any direction.
    // Overlapping counts as touching.
    fn touching(&mut self, other: Coord) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

pub fn rope_shenanigans(input: &String, part_two: bool) -> String {
    // let input = "R 4
    //              U 4
    //              L 3
    //              D 1
    //              R 4
    //              D 1
    //              L 5
    //              R 2"; // Expected answer is 13.

    // Parse input to tuple of Directions and numbers.
    let input = input.lines().map(|s| {
        let split = s.trim().split_at(1);
        let direction = match split.0 {
            "R" => Dir::R,
            "L" => Dir::L,
            "U" => Dir::U,
            "D" => Dir::D,
            _ => {
                println!("Unknown direction: {}", split.0);
                process::exit(1);
            }
        };
        let num_steps = split.1.trim().parse::<i32>().unwrap();
        (direction, num_steps)
    });

    let mut visited_coords: HashSet<Coord> = HashSet::new();

    // Initialize a rope to move around.
    let rope_len = if part_two { 10 } else { 2 };
    let mut rope: Vec<Coord> = Vec::new();
    for _ in 0..rope_len {
        rope.push(Coord { x: 0, y: 0 });
    }
    visited_coords.insert(*rope.last().unwrap());

    for (direction, num_steps) in input {
        for _ in 0..num_steps {
            // Move head in direction.
            rope[0].move_towards(direction);

            // Note: it'd be fun to use a window iterator to store the current
            // and perious knots, but it doesn't seem we can use windows and also
            // mutate the data.

            // Simulate each knot following the previous movement one by one.
            for i in 1..rope.len() {
                // The "to_knot" is knot changing, so we can clone it.
                let to_knot = rope[i - 1].clone();
                rope[i].follow(to_knot);
            }

            visited_coords.insert(*rope.last().unwrap());
        }
    }

    visited_coords.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_equal() {
        let mut coord_1 = Coord { x: 1, y: 2 };

        assert_eq!(coord_1, Coord { x: 1, y: 2 });

        coord_1.follow(Coord { x: 1, y: 2 });
        assert_eq!(coord_1, Coord { x: 1, y: 2 });
    }

    #[test]
    fn test_follow_xy() {
        let mut coord_1 = Coord { x: 1, y: 2 };

        coord_1.follow(Coord { x: 1, y: 4 });
        assert_eq!(coord_1, Coord { x: 1, y: 3 }); // y plus one.

        coord_1.follow(Coord { x: -1, y: 3 });
        assert_eq!(coord_1, Coord { x: 0, y: 3 }); // x minus one.

        coord_1.follow(Coord { x: 0, y: 1 });
        assert_eq!(coord_1, Coord { x: 0, y: 2 }); // y minus one.

        coord_1.follow(Coord { x: 3, y: 2 });
        assert_eq!(coord_1, Coord { x: 1, y: 2 }); // x plus one.
    }

    #[test]
    fn test_follow_diag() {
        let mut coord_1 = Coord { x: 1, y: 1 };
        coord_1.follow(Coord { x: 2, y: 3 });
        assert_eq!(coord_1, Coord { x: 2, y: 2 });

        let mut coord_2 = Coord { x: 1, y: 1 };
        coord_2.follow(Coord { x: 3, y: 2 });
        assert_eq!(coord_2, Coord { x: 2, y: 2 });

        let mut coord_3 = Coord { x: -1, y: -1 };
        coord_3.follow(Coord { x: -2, y: -3 });
        assert_eq!(coord_3, Coord { x: -2, y: -2 });

        let mut coord_4 = Coord { x: -1, y: -1 };
        coord_4.follow(Coord { x: 1, y: -2 });
        assert_eq!(coord_4, Coord { x: 0, y: -2 });
    }
}
