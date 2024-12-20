// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: Apache-2.0

#include "manifold_rs.h"

#include <manifold/manifold.h>

namespace manifold_rs
{
    Polygons::Polygons() : polygons(std::make_unique<::manifold::Polygons>()) {}
    Polygons::Polygons(::manifold::Polygons &&polygons) : polygons(std::make_unique<::manifold::Polygons>(std::move(polygons))) {}
    Polygons::~Polygons() {}

    size_t Polygons::size() const
    {
        return polygons->size();
    }

    rust::Slice<const double> Polygons::get_as_slice(size_t index) const
    {
        auto &polygon = (*polygons)[index];
        return rust::Slice<const double>(static_cast<double *>((void *)polygon.data()), polygon.size() * 2);
    }

    Manifold::Manifold() : manifold(std::make_unique<::manifold::Manifold>()) {}
    Manifold::Manifold(::manifold::Manifold &&manifold) : manifold(std::make_unique<::manifold::Manifold>(std::move(manifold))) {}
    Manifold::~Manifold() {}

    std::unique_ptr<Polygons> Manifold::slice(double height) const
    {
        return std::make_unique<Polygons>(manifold->Slice(height));
    }

    std::unique_ptr<Polygons> Manifold::project() const
    {
        return std::make_unique<Polygons>(manifold->Project());
    }

    std::unique_ptr<Manifold> tetrahedron()
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Tetrahedron());
    }

    std::unique_ptr<Manifold> cube(double x_size, double y_size, double z_size)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Cube({x_size, y_size, z_size}));
    }

    std::unique_ptr<Manifold> sphere(double radius, uint32_t circular_segments)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Sphere(radius, circular_segments));
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
        auto mesh = manifold.manifold->GetMeshGL(0);
        assert(mesh.numProp == 3);
        return std::make_unique<Mesh>(std::move(mesh));
    }

    std::unique_ptr<Manifold> manifold_from_mesh(const Mesh &mesh)
    {
        return std::make_unique<Manifold>(::manifold::Manifold(*mesh.mesh));
    }

    std::unique_ptr<Mesh> mesh_from_vertices(
        rust::Slice<const float> vertices,
        rust::Slice<const uint32_t> indices)
    {
        assert(vertices.size() % 3 == 0);
        assert(indices.size() % 3 == 0);
        ::manifold::Mesh mesh;
        mesh.numProp = 3;
        mesh.vertProperties = std::vector<float>(vertices.begin(), vertices.end());
        mesh.triVerts = std::vector<uint32_t>(indices.begin(), indices.end());

        return std::make_unique<Mesh>(std::move(mesh));
    }

    ::manifold::Polygons to_polygons(rust::Slice<const rust::Slice<const double>> multi_polygon_data)
    {
        ::manifold::Polygons polygons;
        for (auto &polygon_data : multi_polygon_data)
        {
            assert(polygon_data.size() % 2 == 0);

            // Create SimplePolygon from rust::Slice<const float> via memcpy
            // without using a loop
            ::manifold::SimplePolygon p(polygon_data.size() / 2);
            memcpy(p.data(), polygon_data.data(), polygon_data.size() * sizeof(double));
            polygons.push_back(p);
        }
        return polygons;
    }

    std::unique_ptr<Manifold> extrude(
        rust::Slice<const rust::Slice<const double>> multi_polygon_data,
        double height, uint32_t divisions, double twist_degrees, double scale_top_x, double scale_top_y)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Extrude(to_polygons(multi_polygon_data), height, divisions, twist_degrees, {scale_top_x, scale_top_y}));
    }

    std::unique_ptr<Manifold> revolve(
        rust::Slice<const rust::Slice<const double>> multi_polygon_data,
        uint32_t circular_segments, double revolve_degrees)
    {
        return std::make_unique<Manifold>(::manifold::Manifold::Revolve(to_polygons(multi_polygon_data), circular_segments, revolve_degrees));
    }

} // namespace manifold_rs
