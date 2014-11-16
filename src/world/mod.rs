pub enum Attribute {
    Vec3F((f32,f32,f32)),
    Bool(bool),
}

trait Object {
    fn attributes(&self) -> &HashMap<&str, Attribute>;
}
