use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &mut SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(Layers::Info as usize);
    draw_batch.print_centered(2, "Explore the Dungeon. Cursor keys and WASD to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        MAP_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );
    // Number is is intended to be higher than number of tiles in previous
    // two slayers.
    draw_batch.submit(10000).expect("Batch error");
}
