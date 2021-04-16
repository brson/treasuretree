use std::fmt;
use rand::{self, Rng};
use std::borrow::Cow;

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub struct Treasure<'a>(Cow<'a, str>);

impl<'a> Treasure<'a> {
    pub fn new(size: usize) -> Treasure<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        dbg!(&id);

        Treasure(Cow::Owned(id))
    }
}

impl<'a> fmt::Display for Treasure<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
