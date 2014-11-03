struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

trait Object {
    fn position(&self) -> Vector;
    fn set_position(&self, Vector);
}
