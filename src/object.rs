use tcod::Color;
use tcod::console::Offscreen;
use tcod::{Console, BackgroundFlag};
use crate::map::Map;

//objects that appear on the map
pub struct Object {
    pub x : i32,
    pub y : i32,
    char : char,
    col : Color,
}

impl Object {
    //constructor for object
    pub fn new(x : i32, y : i32, char : char, col : Color) -> Self {
        Object { x, y, char, col}
    }

    //move object function by arrow keys
    pub fn move_by(&mut self, dx : i32, dy : i32, objects : &[Object], map : &Map) {
        let tx = self.x + dx;
        let ty = self.y + dy;
        if !map[tx as usize][ty as usize].blocked {
            let mut free = true;
            for object in objects {
                if object.x == tx && object.y == ty {
                    free = false;
                }
            }
            if free {
                self.x = tx;
                self.y = ty;
            }
        }
        
    }

    //draw function for objects
    pub fn draw(&self, con : &mut Offscreen) {
        con.set_default_foreground(self.col);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}