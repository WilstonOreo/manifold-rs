// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: Apache-2.0

//! Rust integration of C++ library *Manifold* for geometric operations

#[cxx::bridge(namespace = "manifold_rs")]
mod ffi {
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("manifold_rs.h");

        /// Manifold object, wrapper for C++ manifold object.
        type Manifold;

        /// Create a sphere manifold.
        fn sphere(radius: f64, segments: u32) -> UniquePtr<Manifold>;

        /// Create a cube manifold.
        fn cube(x_size: f64, y_size: f64, z_size: f64) -> UniquePtr<Manifold>;

        /// Create a cylinder manifold.
        fn cylinder(
            radius_low: f64,
            radius_high: f64,
            height: f64,
            segments: u32,
        ) -> UniquePtr<Manifold>;

        /// Get the union of two manifolds.
        fn union_(a: &Manifold, b: &Manifold) -> UniquePtr<Manifold>;

        /// Get the intersection of two manifolds.
        fn intersection(a: &Manifold, b: &Manifold) -> UniquePtr<Manifold>;

        /// Get the difference of two manifolds.
        fn difference(a: &Manifold, b: &Manifold) -> UniquePtr<Manifold>;

        /// Create a mesh from a manifold.
        type Mesh;

        /// Get the vertices of the mesh.
        fn vertices(self: &Mesh) -> UniquePtr<CxxVector<f32>>;

        /// Get the indices of the mesh.
        fn indices(self: &Mesh) -> UniquePtr<CxxVector<u32>>;

        /// Create a mesh from a manifold.
        fn mesh_from_manifold(manifold: &Manifold) -> UniquePtr<Mesh>;

        /// Create a manifold from a mesh.
        fn manifold_from_mesh(mesh: &Mesh) -> UniquePtr<Manifold>;

        /// Create a mesh from vertices and indices.
        fn mesh_from_vertices(vertices: &[f32], indices: &[u32]) -> UniquePtr<Mesh>;

        /// Extrude a polygon to create a manifold.
        fn extrude(
            multi_polygon_data: &[&[f64]],
            height: f64,
            n_divisions: u32,
            twist_degrees: f64,
            scale_top_x: f64,
            scale_top_y: f64,
        ) -> UniquePtr<Manifold>;

        /// Revolve a polygon to create a manifold.
        fn revolve(
            multi_polygon_data: &[&[f64]],
            circular_segments: u32,
            revolve_degrees: f64,
        ) -> UniquePtr<Manifold>;
    }
}

/// Boolean operation on manifolds.
pub enum BooleanOp {
    /// Union of two manifolds.
    Union,
    /// Intersection of two manifolds.
    Intersection,
    /// Difference of two manifolds.
    Difference,
}

/// Manifold rust wrapper for C++ manifold object.
pub struct Manifold(cxx::UniquePtr<ffi::Manifold>);

impl Manifold {
    /// Create a sphere manifold.
    pub fn sphere(radius: f64, segments: u32) -> Self {
        Self(ffi::sphere(radius, segments))
    }

    /// Create a cube manifold.
    pub fn cube(x_size: f64, y_size: f64, z_size: f64) -> Self {
        Self(ffi::cube(x_size, y_size, z_size))
    }

    /// Create a cylinder manifold.
    pub fn cylinder(radius_low: f64, radius_high: f64, height: f64, segments: u32) -> Self {
        Self(ffi::cylinder(radius_low, radius_high, height, segments))
    }

    /// Get the union of two manifolds.
    pub fn union(&self, b: &Self) -> Self {
        Self(ffi::union_(self.inner(), b.inner()))
    }

    /// Get the intersection of two manifolds.
    pub fn intersection(&self, b: &Self) -> Self {
        Self(ffi::intersection(self.inner(), b.inner()))
    }

    /// Get the difference of two manifolds.
    pub fn difference(&self, b: &Self) -> Self {
        Self(ffi::difference(self.inner(), b.inner()))
    }

    pub fn boolean_op(&self, b: &Self, op: crate::BooleanOp) -> Self {
        match op {
            crate::BooleanOp::Union => self.union(b),
            crate::BooleanOp::Intersection => self.intersection(b),
            crate::BooleanOp::Difference => self.difference(b),
        }
    }

    /// Get the mesh representation of the manifold.
    pub fn to_mesh(&self) -> Mesh {
        Mesh(ffi::mesh_from_manifold(&self.0))
    }

    pub fn from_mesh(mesh: Mesh) -> Self {
        mesh.into()
    }

    /// Get the inner C++ manifold object.
    fn inner(&self) -> &ffi::Manifold {
        self.0.as_ref().unwrap()
    }
}

/// Wrapper around a C++ mesh object.
pub struct Mesh(cxx::UniquePtr<ffi::Mesh>);

/// Implementations for the Mesh struct.
impl Mesh {
    /// Create a new mesh from vertices and indices.
    pub fn new(vertices: &[f32], indices: &[u32]) -> Self {
        let mesh = ffi::mesh_from_vertices(vertices, indices);
        Self(mesh)
    }

    /// Get the vertices of the mesh.
    pub fn vertices(&self) -> Vec<f32> {
        let vertices_binding = self.0.vertices();
        let vertices = vertices_binding.as_ref().unwrap().as_slice();
        vertices.to_vec()
    }

    /// Get the indices of the mesh.
    pub fn indices(&self) -> Vec<u32> {
        let indices_binding = self.0.indices();
        let indices = indices_binding.as_ref().unwrap().as_slice();
        indices.to_vec()
    }

    /// Get the manifold representation of the mesh.
    pub fn to_manifold(&self) -> Manifold {
        let manifold = ffi::manifold_from_mesh(&self.0);
        Manifold(manifold)
    }
}

/// Convert Mesh to Manifold struct
impl From<Mesh> for Manifold {
    fn from(mesh: Mesh) -> Self {
        mesh.to_manifold()
    }
}

/// Convert Manifold to Mesh struct
impl From<Manifold> for Mesh {
    fn from(manifold: Manifold) -> Self {
        manifold.to_mesh()
    }
}

#[test]
fn test_manifold_ffi() {
    let sphere = ffi::sphere(1.0, 32);

    let mesh = ffi::mesh_from_manifold(&sphere);

    let vertices_binding = mesh.vertices();
    let vertices = vertices_binding.as_ref().unwrap().as_slice();
    assert!(!vertices.is_empty());

    let indices_binding = mesh.indices();
    let indices = indices_binding.as_ref().unwrap().as_slice();
    assert!(!indices.is_empty());
}
