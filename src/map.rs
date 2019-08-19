use tcod::Color;
use crate::tile::Tile;
use std::cmp;
use std::vec;
use rand::Rng;

pub type Map = Vec<Vec<Tile>>;

pub const MAP_WIDTH : i32 = 80;
pub const MAP_HEIGHT : i32 = 50;

pub const MAX_ROOM_SIZE : i32 = 12;
pub const MIN_ROOM_SIZE : i32 = 5;
pub const MAX_ROOMS : i32 = 50;

pub const FOV_LIGHT_WALLS: bool = true;
pub const TORCH_RADIUS: i32 = 10;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
pub const COLOR_DARK_GROUND: Color = Color { r: 90, g: 90, b: 90 };
pub const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn get_center_x(&self) -> i32 {
        let center_x = (self.x1 + self.x2) / 2;
        center_x
    }

    pub fn get_center_y(&self) -> i32 {
        let center_y = (self.y1 + self.y2) / 2;
        center_y
    }

    pub fn intersects_with(&self, other : &Rect) -> bool {
        if self.x1 <= other.x2
            && self.x2 >= other.x1
            && self.y1 <= other.y2
            && self.y2 >= other.y1
            {
                return true;
            }
        else {
            return false;
        }
    }
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn make_map() -> (Map, (i32, i32)) {
    // fill map with "unblocked" tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut rooms = Vec::<Rect>::new();
    
    //Loop through max rooms
    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE + 1);
        let h = rand::thread_rng().gen_range(MIN_ROOM_SIZE, MAX_ROOM_SIZE + 1);

        let x = rand::thread_rng().gen_range(0, MAP_WIDTH + 1);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT + 1);
        
        //Make room to test
        let room = Rect::new(x, y, w, h);
        let mut fail = false;
        //Loop through other rooms
        for other in &rooms {
            //Did it intersect with another room?
            if room.intersects_with(other) {
                fail = true;
            }
        }
        //Did it fail by intersecting?
        if !fail {

            //is it within the bounds of the map?
            if room.x1 >= 0
                && room.x2 <= (MAP_WIDTH -1)
                && room.y1 >= 0
                && room.y2 <= (MAP_HEIGHT - 1) {
                    //if all checks out push
                    rooms.push(room);
            }
        }
    }

    let mut first = true;
    let mut start_position = (0, 0);
    let mut prev_x = 0;
    let mut prev_y = 0;
    //Now loop through all our rooms
    for room in &rooms {
        create_room(*room, &mut map);
        if first {
            let cx = room.get_center_x();
            let cy = room.get_center_y();
            prev_x = cx;
            prev_y = cy;
            first = false;
            start_position = (cx, cy);    
        } 
        //carve tunnels
        else {
            let new_x = room.get_center_x();
            let new_y = room.get_center_y();
            //carve hor then vert?
            if rand::random() {
                create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                create_v_tunnel(prev_y, new_y, prev_x, &mut map);
            } 
            //carve vert then hor?
            else {
                create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                create_h_tunnel(prev_x, new_x, prev_y, &mut map);
            }
        }
        prev_x = room.get_center_x();
        prev_y = room.get_center_y();
    }

    (map, start_position)
}