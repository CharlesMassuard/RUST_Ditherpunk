fn main() {
    // ouvrir image
    let img = image::open("../imgs/stark.png").unwrap();

   
    // convertir l'image en mode RGB8
    let rgb_img = img.to_rgb8();

    // sauvegarder au format PNG
    rgb_img.save("../imgs/stark_rgb.png").unwrap();

    afficher_couleurs_pixel(&rgb_img, 32, 52);
}

fn afficher_couleurs_pixel(img: &image::RgbImage, x: u32, y: u32) {
    let pixel = img.get_pixel(x, y);
    println!("Les couleurs du pixel ({}, {}) sont : {:?}", x, y, pixel);
}