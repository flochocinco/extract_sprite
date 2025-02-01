use std::{env};
use image::{open, ImageBuffer, Rgba};

/*
 * cargo build; cargo run -- resources/manga_girl_walking.png 8 1 
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Expecting argument: cargo run -- file/to/image number_split_horizontal number_split_vertical");
        return;
    }

    let path = args.get(1).unwrap();

    // Charger l'image depuis un fichier
    let img = open(path).expect("Échec de l'ouverture de l'image");

    // Convertir l'image en RGBA (c'est ici que tu peux travailler avec les pixels RGBA)
    let img = img.to_rgba8(); // `to_rgba8()` pour obtenir une image RGBA

    // Obtenir les dimensions de l'image
    let (width, height) = img.dimensions();
    println!("Dimensions de l'image d'origine: {}x{}", width, height);

    let split_h = args.get(2).unwrap().parse::<u32>().unwrap();
    let split_v = args.get(3).unwrap().parse::<u32>().unwrap();
    let new_height : u32 = height / split_v;
    let new_width : u32 = width / split_h;

    println!("Creating {} images of size {}x{}", split_h*split_v, new_width, new_height);

    let mut cpt_x = 0;
    let mut cpt_y = 0;

    for cpt in 0..split_h*split_v{
        let mut dest_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(new_width, new_height);
        for y in 0..new_height{
            for x in 0..new_width{
                let pixel  = img.get_pixel(x+cpt_x*new_width, y +cpt_y*new_height).0;
                let new_pixel: Rgba<u8> = Rgba(pixel);
                dest_img.put_pixel(x, y, new_pixel);
            }
        }
        cpt_x += 1;
        if cpt_x >= split_h{
            cpt_y += 1;
            cpt_x = 0;
        }
        let name = format!("output{}.png", cpt);
        dest_img.save(&name).expect("Echec de l'enregistrement");
        println!("{} saved", name);
    }

    println!("Image Saved!")


}
