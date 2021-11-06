use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandmly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandmly)>::query();
    movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let move_point = match rng.range(0, 4) {
            0 => Point::new(-1, 0), // left
            1 => Point::new(1, 0),  // right
            2 => Point::new(0, -1), // up
            _ => Point::new(0, 1),  // down
        };
        let destination = *pos + move_point;
        let new_message = (
            (),
            WantsToMove {
                destination: destination,
                entity: *entity,
            },
        );
        commands.push(new_message);
    });
}
