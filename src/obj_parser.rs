use crate::vertex::Vertex;
use cgmath::Vector3;
use cgmath::prelude::*;
use std::fs;

pub fn calculate_normal(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> [f32; 3] {
    let p0 = Vector3::from(v0.position);
    let p1 = Vector3::from(v1.position);
    let p2 = Vector3::from(v2.position);

    let u = p1 - p0;
    let v = p2 - p0;

    let normal = u.cross(v).normalize();
    normal.into()
}

pub fn load_obj(path: &str, smooth: bool) -> (Vec<Vertex>, Vec<u16>) {
    let contents = fs::read_to_string(path).expect("Failed to read OBJ file");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Temporary storage for positions/colors
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                positions.push([x, y, z]);
                colors.push([0.5, 0.0, 0.5]); // default color
            }
            Some("f") => {
                let face: Vec<usize> = parts
                    .map(|p| p.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                    .collect();

                // Triangulate n-gons
                for i in 1..face.len() - 1 {
                    let idx0 = face[0];
                    let idx1 = face[i];
                    let idx2 = face[i + 1];

                    if smooth {
                        // Smooth shading → share vertices
                        while vertices.len() < positions.len() {
                            vertices.push(Vertex {
                                position: positions[vertices.len()],
                                normal: [0.0, 0.0, 0.0],
                                color: colors[vertices.len()],
                            });
                        }

                        indices.push(idx0 as u16);
                        indices.push(idx1 as u16);
                        indices.push(idx2 as u16);

                        // Accumulate normals for smooth shading
                        let normal =
                            calculate_normal(&vertices[idx0], &vertices[idx1], &vertices[idx2]);
                        for &idx in &[idx0, idx1, idx2] {
                            let v = &mut vertices[idx];
                            v.normal[0] += normal[0];
                            v.normal[1] += normal[1];
                            v.normal[2] += normal[2];
                        }
                    } else {
                        // Flat shading → duplicate vertices per face
                        let v0 = Vertex {
                            position: positions[idx0],
                            normal: [0.0, 0.0, 0.0], // placeholder
                            color: colors[idx0],
                        };
                        let v1 = Vertex {
                            position: positions[idx1],
                            normal: [0.0, 0.0, 0.0],
                            color: colors[idx1],
                        };
                        let v2 = Vertex {
                            position: positions[idx2],
                            normal: [0.0, 0.0, 0.0],
                            color: colors[idx2],
                        };

                        let normal = calculate_normal(&v0, &v1, &v2);

                        vertices.push(Vertex { normal, ..v0 });
                        indices.push((vertices.len() - 1) as u16);

                        vertices.push(Vertex { normal, ..v1 });
                        indices.push((vertices.len() - 1) as u16);

                        vertices.push(Vertex { normal, ..v2 });
                        indices.push((vertices.len() - 1) as u16);
                    }
                }
            }
            _ => {}
        }
    }

    if smooth {
        // Normalize accumulated normals
        for v in vertices.iter_mut() {
            let n = Vector3::from(v.normal).normalize();
            v.normal = n.into();
        }
    }

    (vertices, indices)
}
