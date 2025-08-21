// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: Apache-2.0

#pragma once

#include "rust/cxx.h"

#include <memory>
#include <manifold/manifold.h>

namespace manifold
{
    using Mesh = MeshGLP<float, uint32_t>;
} // namespace manifold

namespace manifold_rs
{
    /// @brief Wrapper around manifold::Polygons
    /// @details This class will be exposed to Rust
    class Polygons
    {
    public:
        Polygons();
        Polygons(::manifold::Polygons &&polygons);
        ~Polygons();

        /// @brief Get the number of polygons
        size_t size() const;

        /// @brief Get a polygon by index as a slice of doubles
        rust::Slice<const double> get_as_slice(size_t index) const;

        std::unique_ptr<::manifold::Polygons> polygons;
    };

    /// @brief Wrapper around manifold::Manifold
    /// @details This class will be exposed to Rust
    class Manifold
    {
    public:
        Manifold();
        Manifold(::manifold::Manifold &&manifold);
        ~Manifold();

        /// @brief Does Manifold have triangles?
        bool is_empty() const;

        /// @brief Slice the manifold at a given height
        std::unique_ptr<Polygons> slice(double height) const;

        /// @brief Project the manifold
        std::unique_ptr<Polygons> project() const;

        /// @brief Trim the manifold by a plane
        /// @param x X coordinate of normal vector of the plane
        /// @param y Y coordinate of normal vector of the plane
        /// @param z Z coordinate of normal vector of the plane
        /// @param offset Offset of the plane
        std::unique_ptr<Manifold> trim_by_plane(double x, double y, double z, double offset) const;

        /// @brief Calculate the convex hull
        std::unique_ptr<Manifold> hull() const;

        /// @brief Translate the manifold
        std::unique_ptr<Manifold> translate(double x, double y, double z) const;

        /// @brief Scale the manifold
        std::unique_ptr<Manifold> scale(double x, double y, double z) const;

        /// @brief Rotate the manifold
        std::unique_ptr<Manifold> rotate(double x_degrees, double y_degrees = 0.0,
                                         double z_degrees = 0.0) const;

        /// @brief Refine manifold `n` times
        std::unique_ptr<Manifold> refine(std::int32_t n) const;

        /// Refine manifold to Length.
        std::unique_ptr<Manifold> refine_to_length(double t) const;

        /// Refine to tolerance.
        std::unique_ptr<Manifold> refine_to_tolerance(double t) const;

        /// Smooth by normals.
        std::unique_ptr<Manifold> smooth_by_normals(std::int32_t n) const;

        /// Smooth out.
        std::unique_ptr<Manifold> smooth_out(double min_sharp_angle, double min_smoothness) const;

        /// Calculate normals for the manifold and return a new one.
        std::unique_ptr<Manifold> calculate_normals(std::int32_t normal_idx, double min_sharp_angle) const;

        std::unique_ptr<::manifold::Manifold> manifold;
    };

    /// @brief Create a new empty manifold
    std::unique_ptr<Manifold> tetrahedron();

    /// @brief Create a cube
    /// @param x_size A size of the cube in x direction
    /// @param y_size A size of the cube in y direction
    /// @param z_size A size of the cube in z direction
    /// @return A new cube as a Manifold
    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size);

    /// @brief Create a sphere
    /// @param radius Radius of the sphere
    /// @param circular_segments  Number of circular segments
    /// @return A new sphere as a Manifold
    std::unique_ptr<Manifold> sphere(double radius, uint32_t circular_segments);

    /// @brief Create a cylinder
    /// @param radius_low Lower radius of the cylinder
    /// @param radius_high Higher radius of the cylinder
    /// @param height Height of the cylinder
    /// @param circular_segments Number of circular segments
    /// @return A new cylinder as a Manifold
    std::unique_ptr<Manifold> cylinder(double radius_low, double radius_high, double height, uint32_t circular_segments);

    /// @brief Perform a union operation
    /// @param a First manifold
    /// @param b Second manifold
    /// @return A new manifold as a result of the union operation
    std::unique_ptr<Manifold> union_(const Manifold &a, const Manifold &b);

    /// @brief Perform an intersection operation
    /// @param a First manifold
    /// @param b Second manifold
    /// @return A new manifold as a result of the intersection operation
    std::unique_ptr<Manifold> intersection(const Manifold &a, const Manifold &b);

    /// @brief Perform a difference operation
    /// @param a First manifold
    /// @param b Second manifold
    /// @return A new manifold as a result of the difference operation
    std::unique_ptr<Manifold> difference(const Manifold &a, const Manifold &b);

    /// @brief Extrude a multi-polygon to create a 3D shape
    std::unique_ptr<Manifold> extrude(
        rust::Slice<const rust::Slice<const double>> multi_polygon_data,
        double height, uint32_t divisions, double twist_degrees, double scale_top_x, double scale_top_y);

    /// @brief Revolve a multi-polygon to create a 3D shape
    std::unique_ptr<Manifold> revolve(
        rust::Slice<const rust::Slice<const double>> multi_polygon_data,
        uint32_t circular_segments, double angle);

    /// @brief A mesh, which is a collection of vertices and indices
    /// @details This class will be exposed to Rust
    class Mesh
    {
    public:
        Mesh();
        Mesh(::manifold::Mesh &&mesh);
        ~Mesh();

        /// @brief  Number of vertex properties of the mesh.
        std::uint32_t num_props() const;

        /// @brief Get the vertices of the mesh
        /// @details The vertex coefficients are in the following order:
        ///          * position x, y, z
        /// @return A vector of vertices
        std::unique_ptr<std::vector<float>> vertices() const;

        /// @brief  Get the indices of the mesh
        std::unique_ptr<std::vector<uint32_t>> indices() const;

        std::unique_ptr<::manifold::Mesh> mesh;
    };

    /// @brief Create a mesh from a manifold
    /// @param manifold A manifold
    /// @return A new mesh
    std::unique_ptr<Mesh> mesh_from_manifold(const Manifold &manifold);

    /// @brief Create a manifold from a mesh
    /// @param mesh A mesh
    /// @return A new manifold
    std::unique_ptr<Manifold> manifold_from_mesh(const Mesh &mesh);

    /// @brief Create a mesh from vertices and indices
    /// @param vertices Vertices, a slice of floats where each 3 elements represent a vertex position (x, y, z)
    /// @param indices Indices
    /// @return A new mesh
    std::unique_ptr<Mesh> mesh_from_vertices(
        rust::Slice<const float> vertices,
        rust::Slice<const uint32_t> indices);
} // namespace manifold_rs
