use std::cmp::Ordering;
use std::result;
use std::collections::BinaryHeap;
use cgmath::Vector2;
use static_grid::StaticGrid;
use spatial_hash::{SpatialHashTable, SpatialHashCell};
use direction::{CardinalDirection, CardinalDirections};
use dijkstra_map::DijkstraMap;

#[derive(Debug, Clone, Copy)]
pub struct PathNode {
    pub direction: CardinalDirection,
    pub origin: Vector2<i32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    NoPath,
    OutOfBounds,
}

pub type Result<T> = result::Result<T, Error>;

struct Node {
    score: u32,
    coord: Vector2<u32>,
}

struct Cell {
    cost: u32,
    score: u32,
    visited: u64,
    seen: u64,
    enter_direction: Option<CardinalDirection>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            cost: 0,
            score: 0,
            visited: 0,
            seen: 0,
            enter_direction: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

pub struct SearchEnv {
    grid: StaticGrid<Cell>,
    queue: BinaryHeap<Node>,
    seq: u64,
}

impl SearchEnv {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            grid: StaticGrid::new_default(width, height),
            queue: BinaryHeap::new(),
            seq: 0,
        }
    }

    pub fn search<C>(&mut self,
                     spatial_hash: &SpatialHashTable,
                     start: Vector2<i32>,
                     end: Vector2<i32>,
                     cost_fn: C,
                     dijkstra_map: &DijkstraMap,
                     path: &mut Vec<PathNode>) -> Result<()>
        where C: Fn(&SpatialHashCell, Vector2<u32>) -> Option<u32>,
    {
        if start == end {
            return Err(Error::OutOfBounds);
        }
        let (start, end) = if let Some((start, end)) = self.search_bounds(start, end) {
            (start, end)
        } else {
            return Err(Error::OutOfBounds);
        };

        self.seq += 1;
        self.queue.clear();
        self.queue.push(Node {
            score: 0,
            coord: start,
        });
        {
            let cell = self.grid.get_checked_mut(start);
            cell.enter_direction = None;
        }

        let mut found = false;

        while let Some(node) = self.queue.pop() {
            let base_cost = {
                let cell = self.grid.get_checked_mut(node.coord);
                if cell.visited == self.seq {
                    // already been here
                    continue;
                }
                if node.coord == end {
                    found = true;
                    break;
                }

                cell.visited = self.seq;

                cell.cost
            };

            let signed_coord = node.coord.cast();

            for direction in CardinalDirections {
                let next_signed_coord = signed_coord + direction.vector();
                if let Some(cell) = self.grid.get_signed_mut(next_signed_coord) {

                    if cell.visited == self.seq {
                        // already visited
                        continue;
                    }

                    let next_coord = next_signed_coord.cast();

                    let heuristic = if let Some(distance) = dijkstra_map.get_distance(next_coord) {
                        distance
                    } else {
                        continue;
                    };

                    let sh_cell = spatial_hash.get(next_coord).expect("Spatial hash of different size to dijkstra map");
                    let next_cost = if let Some(next_cost) = cost_fn(sh_cell, next_coord) {
                        next_cost
                    } else {
                        continue;
                    };

                    let cost = base_cost + next_cost;
                    let score = cost + heuristic;
                    if cell.seen == self.seq && cell.score <= score {
                        // already een better score
                        continue;
                    }

                    // update the best info for the cell
                    cell.seen = self.seq;
                    cell.cost = cost;
                    cell.score = score;
                    cell.enter_direction = Some(direction);

                    self.queue.push(Node {
                        score,
                        coord: next_coord,
                    });
                }
            }
        }

        if found {
            let mut coord = end;
            loop {
                let cell = self.grid.get_checked(coord);
                if let Some(enter_direction) = cell.enter_direction {
                    let origin = coord.cast() + enter_direction.opposite().vector();
                    path.push(PathNode {
                        direction: enter_direction,
                        origin,
                    });
                    coord = origin.cast();
                } else {
                    return Ok(());
                }
            }
        }

        return Err(Error::NoPath);
    }

    fn search_bounds(&self, start_coord: Vector2<i32>, end_coord: Vector2<i32>) -> Option<(Vector2<u32>, Vector2<u32>)> {
        let start_coord = if let Some(start_coord) = self.grid.convert_signed(start_coord) {
            start_coord
        } else {
            return None;
        };
        let end_coord = if let Some(end_coord) = self.grid.convert_signed(end_coord) {
            end_coord
        } else {
            return None;
        };
        Some((start_coord, end_coord))
    }
}
