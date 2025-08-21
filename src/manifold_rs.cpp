// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: Apache-2.0

#include "manifold_rs.h"

#include <manifold/manifold.h>
#include <cstring> // memcpy

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

    bool Manifold::is_empty() const
    {
        return manifold->IsEmpty();
    }

    std::unique_ptr<Polygons> Manifold::slice(double height) const
    {
        return std::make_unique<Polygons>(manifold->Slice(height));
    }

    std::unique_ptr<Polygons> Manifold::project() const
    {
        return std::make_unique<Polygons>(manifold->Project());
    }

    std::unique_ptr<Manifold> Manifold::trim_by_plane(double x, double y, double z, double offset) const
    {
        return std::make_unique<Manifold>(manifold->TrimByPlane({x, y, z}, offset));
    }

    std::unique_ptr<Manifold> Manifold::hull() const
    {
        return std::make_unique<Manifold>(manifold->Hull());
    }

    std::unique_ptr<Manifold> Manifold::translate(double x, double y, double z) const
    {
        return std::make_unique<Manifold>(manifold->Translate({x, y, z}));
    }

    std::unique_ptr<Manifold> Manifold::scale(double x, double y, double z) const
    {
        return std::make_unique<Manifold>(manifold->Scale({x, y, z}));
    }
    std::unique_ptr<Manifold> Manifold::rotate(double x_degrees, double y_degrees, double z_degrees) const
    {
        return std::make_unique<Manifold>(manifold->Rotate(x_degrees, y_degrees, z_degrees));
    }

    std::unique_ptr<Manifold> Manifold::refine(int32_t n) const
    {
        return std::make_unique<Manifold>(manifold->Refine(n));
    }

    std::unique_ptr<Manifold> Manifold::refine_to_length(double t) const
    {
        return std::make_unique<Manifold>(manifold->RefineToLength(t));
    }

    std::unique_ptr<Manifold> Manifold::refine_to_tolerance(double t) const
    {
        return std::make_unique<Manifold>(manifold->RefineToTolerance(t));
    }

    std::unique_ptr<Manifold> Manifold::smooth_by_normals(std::int32_t n) const
    {
        return std::make_unique<Manifold>(manifold->SmoothByNormals(n));
    }

    std::unique_ptr<Manifold> Manifold::smooth_out(double min_sharp_angle, double min_smoothness) const
    {
        return std::make_unique<Manifold>(manifold->SmoothOut(min_sharp_angle, min_smoothness));
    }

    /// Calculate normals for the manifold and return a new one.
    std::unique_ptr<Manifold> Manifold::calculate_normals(std::int32_t normal_idx, double min_sharp_angle) const
    {
        return std::make_unique<Manifold>(manifold->CalculateNormals(normal_idx, min_sharp_angle));
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

    std::uint32_t Mesh::num_props() const
    {
        return mesh->numProp;
    }

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
            memcpy((void *)p.data(), polygon_data.data(), polygon_data.size() * sizeof(double));
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
