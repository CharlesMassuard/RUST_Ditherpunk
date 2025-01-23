use image::{RgbImage, Rgb};
use argh::FromArgs;
use colorconv::Color;
use std::str::FromStr;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    Ordered(OptsOrdered),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {
    /// la couleur1 choisie par l'utilisateur pour le seuil monochrome
    #[argh(option)]
    couleur1 : Option<String>,

    /// la couleur2 choisie par l'utilisateur pour le seuil monochrome
    #[argh(option)]
    couleur2 : Option<String>,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize,

    /// palette personnalisée fournie par l'utilisateur sous forme de chaîne (par exemple : "255,0,0;0,255,0;0,0,255" ou "red;yellow;purple")
    #[argh(option)]
    palette: Option<String>,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="ordered")]
/// Rendu de l’image avec la méthode ordered dithering (matrice de Bayer).
struct OptsOrdered {
    /// l'ordre de la matrice de Bayer à utiliser
    #[argh(option, default = "2")]
    ordre: usize,
}

fn get_couleurs_palette() -> Vec<Rgb<u8>> {
    vec![
        Rgb([0, 0, 0]),      // Noir
        Rgb([255, 255, 255]), // Blanc
        Rgb([255, 0, 0]),     // Rouge
        Rgb([0, 255, 0]),     // Vert
        Rgb([0, 0, 255]),     // Bleu
        Rgb([255, 255, 0]),   // Jaune
        Rgb([0, 255, 255]),   // Cyan
        Rgb([255, 0, 255]),   // Magenta
    ]
}

fn get_couleurs_from_args(mode: &Mode) -> (Rgb<u8>, Rgb<u8>) {
    match mode {
        Mode::Seuil(opts) => {
            let couleur1 = opts.couleur1.clone().unwrap_or_else(|| "0,0,0".to_string());
            let couleur2 = opts.couleur2.clone().unwrap_or_else(|| "255,255,255".to_string());

            let couleur1 = parse_rgb(&couleur1);
            let couleur2 = parse_rgb(&couleur2);

            (couleur1, couleur2)
        },
        _ => (Rgb([0, 0, 0]), Rgb([255, 255, 255])),
    }
}

fn parse_rgb(rgb_str: &str) -> Rgb<u8> {
    let parts: Vec<u8> = rgb_str.split(',')
                                .map(|s| s.trim().parse().unwrap_or(0))
                                .collect();
    Rgb([parts[0], parts[1], parts[2]])
}

fn parse_palette(palette_str: &str) -> Vec<Rgb<u8>> {
    if let Some(first_char) = palette_str.chars().next() {
        if !first_char.is_digit(10) {
            return palette_str
                .split(";")
                .map(|color_str| {
                    match Color::from_str(color_str) {
                        Ok(color) => Rgb(color.rgb),
                        Err(e) => {
                            eprintln!("{:?}", e);
                            Rgb([0, 0, 0])
                        }
                    }
                })
                .collect();
        }
    }

    palette_str
        .split(';')
        .map(|color_str| {
            let parts: Vec<u8> = color_str.split(',')
                                          .map(|s| s.trim().parse().unwrap_or(0))
                                          .collect();
            Rgb([parts[0], parts[1], parts[2]])
        })
        .collect()
}

fn build_bayer_matrix(order: usize) -> Vec<Vec<f32>> {
    if order == 0 {
        return vec![vec![0.0]];
    }
    let prev = build_bayer_matrix(order - 1);
    let size = 1 << (order - 1);
    let scale = 1.0 / (size * size) as f32;
    let mut matrix = vec![vec![0.0; size * 2]; size * 2];

    for y in 0..size {
        for x in 0..size {
            let value = prev[y][x] * 4.0;
            matrix[y][x] = value * scale;
            matrix[y][x + size] = (value + 2.0) * scale;
            matrix[y + size][x] = (value + 3.0) * scale;
            matrix[y + size][x + size] = (value + 1.0) * scale;
        }
    }
    matrix
}

fn apply_ordered_dithering(img: &mut RgbImage, bayer_matrix: &[Vec<f32>]) {
    let matrix_size = bayer_matrix.len() as u32;
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let lum = get_luminosite_pixel(img, x, y) / 255.0;
            let threshold = bayer_matrix[(y % matrix_size) as usize][(x % matrix_size) as usize];
            if lum > threshold {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }
}

fn main() {
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    let path_out = args.output.unwrap_or_else(|| "output.png".to_string());

    let img_input = image::open(path_in).unwrap();
    let img_input_rgb = img_input.to_rgb8();
    let mut mut_img_input_rgb = img_input_rgb.clone();

    match args.mode {
        Mode::Seuil(_) => {
            let (couleur1, couleur2) = get_couleurs_from_args(&args.mode);
            traitement_monochrome(&mut mut_img_input_rgb, couleur1, couleur2);
            mut_img_input_rgb.save(path_out).unwrap();
        },
        Mode::Palette(opts) => {
            let palette = if let Some(palette_str) = opts.palette {
                if palette_str.trim().is_empty() {
                    get_couleurs_palette().into_iter().take(opts.n_couleurs).collect()
                } else {
                    parse_palette(&palette_str)
                }
            } else {
                get_couleurs_palette().into_iter().take(opts.n_couleurs).collect()
            };

            traiter_palette(&mut mut_img_input_rgb, &palette);
            mut_img_input_rgb.save(&path_out).expect("Erreur lors de l'enregistrement de l'image");
        },
        Mode::Ordered(opts) => {
            let bayer_matrix = build_bayer_matrix(opts.ordre);
            apply_ordered_dithering(&mut mut_img_input_rgb, &bayer_matrix);
            mut_img_input_rgb.save(path_out).unwrap();
        }
    }
}

fn get_couleurs_pixel(img: &RgbImage, x: u32, y: u32) -> Rgb<u8> {
    *img.get_pixel(x, y)
}

fn pixel_to_white(img: &mut RgbImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]);
        }
    }
}

fn get_luminosite_pixel(img: &mut RgbImage, x: u32, y: u32) -> f32 {
    let pixel = img.get_pixel(x, y);
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    (r + g + b) / 3.0
}

fn traitement_monochrome(img: &mut RgbImage, couleur1: Rgb<u8>, couleur2: Rgb<u8>) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let luminosite = get_luminosite_pixel(img, x, y);
            if luminosite > 128.0 {
                img.put_pixel(x, y, Rgb(couleur2.0));
            } else {
                img.put_pixel(x, y, Rgb(couleur1.0));
            }
        }
    }
}

fn distance_rgb(couleur1: Rgb<u8>, couleur2: Rgb<u8>) -> f32 {
    let r_diff = couleur2[0] as f32 - couleur1[0] as f32;
    let g_diff = couleur2[1] as f32 - couleur1[1] as f32;
    let b_diff = couleur2[2] as f32 - couleur1[2] as f32;
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff).sqrt()
}

fn traiter_palette(img: &mut RgbImage, palette: &[Rgb<u8>]) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = *img.get_pixel(x, y);
            let mut min_distance = f32::MAX;
            let mut best_color = Rgb([0, 0, 0]);

            for color in palette {
                let dist = distance_rgb(pixel, *color);
                if dist < min_distance {
                    min_distance = dist;
                    best_color = *color;
                }
            }
            img.put_pixel(x, y, best_color);
        }
    }
}

fn tramage_aleatoire(img: &mut RgbImage){
    let mut rng = rand::thread_rng(); 
    for y in 0..img.height() {
        for x in 0..img.width() {
            let seuil: f32 = rng.gen();
            let luminosite = get_luminosite_pixel(img, x, y);
            if luminosite / 255.0 > seuil {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }
}
