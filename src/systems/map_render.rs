use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(Layers::Map as usize);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if let Some(idx) = map.try_idx(pt) {
                let (glyph, color_pair) = match map.tiles[idx] {
                    TileType::Floor => (to_cp437('.'), ColorPair::new(DARKGRAY, BLACK)),
                    TileType::Wall => (to_cp437('#'), ColorPair::new(DARK_ORANGE, BLACK)),
                    TileType::Space => (32, ColorPair::new(WHITE, BLACK)),
                };
                let screen_pos = pt - offset;
                draw_batch.set(screen_pos, color_pair, glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
