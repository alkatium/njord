use njord::components::image::Image;

use njord::models::scene::Scene;
use njord::models::camera::Camera;

fn main() {

    // image.save("test_image.ppm");

    let mut scene = Scene::init("scene0.txt");

    let camera = Camera::default();
    let mut image = Image::new(600, 600);

    camera.generer_image(&mut scene, &mut image);

    image.save("njord.ppm");
}
