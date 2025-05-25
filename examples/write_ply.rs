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
            .intersection(&Manifold::sphere(size * 2.0_f64.sqrt(), 32));

        let manifold = manifold.refine_to_length(1.0).calculate_normals(0, 30.0);

        manifold.write_ply_to_file("dice.ply")?;
    }

    Ok(())
}
