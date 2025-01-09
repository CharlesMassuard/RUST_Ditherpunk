# VILLAIN Théo & MASSUARD Charles

> *Les commandes (cargo build, cargo run) se faire dans **RUST_Ditherpunk/src** afin de fonctionner correctement.*

## Partie 1

### Question 2

- Ouverture de l'image :
```rs
let img = image::open("../imgs/tyrion.jpg").unwrap();
```

Ce type correspond à une image pouvant avoir différents formats de pixels et canaux.

- Pour obtenir une image en mode rgb8 :
```rs
let rgb_img = img.to_rgb8();
```

- Ouverture de l'image passée en argument :
```rs
let args: DitherArgs = argh::from_env();
let path_in = args.input;
let path_out = args.output.unwrap_or_else(|| "output.png".to_string());

let img_input = image::open(path_in).unwrap();
```

### Question 3

Si l'image de départ a un canal alpha *(stark.png)*, l'image rgb8 sauvegardée *(stark_rgb.png)* n'a plus le canal alpha et donc sa transparence est supprimée. Nous remarquons qu'une partie de la transparence, ici l'arrière-plan, est remplie en blanc, tandis qu'une autre partie a des nuances de couleurs prenant en compte les couleurs de l'image de base.

![Image de base avec un canal alpha](./imgs/stark.png)
<br>
*Image de base avec un canal alpha*

![Image rgb8 obtenue](./imgs/stark_rgb.png)
<br>
*Image rgb8 obtenue*

### Question 4

```rs
fn get_couleurs_pixel(img: &RgbImage, x: u32, y: u32) -> Rgb<u8>{
    *img.get_pixel(x, y)
}
```  

Avec ```img = rgb_img (stark_rgb.png)```, ```x = 32``` et ```y = 52```, on obtient :  

```Rgb([20, 20, 22])```

### Question 5

```rs
fn pixel_to_white(img: &mut RgbImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 { // Si la somme des coordonnées est paire
            *pixel = Rgb([255, 255, 255]); // Blanc
        }
    }
}
```

*Note : Afin de modifier les datas d'une image, __&mut__ doit être utilisé. Ainsi, afin de lancer une fonction modifiant les datas de l'image telle que celle-ci, il faut tout d'abord la cloner en tant que mut et appeler la fonction avec :*

```rs
let mut mut_img_input_rgb = img_input_rgb.clone();
pixel_to_white(&mut mut_img_input_rgb);
```

L'image obtenue est reconnaissable.

![Image de base](./imgs/tyrion.jpg)
<br>
*Image de base*

![Image obtenue ayant 1 pixel sur 2 en blanc](./imgs/tyrion_white.png)
<br>
*Image obtenue ayant 1 pixel sur 2 en blanc*

!["Image montrant qu'un pixel sur deux est blanc"](./imgs/preuve_un_pixel_sur_deux_blanc.png)
<br>
*Image montrant qu'un pixel sur deux est blanc*


## Partie 2

### Question 6

Afin de récupérer la luminosité d'un pixel, nous pouvons la calculer en calculant la moyenne de ces valeurs RGB :

$$ luminositeMoyenne = {R + G + B \over 3} $$

En Rust, cela correspond à :

```rs
fn get_luminosite_pixel(img: $image::RgbImage, x: u32, y: u32) -> f32 {
    let pixel = img.get_pixel(x, y);
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    return (r+g+b) / 3.0;
}
```

Avec ```img = rgb_img (stark_rgb.png)```, ```x = 32``` et ```y = 52```, on obtient :  

```Luminosité du pixel en 32, 52 : 20.666666```

### Question 7

```rs
fn traitement_monochrome(img: &mut RgbImage){
    for y in 0..img.height() {
        for x in 0..img.width() {
            let luminosite = get_luminosite_pixel(img, x, y);
            if luminosite > 128.0 {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
    }
}
```

!["Image de Tyrion après le traitement monochrome en noir et blanc"](./imgs/tyrion_monochrome.png)
<br>
*Image de Tyrion après le traitement monochrome en noir et blanc*