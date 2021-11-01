use crate::prelude::*;

pub const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Space,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// Determine if player is within the map bounds.
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }

    /// Determine if the player can enter the tile or is at the bounds of the
    /// map.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(map_idx(point.x, point.y))
        } else {
            None
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}

pub fn convert_idx_to_point(idx: usize) -> Point {
    let i32_idx = idx as i32;
    let x = i32_idx % MAP_WIDTH; // remainder, loops after reaching map width.
    let y = i32_idx / MAP_WIDTH; // result always an integer rounded down.
    Point::new(x, y)
}
