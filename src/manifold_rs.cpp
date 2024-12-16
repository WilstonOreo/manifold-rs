// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: Apache-2.0

#include "manifold_rs.h"

#include <manifold/manifold.h>

namespace manifold_rs
{
    Manifold::Manifold() : manifold(std::make_unique<::manifold::Manifold>()) {}
    Manifold::Manifold(::manifold::Manifold &&manifold) : manifold(std::make_unique<::manifold::Manifold>(std::move(manifold))) {}
    Manifold::~Manifold() {}

    std::unique_ptr<Manifold> sphere(double radius, uint32_t circular_segments)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Sphere(radius, circular_segments));
    }

    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Cube({x_size, y_size, z_size}));
    }

    std::unique_ptr<Manifold> cylinder(double radius_low, double radius_height, double height, uint32_t circular_segments)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Cylinder(height, radius_low, radius_height, circular_segments));
    }

    std::unique_ptr<Manifold> union_(const Manifold &a, const Manifold &b)
    {
        return std::make_unique<Manifold>(a.manifold->Boolean(*b.manifold, ::manifold::OpType::Add));
    }

    std::unique_ptr<Manifold> intersection(const Manifold &a, const Manifold &b)
    {
        return std::make_unique<Manifold>(a.manifold->Boolean(*b.manifold, ::manifold::OpType::Intersect));
    }

    std::unique_ptr<Manifold> difference(const Manifold &a, const Manifold &b)
    {
        return std::make_unique<Manifold>(a.manifold->Boolean(*b.manifold, ::manifold::OpType::Subtract));
    }

    Mesh::Mesh() : mesh(std::make_unique<::manifold::Mesh>()) {}

    Mesh::Mesh(::manifold::Mesh &&mesh) : mesh(std::make_unique<::manifold::Mesh>(std::move(mesh))) {}

    Mesh::~Mesh() {}

    std::unique_ptr<std::vector<float>> Mesh::vertices() const
    {
        return std::make_unique<std::vector<float>>(mesh->vertProperties);
    }

    std::unique_ptr<std::vector<uint32_t>> Mesh::indices() const
    {
        return std::make_unique<std::vector<uint32_t>>(mesh->triVerts);
    }

    std::unique_ptr<Mesh> mesh_from_manifold(const Manifold &manifold)
    {
        return std::make_unique<Mesh>(manifold.manifold->GetMeshGL());
    }

    std::unique_ptr<Manifold> manifold_from_mesh(const Mesh &mesh)
    {
        return std::make_unique<Manifold>(::manifold::Manifold(*mesh.mesh));
    }

    std::unique_ptr<Mesh> mesh_from_vertices(
        rust::Slice<const float> vertices,
        rust::Slice<const uint32_t> indices)
    {
        assert(vertices.size() % 6 == 0);
        assert(indices.size() % 3 == 0);
        ::manifold::Mesh mesh;
        mesh.numProp = 6;
        mesh.vertProperties = std::vector<float>(vertices.begin(), vertices.end());
        mesh.triVerts = std::vector<uint32_t>(indices.begin(), indices.end());

        return std::make_unique<Mesh>(std::move(mesh));
    }

} // namespace manifold_rs
