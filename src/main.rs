use tcod::{colors, OffscreenConsole};
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

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

    //Make player
    let mut player = Object::new(25, 23, '@', colors::WHITE);
    
    //Make npcs
    let npc1 = Object::new(map::MAP_WIDTH / 2 + 5, map::MAP_HEIGHT / 2 + 1, '@', colors::YELLOW);
    let npc2 = Object::new(map::MAP_WIDTH / 2 + 2, map::MAP_HEIGHT / 2 + 2, '@', colors::YELLOW);

    //put npcs into array
    let objects = [npc1, npc2];

    //make map
    let mut map = map::make_map();
    
    //Main loop
    while !root.window_closed() {

        render_all(&mut root, &mut con, &mut player, &objects, &map);

        let handle = handle_keys(&mut root, &objects, &mut player, &map);
        if handle {
            break;
        }
    }
}

//main draw function
fn render_all(root : &mut Root, con : &mut OffscreenConsole, player : &mut Object, objects : &[Object], map : &Map) {
    //clear the offscreen canvas with black
    con.set_default_foreground(colors::BLACK);
    con.clear();

    //draw walls
    for y in 0..map::MAP_HEIGHT {
        for x in 0..map::MAP_WIDTH {
            //if tile blocks sight? Then it must be a wall?
            let wall = map[x as usize][y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, map::COLOR_DARK_WALL, BackgroundFlag::Set);
            }
            else {
                con.set_char_background(x, y, map::COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    //draw objects
    for object in objects {
        object.draw(con);
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
