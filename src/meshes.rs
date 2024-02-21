use anyhow::Result;

use crate::mesh::{Mesh, MeshPrimitive};
use crate::vertex::PNVertex;
use crate::Context;

#[rustfmt::skip]
pub fn box_mesh(ctx: &Context) -> Result<Mesh<PNVertex>> {
    let vertices = [
       PNVertex { position: [ 0.5,  0.5,  0.5], normal: [ 0.0,  0.0,  1.0,] },
       PNVertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  0.0,  1.0,] },
       PNVertex { position: [ 0.5, -0.5,  0.5], normal: [ 0.0,  0.0,  1.0,] },
       PNVertex { position: [-0.5, -0.5,  0.5], normal: [ 0.0,  0.0,  1.0,] },

       PNVertex { position: [-0.5,  0.5, -0.5], normal: [ 0.0,  0.0, -1.0,] },
       PNVertex { position: [ 0.5,  0.5, -0.5], normal: [ 0.0,  0.0, -1.0,] },
       PNVertex { position: [-0.5, -0.5, -0.5], normal: [ 0.0,  0.0, -1.0,] },
       PNVertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0,  0.0, -1.0,] },

       PNVertex { position: [ 0.5,  0.5,  0.5], normal: [ 1.0,  0.0,  0.0,] },
       PNVertex { position: [ 0.5, -0.5,  0.5], normal: [ 1.0,  0.0,  0.0,] },
       PNVertex { position: [ 0.5,  0.5, -0.5], normal: [ 1.0,  0.0,  0.0,] },
       PNVertex { position: [ 0.5, -0.5, -0.5], normal: [ 1.0,  0.0,  0.0,] },

       PNVertex { position: [-0.5, -0.5,  0.5], normal: [-1.0,  0.0,  0.0,] },
       PNVertex { position: [-0.5,  0.5,  0.5], normal: [-1.0,  0.0,  0.0,] },
       PNVertex { position: [-0.5, -0.5, -0.5], normal: [-1.0,  0.0,  0.0,] },
       PNVertex { position: [-0.5,  0.5, -0.5], normal: [-1.0,  0.0,  0.0,] },

       PNVertex { position: [ 0.5,  0.5,  0.5], normal: [ 0.0,  1.0,  0.0,] },
       PNVertex { position: [ 0.5,  0.5, -0.5], normal: [ 0.0,  1.0,  0.0,] },
       PNVertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  1.0,  0.0,] },
       PNVertex { position: [-0.5,  0.5, -0.5], normal: [ 0.0,  1.0,  0.0,] },

       PNVertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0, -1.0,  0.0,] },
       PNVertex { position: [ 0.5, -0.5,  0.5], normal: [ 0.0, -1.0,  0.0,] },
       PNVertex { position: [-0.5, -0.5, -0.5], normal: [ 0.0, -1.0,  0.0,] },
       PNVertex { position: [-0.5, -0.5,  0.5], normal: [ 0.0, -1.0,  0.0,] },
    ];
    let indices = [
         0,  1,  2,  2,  1,  3,
         4,  5,  6,  6,  5,  7,
         8,  9, 10, 10,  9, 11,
        12, 13, 14, 14, 13, 15,
        16, 17, 18, 18, 17, 19,
        20, 21, 22, 22, 21, 23,
    ];
    Mesh::new(ctx, &vertices, &indices, MeshPrimitive::Triangles)
}
