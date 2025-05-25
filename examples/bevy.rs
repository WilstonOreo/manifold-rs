use bevy::{color::palettes::tailwind::*, prelude::*};
#[allow(unused_imports)]
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct ManifoldPlugin;

impl ManifoldPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let tube = Self::cylinder_manifold(1.0, 3.0)
            .boolean_op(
                &Self::cylinder_manifold(0.5, 4.0),
                manifold_rs::BooleanOp::Difference,
            )
            .translate(-1.5, -1.5, 0.0);

        Self::add_manifold(&mut commands, &mut meshes, &mut materials, tube);

        let tube_with_normals = Self::cylinder_manifold(1.0, 3.0)
            .boolean_op(
                &Self::cylinder_manifold(0.5, 4.0),
                manifold_rs::BooleanOp::Difference,
            )
            .calculate_normals(0, 50.0)
            .translate(1.5, 1.5, 0.0);

        Self::add_manifold(
            &mut commands,
            &mut meshes,
            &mut materials,
            tube_with_normals,
        );
    }

    fn add_manifold(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        manifold: manifold_rs::Manifold,
    ) {
        let manifold_mesh_handle: Handle<Mesh> = meshes.add(Self::manifold_to_bevy_mesh(manifold));

        let white_matl = materials.add(Color::srgb(0.5, 0.5, 0.5));
        let hover_matl = materials.add(Color::from(CYAN_300));
        let pressed_matl = materials.add(Color::from(YELLOW_300));

        // Insert mesh
        commands
            .spawn((
                Mesh3d(manifold_mesh_handle),
                MeshMaterial3d(white_matl.clone()),
            ))
            .observe(Self::update_material_on::<Pointer<Over>>(
                hover_matl.clone(),
            ))
            .observe(Self::update_material_on::<Pointer<Out>>(white_matl.clone()))
            .observe(Self::update_material_on::<Pointer<Pressed>>(
                pressed_matl.clone(),
            ))
            .observe(Self::update_material_on::<Pointer<Released>>(
                hover_matl.clone(),
            ));
    }

    // Generate cylinder manifold
    fn cylinder_manifold(d: f64, h: f64) -> manifold_rs::Manifold {
        manifold_rs::Manifold::cylinder(d, d, h, (d * 32.0) as u32)
    }

    /// Returns an observer that updates the entity's material to the one specified.
    fn update_material_on<E>(
        new_material: Handle<StandardMaterial>,
    ) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
        // An observer closure that captures `new_material`. We do this to avoid needing to write four
        // versions of this observer, each triggered by a different event and with a different hardcoded
        // material. Instead, the event type is a generic, and the material is passed in.
        move |trigger, mut query| {
            if let Ok(mut material) = query.get_mut(trigger.target()) {
                material.0 = new_material.clone();
            }
        }
    }

    /// Convert Manifold to bevy mesh
    fn manifold_to_bevy_mesh(manifold: manifold_rs::Manifold) -> Mesh {
        let mesh = manifold.to_mesh();

        let vertices = mesh.vertices();
        let indices = mesh.indices();

        match mesh.num_props() {
            // Vertex without normals
            3 => {
                let vertices = vertices
                    .chunks(3)
                    .map(|c| -> [f32; 3] { c.try_into().expect("Chunk size should be 3") })
                    .collect::<Vec<[f32; 3]>>();

                Mesh::new(
                    bevy::render::mesh::PrimitiveTopology::TriangleList,
                    bevy::asset::RenderAssetUsages::MAIN_WORLD
                        | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
                .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
                .with_duplicated_vertices()
                .with_computed_flat_normals()
            }
            // Vertex with normals
            6 => {
                let normals = vertices
                    .chunks(6)
                    .map(|c| -> [f32; 3] { [c[3], c[4], c[5]] })
                    .collect::<Vec<[f32; 3]>>();

                let vertices = vertices
                    .chunks(6)
                    .map(|c| -> [f32; 3] { [c[0], c[1], c[2]] })
                    .collect::<Vec<[f32; 3]>>();

                Mesh::new(
                    bevy::render::mesh::PrimitiveTopology::TriangleList,
                    bevy::asset::RenderAssetUsages::MAIN_WORLD
                        | bevy::asset::RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
                .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
            }
            num_props => panic!("Invalid property count {num_props}"),
        }
    }
}

impl Plugin for ManifoldPlugin {
    fn build(&self, _app: &mut App) {}
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(ManifoldPlugin)
        .add_systems(Startup, (setup, ManifoldPlugin::setup).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_matl = materials.add(Color::from(GRAY_300));

    // Ground
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d {
                    normal: Dir3::Z,
                    ..Default::default()
                }
                .mesh()
                .size(5.0, 5.0),
            ),
        ),
        MeshMaterial3d(ground_matl),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ));

    // Camera
    commands.spawn((
        Transform::from_xyz(0.0, 5.5, 10.0),
        PanOrbitCamera {
            axis: [Vec3::X, Vec3::Z, -Vec3::Y],
            ..Default::default()
        },
    ));
}
