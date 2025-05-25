extern crate manifold_rs;

use manifold_rs::output::WriteStl;

fn generate_circle(radius: f64, offset: (f64, f64), segments: usize) -> Vec<f64> {
    let mut circle = Vec::new();
    for i in 0..segments {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
        circle.append(&mut vec![
            radius * angle.cos() + offset.0,
            radius * angle.sin() + offset.1,
        ]);
    }
    circle
}

fn main() -> std::io::Result<()> {
    // Write sphere to an STL file
    manifold_rs::Manifold::sphere(4.0, 128).write_stl_to_file("sphere.stl")?;

    // Write cylinder to an STL file
    {
        let manifold = manifold_rs::Manifold::cylinder(1.0, 4.0, 3.0, 32);

        // Convert the manifold to a mesh and back to a manifold, just for testing
        let mesh = manifold.to_mesh();
        let manifold = mesh.to_manifold();

        manifold.write_stl_to_file("cylinder.stl")?;
    }

    // Generate torus with `revolve` and write resulting mesh to an STL file
    {
        // Generate circle with 32 vertices
        let circle = generate_circle(2.0, (4.0, 0.0), 32);

        // Revolve the circle 360° around the z-axis
        let manifold = manifold_rs::Manifold::revolve(&[circle.as_slice()], 32, 360.0);

        manifold.write_stl_to_file("torus.stl")?;
    }

    // Generate a tube via `extrude` and write resulting mesh to an STL file
    {
        // Generate circle with 32 vertices
        let inner_circle = generate_circle(0.3, (0.0, 0.0), 32);
        let outer_circle = generate_circle(1.0, (0.0, 0.0), 32);

        // CCW winding order to create a hole in the tube
        let inner_circle = inner_circle
            .into_iter()
            .enumerate()
            .map(|(i, x)| if i % 2 == 0 { x } else { -x })
            .collect::<Vec<_>>();

        // Extrude the circle along the z-axis
        let manifold = manifold_rs::Manifold::extrude(
            &[outer_circle.as_slice(), inner_circle.as_slice()],
            4.0,
            16,
            0.0,
            1.0,
            1.0,
        );

        manifold.write_stl_to_file("tube.stl")?;
    }

    // Convex hull of two circles
    {
        let left_circle = generate_circle(1.0, (-1.0, 0.0), 32);
        let right_circle = generate_circle(1.0, (1.0, 0.0), 32);

        // Extrude the circle along the z-axis
        let manifold = manifold_rs::Manifold::extrude(
            &[left_circle.as_slice(), right_circle.as_slice()],
            4.0,
            16,
            0.0,
            1.0,
            1.0,
        );
        let manifold = manifold.hull();

        manifold.write_stl_to_file("hull.stl")?;
    }

    // Trim a cylinder by a plane
    {
        let manifold = manifold_rs::Manifold::cylinder(1.0, 1.0, 3.0, 32);
        let manifold = manifold.trim_by_plane(0.5, 0.5, 0.5, 0.0);

        manifold.write_stl_to_file("cylinder_trimmed.stl")?;
    }
    Ok(())
}
