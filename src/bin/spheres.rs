use dirt::spheres::Spheres;
use dirt::Vector3;
use kiss3d::light::Light;
use kiss3d::nalgebra::Translation3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

fn add_sphere(window: &mut Window, radius: f32, translation: Translation3<f32>) -> SceneNode {
    let mut sphere = window.add_sphere(radius);
    sphere.set_local_translation(translation);
    sphere
}

fn to_translation(vector: &Vector3) -> Translation3<f32> {
    Translation3::new(vector.0, vector.1, vector.2)
}

fn main() {
    let mut window = Window::new("spheres");
    window.set_light(Light::StickToCamera);
    let mut physics_spheres = Spheres {
        positions: vec![Vector3(0., 0., 0.), Vector3(10., 0., 0.)],
        velocities: vec![Vector3(1., 0., 0.), Vector3(0., 0., 0.)],
    };
    let mut rendered_spheres: Vec<_> = physics_spheres
        .positions
        .iter()
        .map(|position| add_sphere(&mut window, 1., to_translation(position)))
        .collect();

    let timestep = dirt::Timestep(0.01);
    while window.render() {
        physics_spheres.update_positions(timestep);
        std::iter::zip(physics_spheres.positions.iter(), &mut rendered_spheres).for_each(
            |(position, rendered)| rendered.set_local_translation(to_translation(&position)),
        );
    }
}
