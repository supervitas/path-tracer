pub trait Renderable {
    fn new() -> Self where Self: Sized;
    fn intersects(&self) -> bool;
}
