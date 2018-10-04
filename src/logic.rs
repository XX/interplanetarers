struct Player;

struct Thing;

pub struct Level;

trait Move<V> {
    fn go(&mut self, dir: V);
}

trait Rotate<O, V> {
    fn roll(&mut self, orig: O, dir: V);
}