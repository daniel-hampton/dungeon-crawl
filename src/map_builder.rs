use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.remove_extra_walls();
        // place player in the center of the first room.
        mb.player_start = mb.rooms[0].center();
        mb
    }

    /// Fill in every tile on the map.
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, MAP_WIDTH - 10),
                rng.range(1, MAP_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;
            // Check if any existing rooms overlap with the new room.
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true
                }
            }

            if !overlap {
                // draw room on map
                room.for_each(|point| {
                    if self.map.in_bounds(point) {
                        let idx = map_idx(point.x, point.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                // add room
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            let point = Point::new(x, y);
            if let Some(idx) = self.map.try_idx(point) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            let point = Point::new(x, y);
            if let Some(idx) = self.map.try_idx(point) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // 50/50 choice between drawing a corridor elbow up/down and over or
            // over and up/down
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    /**
     * If a wall tile has no adjacent floor tiles, replace it with a space tile.
     */
    fn remove_extra_walls(&mut self) {
        let mut space_tile_index: Vec<usize> = vec![];

        let neighboring_offsets: [(&str, Point); 8] = [
            ("top", Point::new(0, -1)),
            ("top_right", Point::new(1, -1)),
            ("right", Point::new(1, 0)),
            ("bottom_right", Point::new(1, 1)),
            ("bottom", Point::new(0, 1)),
            ("buttom_left", Point::new(-1, 1)),
            ("left", Point::new(-1, 0)),
            ("top_left", Point::new(-1, -1)),
        ];

        self.map.tiles.iter().enumerate().for_each(|(index, tile)| {
            let map_point = convert_idx_to_point(index);

            if *tile == TileType::Wall {
                let mut neighbor_is_floor: Vec<bool> = vec![];
                // check if any neighbors are Floor tiles.
                neighboring_offsets.iter().for_each(|(_, offset)| {
                    let neighboring_point = map_point + *offset;
                    // if index == 400 {

                    //     println!("index {:?}", index);
                    //     println!("map_point {:?}", map_point);
                    //     println!("offset {:?}", offset);
                    //     println!("neighboring_point {:?}", neighboring_point);
                    // }
                    if let Some(idx) = self.map.try_idx(neighboring_point) {
                        if self.map.tiles[idx] == TileType::Floor {
                            neighbor_is_floor.push(true);
                        }
                    }
                    // if there are no surrounding floors, add index to list.
                });

                if neighbor_is_floor.len() == 0 {
                    space_tile_index.push(index);
                }
            }
        });
        // Add space tiles.
        space_tile_index.iter().for_each(|index| {
            self.map.tiles[*index] = TileType::Space;
        })
    }
}
