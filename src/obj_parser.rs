use crate::vertex::Vertex;
use std::fs;

pub fn load_obj(path: &str) -> (Vec<Vertex>, Vec<u16>) {
    let contents = fs::read_to_string(path).expect("Failed to read OBJ file");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                vertices.push(Vertex {
                    position: [x, y, z],
                    color: [0.5, 0.0, 0.5],
                });
            }
            Some("f") => {
                let face: Vec<u16> = parts
                    .map(|p| p.split('/').next().unwrap().parse::<u16>().unwrap() - 1)
                    .collect();
                // Triangulate in case of quads or n-gons
                for i in 1..face.len() - 1 {
                    indices.push(face[0]);
                    indices.push(face[i]);
                    indices.push(face[i + 1]);
                }
            }
            _ => {}
        }
    }

    (vertices, indices)
}
