#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

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
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}
impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // Spawn monsters in the center of every room except the first room
        // for the starting player.
        map_builder.rooms.iter().skip(1).for_each(|r| {
            spawn_monster(&mut ecs, &mut rng, r.center());
        });

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        spawn_player(&mut ecs, map_builder.player_start);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
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

        // Execute Systems depending on current turn state.
        self.resources.insert(ctx.key);
        let current_turn = self.resources.get::<TurnState>().unwrap().clone();
        match current_turn {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
        };

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
        .with_font("terminal8x8.png", 8, 8)
        // Layers 0, 1, 2
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(MAP_WIDTH * 2, MAP_HEIGHT * 2, "terminal8x8.png")
        // Finish
        .build()?;

    main_loop(context, State::new())
}
