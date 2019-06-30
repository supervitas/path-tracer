pub trait Renderable {
    fn new() -> Self;
    fn intersects(&self) -> bool;
}
