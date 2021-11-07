use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandmly)]
#[read_component(Player)]
#[read_component(Health)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandmly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let move_point = match rng.range(0, 4) {
            0 => Point::new(-1, 0), // left
            1 => Point::new(1, 0),  // right
            2 => Point::new(0, -1), // up
            _ => Point::new(0, 1),  // down
        };
        let destination = *pos + move_point;

        let mut attacked = false;
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(victim, _, _)| {
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },
                    ));
                }
                attacked = true;
            });

        if !attacked {
            let move_message = (
                (),
                WantsToMove {
                    destination: destination,
                    entity: *entity,
                },
            );
            commands.push(move_message);
        }
    });
}
