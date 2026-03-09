use crate::obj_parser;
use crate::vertex::Vertex;

pub struct Scene {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

pub fn load_scene(path: &str) -> Scene {
    let (vertices, indices) = obj_parser::load_obj(path);
    Scene { vertices, indices }
}
