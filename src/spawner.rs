use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    // let enemy_glyph = match rng.range(0, 4) {
    //     0 => to_cp437('E'),
    //     1 => to_cp437('O'),
    //     2 => to_cp437('o'),
    //     _ => to_cp437('g'),
    // };
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        ChasingPlayer,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: glyph,
        },
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}
fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
// fn ogre() -> (i32, String, FontCharType) {
//     (1, "Ogre".to_string(), to_cp437('O'))
// }
