pub mod camera;
pub mod input;
pub mod timing;
pub mod window;

struct BoundingBody;

pub trait Game {
    fn main_loop(&mut self);
}