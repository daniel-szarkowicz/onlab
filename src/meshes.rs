use std::f32::consts::PI;

use anyhow::Result;

use crate::mesh::{Mesh, MeshPrimitive};
use crate::vertex::{PNVertex, PVertex};
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

pub fn sphere_mesh(
    ctx: &Context,
    resolution: u16,
    half_triangles: bool,
) -> Result<Mesh<PNVertex>> {
    let lat = resolution * 2;
    let lon = resolution;
    let mut vertices = Vec::with_capacity((lat * lon) as usize);
    for b in 0..lon {
        for a in 0..lat {
            let alpha = (a as f32) * PI * 2.0 / (lat as f32);
            let beta = ((b as f32) * PI / ((lon - 1) as f32)) - PI / 2.0;
            let y = beta.sin();
            let x = beta.cos() * alpha.sin();
            let z = beta.cos() * alpha.cos();
            vertices.push(PNVertex {
                position: [x, y, z],
                normal: [x, y, z],
            });
        }
    }
    let mut indices = Vec::with_capacity(
        (lat * (lon - 1)) as usize * if half_triangles { 3 } else { 6 },
    );
    for b in 0..lon - 1 {
        for a in 0..lat {
            let i0 = a + b * lat;
            let i1 = (a + 1) % lat + b * lat;
            let i2 = i0 + lat;
            let i3 = i1 + lat;
            indices.extend([i0, i1, i2]);
            if !half_triangles {
                indices.extend([i2, i1, i3]);
            }
        }
    }
    Mesh::new(ctx, &vertices, &indices, MeshPrimitive::Triangles)
}

#[rustfmt::skip]
pub fn bounding_box_mesh(ctx: &Context) -> Result<Mesh<PVertex>> {
    let vertices = [
       PVertex { position: [ 0.5,  0.5,  0.5] },
       PVertex { position: [-0.5,  0.5,  0.5] },
       PVertex { position: [ 0.5, -0.5,  0.5] },
       PVertex { position: [-0.5, -0.5,  0.5] },

       PVertex { position: [-0.5,  0.5, -0.5] },
       PVertex { position: [ 0.5,  0.5, -0.5] },
       PVertex { position: [-0.5, -0.5, -0.5] },
       PVertex { position: [ 0.5, -0.5, -0.5] },
    ];
    let indices = [
        0, 1, 1, 3, 3, 2, 2, 0,
        4, 5, 5, 7, 7, 6, 6, 4,
        0, 5, 1, 4, 3, 6, 2, 7,
    ];
    Mesh::new(ctx, &vertices, &indices, MeshPrimitive::Lines)
}

#[rustfmt::skip]
pub fn rectangle_mesh(ctx: &Context) -> Result<Mesh<PVertex>> {
    let vertices = [
       PVertex { position: [ 0.5,  0.5,  0.0] },
       PVertex { position: [-0.5,  0.5,  0.0] },
       PVertex { position: [ 0.5, -0.5,  0.0] },
       PVertex { position: [-0.5, -0.5,  0.0] },
    ];
    let indices = [
        0, 1, 2, 2, 1, 3
    ];
    Mesh::new(ctx, &vertices, &indices, MeshPrimitive::Triangles)
}
