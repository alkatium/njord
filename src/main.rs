use njord::geometry::vector::Vector;

use njord::components::color::Color;
use njord::components::image::Image;

use njord::models::scene::Scene;

fn main() {
    // let v1 = Vector {
    //     dx: 10., 
    //     dy:10., 
    //     dz: 10. 
    // };

    // let v2 = Vector {
    //     dx: 30., 
    //     dy: 22., 
    //     dz: 35.5 
    // };

    // println!("Distance between {:?} and {:?}", v1, v2);
    // //println!("is {}", p1.dist(&p2));
    // let v3 = v1 + v2;
    // println!("Vector sum is {:?}", v3);

    // Test creating image
    // let mut image = Image::new(600, 600);
    // image.set_pixel(500, 500, &Color::white());
    // image.set_pixel(501, 500, &Color::white());
    // image.set_pixel(500, 501, &Color::white());
    // image.set_pixel(501, 501, &Color::white());

    // let pixel = image.get_pixel(500, 500);
    // println!("{:?}", pixel);

    // image.save("test_image.ppm");

    let scene = Scene::init("scene0.txt");
}
