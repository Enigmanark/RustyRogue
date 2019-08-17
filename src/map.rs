use tcod::Color;
use crate::tile::Tile;

pub type Map = Vec<Vec<Tile>>;

pub const MAP_WIDTH : i32 = 80;
pub const MAP_HEIGHT : i32 = 50;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

pub fn make_map() -> Map {
    // fill map with "unblocked" tiles
    let map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map
}