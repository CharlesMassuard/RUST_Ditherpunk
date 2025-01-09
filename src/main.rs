fn main() {
    // ouvrir image
    let img = image::open("../imgs/stark.png").unwrap();

   
    // convertir l'image en mode RGB8
    let rgb_img = img.to_rgb8();

    // sauvegarder au format PNG
    rgb_img.save("../imgs/stark_rgb.png").unwrap();
}