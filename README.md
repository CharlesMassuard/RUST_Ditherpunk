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
fn afficher_couleurs_pixel(img: &image::RgbImage, x: u32, y: u32) {
    let pixel = img.get_pixel(x, y);
    println!("Les couleurs du pixel ({}, {}) sont : {:?}", x, y, pixel);
}
```  

Avec ```img = rgb_img (stark_rgb.png)```, ```x = 32``` et ```y = 52```, on obtient :  

```Les couleurs du pixel (32, 52) sont : Rgb([20, 20, 22])```

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

L'image obtenue est reconnaissable.

![Image de base](./imgs/tyrion.jpg)
<br>
*Image de base*

![Image obtenue ayant 1 pixel sur 2 en blanc](./imgs/tyrion_white.png)
<br>
*Image obtenue ayant 1 pixel sur 2 en blanc*

!["Image montrant qu'un pixel sur deux est blanc](./imgs/preuve_un_pixel_sur_deux_blanc.png)
<br>
*Image montrant qu'un pixel sur deux est blanc*