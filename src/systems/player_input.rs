use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn get_player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
            let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
                .unwrap();

            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    // return true if an enemy is positioned at the destination.
                    **pos == destination
                })
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            // If nothing is attackable, send normal move intent.
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination: destination,
                    },
                ));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}
