use crate::prelude::*;

pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(Layers::Map as usize); // cast enum to integer
        for world_y in camera.top_y..camera.bottom_y {
            for world_x in camera.left_x..camera.right_x {
                if let Some(idx) = self.try_idx(Point::new(world_x, world_y)) {
                    let screen_x = world_x - camera.left_x;
                    let screen_y = world_y - camera.top_y;
                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(screen_x, screen_y, WHITE, BLACK, to_cp437('.')),
                        TileType::Wall => ctx.set(screen_x, screen_y, WHITE, BLACK, to_cp437('#')),
                    }
                }
            }
        }
    }

    /// Determine if player is within the map bounds.
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
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
    ((y * SCREEN_WIDTH) + x) as usize
}
