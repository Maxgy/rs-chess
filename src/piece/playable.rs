use super::Color;

pub trait Playable {
    fn color(&self) -> Color;

    fn symbol(&self) -> char;
}
