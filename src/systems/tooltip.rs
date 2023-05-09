use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltip(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();

    // For moving the tooltip to display just over the entity's tile.
    let tooltip_overhead_offset = Point::new(0, -2);

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    draw_batch.target(Layers::Info as usize);
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(&pos))
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos + tooltip_overhead_offset, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
