extern crate manifold_rs;

use manifold_rs::output::WritePly;
use manifold_rs::*;

fn main() -> std::io::Result<()> {
    // Generate a cube
    {
        let size = 10.0;
        let manifold = Manifold::cube(size, size, size);
        manifold.write_ply_to_file("cube.ply")?;
    }

    // Generate a cube with normals
    {
        let size = 10.0;
        let manifold = Manifold::cube(size, size, size).calculate_normals(0, 30.0);
        manifold.write_ply_to_file("cube_normals.ply")?;
    }

    // Generate a dice and smooth it
    {
        let size = 10.0;
        let manifold = Manifold::cube(size, size, size)
            .translate(-size / 2.0, -size / 2.0, -size / 2.0)
            .intersection(&Manifold::sphere(size / 2.0_f64.sqrt(), 64));

        let manifold = manifold
            .calculate_normals(0, 40.0)
            .refine_to_length(0.2)
            .smooth_out(40.0, 1.0);

        manifold.write_ply_to_file("dice.ply")?;
    }

    // Generate tube with normals
    {
        let inner_radius = 0.3;
        let outer_radius = 1.0;
        let height = 2.0;

        let manifold = Manifold::cylinder(outer_radius, outer_radius, height, 32)
            .difference(&Manifold::cylinder(inner_radius, inner_radius, height, 32));
        let manifold = manifold.calculate_normals(0, 40.0);

        manifold.write_ply_to_file("tube_normals.ply")?;
    }

    Ok(())
}
