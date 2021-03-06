use std::io::{BufferedReader, File};
use std::str;
use std::fmt::{Show, Formatter, FormatError};
use std::collections::HashMap;

struct Face {
    points: [Vertex3<u32>, ..3],
}

impl Face {
    fn new(p1: Vertex3<u32>, p2: Vertex3<u32>, p3: Vertex3<u32>) -> Face {
        Face {
            points: [p1, p2, p3],
        }
    }
}

#[deriving(Show, Hash, PartialEq)]
pub struct Vertex3<T: Copy> {
    x: T,
    y: T,
    z: T,
}

#[deriving(Show, Hash, PartialEq)]
pub struct Vertex2<T: Copy> {
    u: T,
    v: T,
}

impl<T: Copy> Vertex3<T> {
    pub fn new(x: T, y: T, z: T) -> Vertex3<T> {
        Vertex3 {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl<T: Copy> Vertex2<T> {
    pub fn new(u: T, v: T) -> Vertex2<T> {
        Vertex2 {
            u: u,
            v: v,
        }
    }
}

#[vertex_format]
pub struct Vertex {

    #[name = "a_Pos"]
    pos: [f32, ..3],

    #[name = "a_TexCoord"]
    uv: [f32, ..2],

    #[name = "norm"]
    norm: [f32, ..3],
}

pub struct Wavefront {
    verts: Vec<Vertex>,
    faces: Vec<u32>,
}

impl Show for Wavefront {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "<Wavefront OBJ with {} verts and {} faces>", self.verts.len(), self.faces.len())
    }
}

impl Wavefront {
    pub fn open(path: &Path) -> Wavefront {
        let file = match File::open(path) {
            Err(err) => panic!("couldn't open {}: {}", path.display(), err.desc),
            Ok(file) => file,
        };

        let mut reader = BufferedReader::new(file);

        let mut verts = Vec::new();
        let mut uvs = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();
        let mut map = HashMap::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let mut words = line.as_slice().words();
            match words.next().unwrap() {
                "v" => verts.push(parse_vertex3(words)),
                "vt" => uvs.push(parse_vertex2(words)),
                "vn" => normals.push(parse_vertex3(words)),
                "f" => faces.push(parse_face(words)),
                _ => (),
            }
        }

        if verts.len() == 0 {
            verts.push(Vertex3::new(0.,0.,0.))
        }
        if uvs.len() == 0 {
            uvs.push(Vertex2::new(0.,0.))
        }
        if normals.len() == 0 {
            normals.push(Vertex3::new(0.,0.,0.))
        }

        let mut obj = Wavefront {
            verts: Vec::new(),
            faces: Vec::new(),
        };

        for face in faces.iter() {
            for point in face.points.iter() {
                let idx = match map.get(&(point.x, point.y, point.z)) {
                    Some(&index) => {
                        obj.faces.push(index);
                        None
                    }
                    None => {
                        let pos = verts[point.x as uint];
                        let pos_arr = [pos.x, pos.y, pos.z];
                        let uv = uvs[point.y as uint];
                        let uv_arr = [uv.u, uv.v];
                        let norms = normals[point.z as uint];
                        let norm_arr = [norms.x, norms.y, norms.z];
                        obj.verts.push(Vertex {pos: pos_arr, uv: uv_arr, norm: norm_arr});
                        let flen = obj.verts.len() as u32 -1;
                        obj.faces.push(flen);
                        Some(flen)
                    }
                };
                idx.map(|x| map.insert((point.x, point.y, point.z), x));
            }
        }

        obj
    }

    pub fn vertex(&self) -> &Vec<Vertex> {
        &self.verts
    }

    pub fn faces(&self) -> &Vec<u32> {
        &self.faces
    }
}

fn parse_vertex3(words: str::Words) -> Vertex3<f32> {
    let mut points = words.map(|x| from_str::<f32>(x).unwrap());
    Vertex3::new(points.next().unwrap(), points.next().unwrap(), points.next().unwrap())
}

fn parse_vertex2(words: str::Words) -> Vertex2<f32> {
    let mut points = words.map(|x| from_str::<f32>(x).unwrap());
    Vertex2::new(points.next().unwrap(), points.next().unwrap())
}

fn parse_face(words: str::Words) -> Face {
    let mut parts = words.take(3);

    let mut stuff = Vec::with_capacity(3);
    for part in parts {
        let component: Vec<u32> = part.split('/').map(|x| from_str::<u32>(x).unwrap_or(1)-1).take(3).collect();
        stuff.push(Vertex3::new(component[0], component[1], component[2]));
    }
    Face::new(stuff[0], stuff[1], stuff[2])
}
