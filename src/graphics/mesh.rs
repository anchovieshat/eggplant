use gl;


pub trait ToMesh {
    fn to_mesh(Self) -> Mesh;
}

pub struct Mesh {
    verts: Vec<gl::types::GLfloat>,
    uvs: Vec<gl::types::GLfloat>,
    normals: Vec<gl::types::GLfloat>,
    indices: Vec<gl::types::GLint>,
}

impl Mesh {
    pub fn new(verts: Vec<gl::types::GLfloat>, uvs: Vec<gl::types::GLfloat>, normals: Vec<gl::types::GLfloat>, indices: Vec<gl::types::GLint>) -> Mesh {
        Mesh {
            verts: verts,
            uvs: uvs,
            normals: normals,
            indices: indices,
        }
    }
}
