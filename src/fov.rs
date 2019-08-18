use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
for y in 0..MAP_HEIGHT {
    for x in 0..MAP_WIDTH {
        fov_map.set(
            x,
            y,
            !map[x as usize][y as usize].block_sight,
            !map[x as usize][y as usize].blocked,
        );
    }
}