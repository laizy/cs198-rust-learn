use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    // TODO: Implement the necessary methods for Rooms.
    //
    pub fn neighbor(&self, name: String) -> Result<Rc<RefCell<Room>>, ()> {
        for hall in &self.halls {
            let room = hall.other(&self);
            if room.borrow().name.to_lowercase() == name.to_lowercase() {
                return Ok(room.clone())
            }
        }

        Err(())
    }

    pub fn neighbors_string(&self) -> String {
        let mut res = String::new();
        for hall in &self.halls {
            let room = hall.other(&self);
            res += " ";
            res += &room.borrow().name;
        }

        res
    }
}
