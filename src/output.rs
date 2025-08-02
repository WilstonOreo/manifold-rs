// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: Apache-2.0

//! Write [Mesh] to STL and PLY

use crate::{Manifold, Mesh};

type Vec3 = cgmath::Vector3<f32>;

pub trait Vertex {
    fn from_slice_and_offset(slice: &[f32], offset: usize) -> Self;

    /// Return position
    fn pos(&self) -> &Vec3;

    /// Return normal
    fn normal(&self) -> Option<&Vec3>;

    /// Return number of properties
    fn num_props(&self) -> u32;
}

/// Vertex with position
#[derive(Clone, Copy, Debug)]
pub struct VertexPos3 {
    /// position
    pub pos: Vec3,
}

impl Vertex for VertexPos3 {
    /// Create a vertex from a slice and an offset
    fn from_slice_and_offset(slice: &[f32], offset: usize) -> Self {
        Self {
            pos: Vec3::new(slice[offset], slice[offset + 1], slice[offset + 2]),
        }
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn normal(&self) -> Option<&Vec3> {
        None
    }

    fn num_props(&self) -> u32 {
        3
    }
}

/// A Vertex with Normal and Position
pub struct VertexPos3Normal {
    pub pos: Vec3,
    pub normal: Vec3,
}

impl Vertex for VertexPos3Normal {
    fn from_slice_and_offset(slice: &[f32], offset: usize) -> Self {
        Self {
            pos: Vec3::new(slice[offset], slice[offset + 1], slice[offset + 2]),
            normal: Vec3::new(slice[offset + 3], slice[offset + 4], slice[offset + 5]),
        }
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn normal(&self) -> Option<&Vec3> {
        Some(&self.normal)
    }

    fn num_props(&self) -> u32 {
        6
    }
}

/// Triangle
#[derive(Clone, Copy, Debug)]
pub struct Triangle<T>(pub T, pub T, pub T);

impl Triangle<VertexPos3> {
    /// Calculate the normal of the triangle
    fn normal(&self) -> Vec3 {
        use cgmath::InnerSpace;
        let u = self.1.pos() - self.0.pos();
        let v = self.2.pos() - self.0.pos();
        u.cross(v).normalize()
    }

    /// Write the triangle to an STL file
    fn write_stl(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        let n = self.normal();
        writeln!(writer, "facet normal {} {} {}", n.x, n.y, n.z)?;
        writeln!(writer, "\touter loop")?;
        writeln!(
            writer,
            "\t\tvertex {} {} {}",
            self.0.pos().x,
            self.0.pos().y,
            self.0.pos().z
        )?;
        writeln!(
            writer,
            "\t\tvertex {} {} {}",
            self.1.pos().x,
            self.1.pos().y,
            self.1.pos().z
        )?;
        writeln!(
            writer,
            "\t\tvertex {} {} {}",
            self.2.pos().x,
            self.2.pos().y,
            self.2.pos().z
        )?;
        writeln!(writer, "\tendloop")?;
        writeln!(writer, "endfacet")?;
        Ok(())
    }
}

/// Interpret vertices and indices as triangles and write them to an STL file
pub fn write_stl(
    vertices: &[f32],
    num_props: u32,
    indices: &[u32],
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    writeln!(writer, "solid")?;
    let num_props = num_props as usize;
    for i in (0..indices.len()).step_by(3) {
        Triangle(
            VertexPos3::from_slice_and_offset(vertices, indices[i] as usize * num_props),
            VertexPos3::from_slice_and_offset(vertices, indices[i + 1] as usize * num_props),
            VertexPos3::from_slice_and_offset(vertices, indices[i + 2] as usize * num_props),
        )
        .write_stl(writer)?;
    }
    writeln!(writer, "endsolid")?;

    Ok(())
}

pub fn write_ply(
    vertices: &[f32],
    num_props: u32,
    indices: &[u32],
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    let num_props = num_props as usize;

    writeln!(writer, "ply")?;
    writeln!(writer, "format ascii 1.0")?;
    writeln!(writer, "comment written by rust-sdf")?;

    assert!(
        vertices.len() % num_props == 0,
        "Number of vertices elements must be divisible by num_props"
    );

    assert!(
        indices.len() % 3 == 0,
        "Number of indices must be divisible by 3"
    );

    writeln!(
        writer,
        "element vertex {len}",
        len = vertices.len() / num_props
    )?;
    writeln!(writer, "property float x")?;
    writeln!(writer, "property float y")?;
    writeln!(writer, "property float z")?;
    if num_props > 3 {
        writeln!(writer, "property float nx")?;
        writeln!(writer, "property float ny")?;
        writeln!(writer, "property float nz")?;
    }

    writeln!(writer, "element face {len}", len = indices.len() / 3)?;
    writeln!(writer, "property list uchar int vertex_index")?;
    writeln!(writer, "end_header")?;

    vertices.chunks(num_props).try_for_each(|chunk| {
        chunk.iter().try_for_each(|x| write!(writer, "{x} "))?;
        writeln!(writer)
    })?;

    indices.chunks(3).try_for_each(|triangle| {
        writeln!(writer, "3 {} {} {}", triangle[0], triangle[1], triangle[2])
    })?;

    Ok(())
}

pub trait WriteStl {
    fn write_stl(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>;

    fn write_stl_to_file(&self, filename: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let mut writer = std::fs::File::create(filename)?;
        self.write_stl(&mut writer)
    }
}

pub trait WritePly {
    fn write_ply(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>;

    fn write_ply_to_file(&self, filename: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let mut writer = std::fs::File::create(filename)?;
        self.write_ply(&mut writer)
    }
}

impl WriteStl for Mesh {
    fn write_stl(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        write_stl(&self.vertices(), self.num_props(), &self.indices(), writer)
    }
}

impl WriteStl for Manifold {
    fn write_stl(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        self.to_mesh().write_stl(writer)
    }
}

impl WritePly for Mesh {
    fn write_ply(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        write_ply(&self.vertices(), self.num_props(), &self.indices(), writer)
    }
}

impl WritePly for Manifold {
    fn write_ply(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        self.to_mesh().write_ply(writer)
    }
}
