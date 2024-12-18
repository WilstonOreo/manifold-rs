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
    /// @brief Wrapper around manifold::Manifold
    /// @details This class will be exposed to Rust
    class Manifold
    {
    public:
        Manifold();
        Manifold(::manifold::Manifold &&manifold);
        ~Manifold();

        std::unique_ptr<::manifold::Manifold> manifold;
    };

    /// @brief Create a sphere
    /// @param radius Radius of the sphere
    /// @param circular_segments  Number of circular segments
    /// @return A new sphere as a Manifold
    std::unique_ptr<Manifold> sphere(double radius, uint32_t circular_segments);

    /// @brief Create a cube
    /// @param x_size A size of the cube in x direction
    /// @param y_size A size of the cube in y direction
    /// @param z_size A size of the cube in z direction
    /// @return A new cube as a Manifold
    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size);

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

    /// @brief A mesh, which is a collection of vertices and indices
    /// @details This class will be exposed to Rust
    class Mesh
    {
    public:
        Mesh();
        Mesh(::manifold::Mesh &&mesh);
        ~Mesh();

        /// @brief Get the vertices of the mesh
        /// @details The vertex coefficients are in the following order:
        ///          * position x, y, z
        ///          * normal x, y, z
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
    /// @param vertices Vertices, interleaved in the following order: position x, y, z, normal x, y, z
    /// @param indices Indices
    /// @return A new mesh
    std::unique_ptr<Mesh> mesh_from_vertices(
        rust::Slice<const float> vertices,
        rust::Slice<const uint32_t> indices);
}
