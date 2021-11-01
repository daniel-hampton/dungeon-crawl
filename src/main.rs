#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::*;
    pub use legion::*;

    pub const MAP_WIDTH: i32 = 80;
    pub const MAP_HEIGHT: i32 = 50;
    pub const MAP_SIZE: i32 = MAP_WIDTH * MAP_HEIGHT;
    pub const DISPLAY_WIDTH: i32 = MAP_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = MAP_HEIGHT / 2;

    pub enum Layers {
        Map,
        Characters,
        Info,
    }

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        spawn_player(&mut ecs, map_builder.player_start);
        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(Layers::Map as usize);
        ctx.cls();
        ctx.set_active_console(Layers::Characters as usize);
        ctx.cls();
        ctx.set_active_console(Layers::Info as usize);
        ctx.cls();

        // Execute Systems
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // Render Draw Buffer
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    // Initialize engine/terminal connection.
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
