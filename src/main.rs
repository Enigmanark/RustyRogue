use tcod::{colors, OffscreenConsole};
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::Color;

pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

mod object;
use object::Object;
mod map;
use map::Map;
mod tile;

//main method
fn main() {

    //Create main canvas and initialize
    let mut root = Root::initializer()
    .font("assets/arial10x10.png", FontLayout::Tcod)
    .font_type(FontType::Greyscale)
    .size(map::MAP_WIDTH, map::MAP_HEIGHT)
    .title("Rusty Rogue v0.1.0")
    .init();

    //Make offscreen canvas
    let mut con = tcod::OffscreenConsole::new(map::MAP_WIDTH, map::MAP_HEIGHT);

    //make map
    let (map, (player_x, player_y)) = map::make_map();
    // make player
    let mut player = Object::new(player_x, player_y, '@', colors::WHITE);

    //make npcs
    let npc1 = Object::new(32, 32, '@', colors::YELLOW);
    
    //place npcs into array
    let objects = [npc1];
    let mut map = map;
    //Main loop
    while !root.window_closed() {
        let mut previous_player_position = (-1, -1);

        let mut fov_map = FovMap::new(map::MAP_WIDTH, map::MAP_HEIGHT);
        for y in 0..map::MAP_HEIGHT {
            for x in 0..map::MAP_WIDTH {
                fov_map.set(
                    x,
                    y,
                    !map[x as usize][y as usize].block_sight,
                    !map[x as usize][y as usize].blocked,
                );
            }
        }
        let fov_recompute = previous_player_position != (player.x, player.y);
        render_all(&mut root, &mut con, &mut player, &objects, &mut map, &mut fov_map, fov_recompute);

        previous_player_position = (player.x, player.y);

        let handle = handle_keys(&mut root, &objects, &mut player, &map);
        if handle {
            break;
        }
    }
}

//main draw function
fn render_all(root : &mut Root, con : &mut OffscreenConsole, player : &mut Object, objects : &[Object], map : &mut Map, fov : &mut FovMap, frc : bool) {
    //clear the offscreen canvas with black
    con.set_default_foreground(colors::BLACK);
    con.clear();
    
    //If we should recompute fov
    if frc {
        fov.compute_fov(player.x, player.y, map::TORCH_RADIUS, map::FOV_LIGHT_WALLS, FOV_ALGO);
    }

    //draw walls
    for y in 0..map::MAP_HEIGHT {
        for x in 0..map::MAP_WIDTH {
            let visible = fov.is_in_fov(x, y);
            let wall = map[x as usize][y as usize].block_sight;
            let color : Color;
            if visible && wall {
                color = map::COLOR_LIGHT_WALL;
            }
            else if !visible && wall {
                color = map::COLOR_DARK_WALL;
            }
            else if visible && !wall {
                color = map::COLOR_LIGHT_GROUND;
            }
            else if !visible && !wall {
                color = map::COLOR_DARK_GROUND;
            }
            else {
                color = colors::WHITE;
            }

            let explored = &mut map[x as usize][y as usize].explored;
            if visible {
                *explored = true;
            }
            if *explored {
                con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    //draw objects
    for object in objects {
        if fov.is_in_fov(object.x, object.y) {
            object.draw(con);
        }
    }

    //draw player
    player.draw(con);

    //blit offscreen canvas onto main canvas
    blit(
        con,
        (0, 0),
        (map::MAP_WIDTH, map::MAP_HEIGHT),
        root,
        (0, 0),
        1.0,
        1.0,
    );

    //flush main canvas
    root.flush();
}

//get input function
fn handle_keys(root : &mut Root, objects : &[Object], player : &mut Object, map : &Map) -> bool {
    let key = root.wait_for_keypress(true);

    match key {
        Key { code : Up, .. } => player.move_by(0, -1, objects, map),
        Key { code : Down , .. } => player.move_by(0, 1, objects, map),
        Key { code : Left , .. } => player.move_by(-1, 0, objects, map),
        Key { code : Right , .. } => player.move_by(1, 0, objects, map),
        Key { code : Escape , .. } => return true,
        _ => {},
    }

    false
}
