extern crate manifold_rs;

use manifold_rs::output::WritePly;
use manifold_rs::*;

fn main() -> std::io::Result<()> {
    // Generate a cube and smooth it
    {
        let size = 10.0;
        let manifold = Manifold::cube(size, size, size)
            .intersection(&Manifold::sphere(size * 2.0_f64.sqrt(), 32));

        let manifold = manifold.refine_to_length(1.0);

        manifold.write_ply_to_file("cube.ply")?;
    }

    Ok(())
}
