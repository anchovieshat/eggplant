use std::io::{BufferedReader, File};
use std::str;
use std::fmt::{Show, Formatter, FormatError};
use std::collections::HashMap;
use gl;
use super::mesh::{ToMesh, Mesh};
use std::vec;

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
pub struct Vertex3<T> {
    x: T,
    y: T,
    z: T,
}

pub struct Vertex3Iter<'a, T: 'a> {
    vt: &'a Vertex3<T>,
    pos: uint,
}

impl<'a, T> Iterator<&'a T> for Vertex3Iter<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        self.pos += 1;
        match self.pos {
            1 => Some(&self.vt.x),
            2 => Some(&self.vt.y),
            3 => Some(&self.vt.z),
            _ => None,
        }
    }
}

#[deriving(Show, Hash, PartialEq)]
pub struct Vertex2<T> {
    u: T,
    v: T,
}

pub struct Vertex2Iter<'a, T: 'a> {
    vt: &'a Vertex2<T>,
    pos: uint,
}

impl<'a, T> Iterator<&'a T> for Vertex2Iter<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        self.pos += 1;
        match self.pos {
            1 => Some(&self.vt.u),
            2 => Some(&self.vt.v),
            _ => None,
        }
    }
}


impl<T: Num> Vertex3<T> {
    pub fn new(x: T, y: T, z: T) -> Vertex3<T> {
        Vertex3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn iter(&self) -> Vertex3Iter<T> {
        Vertex3Iter { vt: self, pos: 0}
    }

    pub fn into_iter(self) -> vec::MoveItems<T> {
        (vec!(self.x, self.y, self.z)).into_iter()
    }
}

impl<T: Num> Vertex2<T> {
    pub fn new(u: T, v: T) -> Vertex2<T> {
        Vertex2 {
            u: u,
            v: v,
        }
    }

    pub fn iter(&self) -> Vertex2Iter<T> {
        Vertex2Iter { vt: self, pos: 0}
    }

    pub fn into_iter(self) -> vec::MoveItems<T> {
        (vec!(self.u, self.v)).into_iter()
    }
}

pub struct Wavefront {
    verts: Vec<Vertex3<f32>>,
    uvs: Vec<Vertex2<f32>>,
    normals: Vec<Vertex3<f32>>,
    faces: Vec<u32>,
}

impl Show for Wavefront {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "<Wavefront OBJ with {} verts ({} n, {} uv), and {} faces>", self.verts.len(), self.normals.len(), self.uvs.len(), self.faces.len())
    }
}

impl ToMesh for Wavefront {
    fn to_mesh(obj: Wavefront) -> Mesh {
        let verts = obj.verts.into_iter().flat_map(|x| x.into_iter().map(|y| y as gl::types::GLfloat)).collect();
        let uvs = obj.uvs.into_iter().flat_map(|x| x.into_iter().map(|y| y as gl::types::GLfloat)).collect();
        let normals = obj.normals.into_iter().flat_map(|x| x.into_iter().map(|y| y as gl::types::GLfloat)).collect();
        let indices = obj.faces.into_iter().map(|x| x as gl::types::GLint).collect();

        Mesh::new(verts, uvs, normals, indices)
    }
}

impl Wavefront {
    pub fn open(path: &Path) -> Wavefront {
        let file = match File::open(path) {
            Err(err) => panic!("couldn't open {}: {}", path.display(), err.desc),
            Ok(file) => file,
        };

        let mut reader = BufferedReader::new(file);

        let mut verts_init = Vec::new();
        let mut uvs_init = Vec::new();
        let mut normals_init = Vec::new();
        let mut faces_init = Vec::new();
        let mut data_map = HashMap::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let mut words = line.as_slice().words();
            match words.next().unwrap() {
                "v" => verts_init.push(parse_vertex3(words)),
                "vt" => uvs_init.push(parse_vertex2(words)),
                "vn" => normals_init.push(parse_vertex3(words)),
                "f" => faces_init.push(parse_face(words)),
                _ => (),
            }
        }

        if verts_init.len() == 0 {
            verts_init.push(Vertex3::new(0.,0.,0.))
        }
        if uvs_init.len() == 0 {
            uvs_init.push(Vertex2::new(0.,0.))
        }
        if normals_init.len() == 0 {
            normals_init.push(Vertex3::new(0.,0.,0.))
        }

        let mut obj = Wavefront {
            verts: Vec::new(),
            uvs: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),
        };

        for face in faces_init.iter() {
            for point in face.points.iter() {
                let idx = match data_map.find(&(point.x, point.y, point.z)) {
                    Some(&index) => {
                        obj.faces.push(index);
                        None
                    }
                    None => {
                        obj.verts.push(verts_init[point.x as uint]);
                        obj.uvs.push(uvs_init[point.y as uint]);
                        obj.normals.push(normals_init[point.z as uint]);
                        let flen = obj.faces.len() as u32;
                        obj.faces.push(flen);
                        Some((obj.faces.len()-1) as u32)
                    }
                };
                idx.map(|x| data_map.insert((point.x, point.y, point.z), x));
            }
        }

        obj
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
