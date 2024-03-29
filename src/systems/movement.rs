use crate::prelude::*;

/**
This is using a Legion shorthand for systems that run a single query.
The query parameters are the components that are included in the function
parameters.

*/

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // This is updating the Point componenent on the entity.
        // Essentially changing the entity's position.
        // This is the preferred approach over directly editing the value
        // of the reference.
        commands.add_component(want_move.entity, want_move.destination);

        // Mark the FieldOfView as dirty so it can update if the entity moves.
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    // Moves the camera when the player position changes.
                    camera.on_player_move(want_move.destination);

                    // Marks map tiles as revealed
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed[map_idx(pos.x, pos.y)] = true;
                    })
                }
            }
        }
        // Yay extracting things to functions!
        move_camera_if_player_moves(ecs, camera, want_move);
    }
    commands.remove(*entity);
}

fn move_camera_if_player_moves(ecs: &mut SubWorld, camera: &mut Camera, want_move: &WantsToMove) {
    // Checking if the entity has the Player component.
    if ecs
        .entry_ref(want_move.entity)
        .unwrap()
        .get_component::<Player>()
        .is_ok()
    {}
}
