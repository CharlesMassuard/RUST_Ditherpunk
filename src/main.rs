use image::{RgbImage, Rgb};
use argh::FromArgs;

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

    /// palette personnalisée fournie par l'utilisateur sous forme de chaîne (par exemple : "255,0,0;0,255,0;0,0,255")
    #[argh(option)]
    palette: Option<String>,
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

// Fonction pour récupérer les couleurs à partir des arguments
fn get_couleurs_from_args(mode: &Mode) -> (Rgb<u8>, Rgb<u8>) {
    match mode {
        Mode::Seuil(opts) => {
            let couleur1 = opts.couleur1.clone().unwrap_or_else(|| "0,0,0".to_string()); // Valeur par défaut noire
            let couleur2 = opts.couleur2.clone().unwrap_or_else(|| "255,255,255".to_string()); // Valeur par défaut blanche

            // Conversion des chaînes en Rgb
            let couleur1 = parse_rgb(&couleur1);
            let couleur2 = parse_rgb(&couleur2);

            (couleur1, couleur2)
        },
        _ => (Rgb([0, 0, 0]), Rgb([255, 255, 255])), // Valeurs par défaut si pas de couleur spécifiée
    }
}

// Fonction pour convertir une chaîne de caractères en Rgb<u8>
fn parse_rgb(rgb_str: &str) -> Rgb<u8> {
    let parts: Vec<u8> = rgb_str.split(',')
                                .map(|s| s.trim().parse().unwrap_or(0)) // Parse chaque partie en u8
                                .collect();
    Rgb([parts[0], parts[1], parts[2]])
}


fn parse_palette(palette_str: &str) -> Vec<Rgb<u8>> {
    palette_str
        .split(';')  // Séparer par des points-virgules (chaque couleur est séparée par un point-virgule)
        .map(|color_str| {
            let parts: Vec<u8> = color_str.split(',')
                                          .map(|s| s.trim().parse().unwrap_or(0)) // Convertir chaque composant de couleur en u8
                                          .collect();
            Rgb([parts[0], parts[1], parts[2]]) // Retourner un objet Rgb
        })
        .collect()
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
                parse_palette(&palette_str)
            } else {
                // Si aucune palette n'est fournie, utiliser une palette par défaut
                get_couleurs_palette().into_iter().take(opts.n_couleurs).collect()
            };

            // Vérifier si la palette est vide ou incorrecte
            if palette.is_empty() {
                eprintln!("Erreur : la palette ne peut pas être vide.");
                return;
            }

            traiter_palette(&mut mut_img_input_rgb, &palette);
            mut_img_input_rgb.save(path_out).unwrap();
        }
    }
}


fn get_couleurs_pixel(img: &RgbImage, x: u32, y: u32) -> Rgb<u8> {
    *img.get_pixel(x, y)
}

fn pixel_to_white(img: &mut RgbImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 { // Si la somme des coordonnées est paire
            *pixel = Rgb([255, 255, 255]); // Blanc
        }
    }
}

fn get_luminosite_pixel(img: &mut RgbImage, x: u32, y: u32) -> f32 {
    let pixel = img.get_pixel(x, y);
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    return (r+g+b) / 3.0;
}

fn traitement_monochrome(img: &mut RgbImage, couleur1: Rgb<u8>, couleur2: Rgb<u8>){
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
            // Trouver la couleur la plus proche dans la palette
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