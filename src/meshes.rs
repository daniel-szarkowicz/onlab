use core::panic;
use std::collections::HashSet;
use std::f32::consts::PI;

use anyhow::Result;

use crate::context::Context;
use crate::mesh::{Mesh, MeshPrimitive};
use crate::vertex::{PNVertex, PVertex};

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
            let alpha = f32::from(a) * PI * 2.0 / f32::from(lat);
            let beta = f32::from(b) * PI / f32::from(lon - 1) - PI / 2.0;
            let y = beta.sin();
            let x = beta.cos() * alpha.sin();
            let z = beta.cos() * alpha.cos();
            vertices.push(PNVertex {
                position: [x, y, z],
                normal: [x, y, z],
            });
            if half_triangles {
                vertices.push(PNVertex {
                    position: [x, y, z],
                    normal: [-x, -y, -z],
                });
            }
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
            if half_triangles {
                indices.extend([
                    i0 * 2,
                    i1 * 2,
                    i2 * 2,
                    i0 * 2 + 1,
                    i2 * 2 + 1,
                    i1 * 2 + 1,
                ]);
            } else {
                indices.extend([i0, i1, i2, i2, i1, i3]);
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

struct Polyhedron {
    vertices: Vec<[f32; 3]>,
    faces: Vec<[usize; 3]>,
}

fn half_point(v1: &[f32; 3], v2: &[f32; 3]) -> [f32; 3] {
    [
        (v1[0] + v2[0]) / 2.0,
        (v1[1] + v2[1]) / 2.0,
        (v1[2] + v2[2]) / 2.0,
    ]
}

impl Polyhedron {
    fn normalize(&mut self) {
        for [x, y, z] in &mut self.vertices {
            let len = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
            *x /= len;
            *y /= len;
            *z /= len;
        }
    }

    fn subdivide(&mut self) {
        // collect edges
        let edges: Vec<_> = self
            .faces
            .iter()
            .flat_map(|&[a, b, c]| [[a, b], [b, c], [c, a]])
            .map(|[a, b]| [a.min(b), b.max(a)])
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        // panic!("{:?}", edges.len());

        // split edges into oldedge -> newedges maps (this creates the new vertices)
        // let mut newedges = vec![];
        // edge -> newvertex
        let mut edge_map = vec![];
        for [a, b] in &edges {
            let n = self.vertices.len();
            self.vertices
                .push(half_point(&self.vertices[*a], &self.vertices[*b]));
            // let k = newedges.len();
            // newedges.push([*a, n]);
            // newedges.push([n, *b]);
            edge_map.push(n);
        }
        // face is defined as a -> b -> c (vertices as ccw)
        // edges are min(a, b) -> max(a, b) ...
        // face with new vertices is a -> x -> b -> y -> c -> z
        // new faces are a -> x -> z, b -> y -> x, c -> z -> y, x -> y -> z
        let oldfaces = std::mem::take(&mut self.faces);
        for [a, b, c] in oldfaces {
            // x == find a -> b edge
            let x = edges
                .iter()
                .enumerate()
                .find(|(_, &[i, j])| (i == a && j == b) || (i == b && j == a))
                .map(|(e, _)| edge_map[e])
                .unwrap();
            let y = edges
                .iter()
                .enumerate()
                .find(|(_, &[i, j])| (i == b && j == c) || (i == c && j == b))
                .map(|(e, _)| edge_map[e])
                .unwrap();
            let z = edges
                .iter()
                .enumerate()
                .find(|(_, &[i, j])| (i == c && j == a) || (i == a && j == c))
                .map(|(e, _)| edge_map[e])
                .unwrap();
            self.faces.push([a, x, z]);
            self.faces.push([b, y, x]);
            self.faces.push([c, z, y]);
            self.faces.push([x, y, z]);
        }
    }
}

fn icosphere(subdivisions: usize) -> Polyhedron {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let a = 1.0;
    let b = 1.0 / phi;
    let vertices = vec![
        [0.0, b, -a],
        [b, a, 0.0],
        [-b, a, 0.0],
        [0.0, b, a],
        [0.0, -b, a],
        [-a, 0.0, b],
        [0.0, -b, -a],
        [a, 0.0, -b],
        [a, 0.0, b],
        [-a, 0.0, -b],
        [b, -a, 0.0],
        [-b, -a, 0.0],
    ];
    let faces = vec![
        [2, 1, 0],
        [1, 2, 3],
        [5, 4, 3],
        [4, 8, 3],
        [7, 6, 0],
        [6, 9, 0],
        [11, 10, 4],
        [10, 11, 6],
        [9, 5, 2],
        [5, 9, 11],
        [8, 7, 1],
        [7, 8, 10],
        [2, 5, 3],
        [8, 1, 3],
        [9, 2, 0],
        [1, 7, 0],
        [11, 9, 6],
        [7, 10, 6],
        [5, 11, 4],
        [10, 8, 4],
    ];
    let mut icosphere = Polyhedron { vertices, faces };
    icosphere.normalize();
    for _ in 0..subdivisions {
        icosphere.subdivide();
        icosphere.normalize();
    }
    icosphere
}

pub fn icosphere_mesh(
    ctx: &Context,
    subdivisions: usize,
) -> Result<Mesh<PNVertex>> {
    let Polyhedron { vertices, faces } = icosphere(subdivisions);
    let vertices: Vec<PNVertex> = vertices
        .into_iter()
        .map(|p| PNVertex {
            position: p,
            normal: p,
        })
        .collect();
    let indices: Vec<u16> = faces
        .into_iter()
        .flat_map(|f| f.map(|p| p as u16))
        .collect();
    Mesh::new(ctx, &vertices, &indices, MeshPrimitive::Triangles)
}
