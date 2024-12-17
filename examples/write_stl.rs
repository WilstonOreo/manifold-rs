use std::io::Write;

use cgmath::InnerSpace;

extern crate manifold_rs;

type Vec3 = cgmath::Vector3<f32>;

/// Vertex with position and normal vector
#[derive(Clone, Copy, Debug)]
struct Vertex {
    /// position
    pub pos: Vec3,
    /// normal vector (unused)
    pub _normal: Vec3,
}

impl Vertex {
    /// Create a vertex from a slice and an offset
    fn from_slice_and_offset(slice: &[f32], offset: usize) -> Self {
        Vertex {
            pos: Vec3::new(slice[offset], slice[offset + 1], slice[offset + 2]),
            _normal: Vec3::new(slice[offset + 3], slice[offset + 4], slice[offset + 5]),
        }
    }
}

/// Triangle
#[derive(Clone, Copy, Debug)]
struct Triangle<T>(pub T, pub T, pub T);

impl Triangle<Vertex> {
    /// Calculate the normal of the triangle
    fn normal(&self) -> Vec3 {
        let u = self.1.pos - self.0.pos;
        let v = self.2.pos - self.0.pos;
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
            self.0.pos.x, self.0.pos.y, self.0.pos.z
        )?;
        writeln!(
            writer,
            "\t\tvertex {} {} {}",
            self.1.pos.x, self.1.pos.y, self.1.pos.z
        )?;
        writeln!(
            writer,
            "\t\tvertex {} {} {}",
            self.2.pos.x, self.2.pos.y, self.2.pos.z
        )?;
        writeln!(writer, "\tendloop")?;
        writeln!(writer, "endfacet")?;
        Ok(())
    }
}

/// Interpret vertices and indices as triangles and write them to an STL file
fn write_stl(
    vertices: &[f32],
    indices: &[u32],
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    for i in (0..indices.len()).step_by(3) {
        Triangle(
            Vertex::from_slice_and_offset(vertices, indices[i] as usize),
            Vertex::from_slice_and_offset(vertices, indices[i + 1] as usize),
            Vertex::from_slice_and_offset(vertices, indices[i + 2] as usize),
        )
        .write_stl(writer)?;
    }
    Ok(())
}

/// Write a manifold to an STL file
fn write_manifold_to_stl_file(
    manifold: &manifold_rs::Manifold,
    filename: &str,
) -> std::io::Result<()> {
    let mesh = manifold.to_mesh();

    let vertices = mesh.vertices();
    let indices = mesh.indices();

    let mut writer = std::fs::File::create(filename)?;

    writeln!(&mut writer, "solid")?;

    write_stl(&vertices, &indices, &mut writer)?;

    writeln!(&mut writer, "endsolid")?;

    println!("Wrote {}", filename);

    Ok(())
}

fn main() -> std::io::Result<()> {
    write_manifold_to_stl_file(&manifold_rs::Manifold::sphere(4.0, 128), "sphere.stl")?;

    Ok(())
}
