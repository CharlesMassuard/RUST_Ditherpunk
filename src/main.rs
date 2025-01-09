use image::{RgbImage, Rgb};

fn main() {
    // ouvrir image
    let img = image::open("../imgs/stark.png").unwrap();

   
    // convertir l'image en mode RGB8
    let rgb_img = img.to_rgb8();

    // sauvegarder au format PNG
    rgb_img.save("../imgs/stark_rgb.png").unwrap();

    afficher_couleurs_pixel(&rgb_img, 32, 52);

    //Passage de 1 pixel sur 2 en blanc
    let img_tyrion = image::open("../imgs/tyrion.jpg").unwrap();
    let mut img_tyrion_rgb = img_tyrion.to_rgb8();
    pixel_to_white(&mut img_tyrion_rgb);
    img_tyrion_rgb.save("../imgs/tyrion_white.png").unwrap();
}

fn afficher_couleurs_pixel(img: &image::RgbImage, x: u32, y: u32) {
    let pixel = img.get_pixel(x, y);
    println!("Les couleurs du pixel ({}, {}) sont : {:?}", x, y, pixel);
}

fn pixel_to_white(img: &mut RgbImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 { // Si la somme des coordonnées est paire
            *pixel = Rgb([255, 255, 255]); // Blanc
        }
    }
}