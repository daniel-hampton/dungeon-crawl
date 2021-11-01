use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(Layers::Characters as usize);

    let offset = Point::new(camera.left_x, camera.top_y);

    let mut entities = <(&Point, &Render)>::query();
    entities.iter(ecs).for_each(|(pos, render)| {
        let screen_pos = *pos - offset;
        draw_batch.set(screen_pos, render.color, render.glyph);
    });
    draw_batch
        .submit((MAP_SIZE + 1000).try_into().unwrap())
        .expect("Batch error");
}
