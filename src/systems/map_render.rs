use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(Layers::Map as usize);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let idx = map_idx(x, y);
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) && (player_fov.visible_tiles.contains(&pt) | map.revealed[idx]) {
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };

                let (glyph, color_pair) = match map.tiles[idx] {
                    TileType::Floor => (to_cp437('.'), ColorPair::new(tint, BLACK)),
                    TileType::Wall => (to_cp437('#'), ColorPair::new(tint, BLACK)),
                    TileType::Space => (32, ColorPair::new(WHITE, BLACK)),
                };
                let screen_pos = pt - offset;
                draw_batch.set(screen_pos, color_pair, glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
