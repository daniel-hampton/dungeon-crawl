use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandmly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &MovingRandmly)>::query();
    movers.iter_mut(ecs).for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let move_point = match rng.range(0, 4) {
            0 => Point::new(-1, 0), // left
            1 => Point::new(1, 0),  // right
            2 => Point::new(0, -1), // up
            _ => Point::new(0, 1),  // down
        };
        let destination = *pos + move_point;

        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    });
}
