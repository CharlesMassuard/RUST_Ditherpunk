use image::{RgbImage, Rgb};


use argh::FromArgs;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette rÃ©duite de couleurs.
struct DitherArgs {

    /// le fichier dâentrÃ©e
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode dâopÃ©ration
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de lâimage par seuillage monochrome.
struct OptsSeuil {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de lâimage avec une palette contenant un nombre limitÃ© de couleurs
struct OptsPalette {

    /// le nombre de couleurs Ã  utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}

fn main() {
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;


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
        if (x + y) % 2 == 0 { // Si la somme des coordonnÃ©es est paire
            *pixel = Rgb([255, 255, 255]); // Blanc
        }
    }
}