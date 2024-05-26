
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Ne pas faire de warning si des fonctions ne sont pas appelées
#![warn(dead_code)]

use std::fmt::{Write};
use crate::fichiers as fichiers;
use std::cmp::min;

pub trait vectorisable
{
    fn convertit_en_syntaxe_svg(&self) -> String;
}

#[derive(Clone)]
pub struct Ligne
{
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    couleur: String,
    epaisseur: u32 
}


impl vectorisable for Ligne
{
    fn convertit_en_syntaxe_svg(&self) -> String
    {
        // Exemple: <line x1="0" y1="0" x2="150" y2="200" style="stroke:blue;stroke-width:2" />
        let mut ligne : String = String::new();
        ligne += "<line x1=\"";
        write!(ligne, "{}", self.x1).expect("Erreur dans la conversion (1).");
        ligne += "\" y1=\"";
        write!(ligne, "{}", self.y1).expect("Erreur dans la conversion (2).");
        ligne += "\" x2=\"";
        write!(ligne, "{}", self.x2).expect("Erreur dans la conversion (3).");
        ligne += "\" y2=\"";
        write!(ligne, "{}", self.y2).expect("Erreur dans la conversion (4).");
        ligne += "\" style=\"stroke:";
        ligne += &self.couleur;
        ligne += ";stroke-width:";
        write!(ligne, "{}", self.epaisseur).expect("Erreur dans la conversion (5).");
        ligne += "\"/>";

        return ligne;
    }
}


pub fn cree_fichier_svg(fichier_chemin: &String, hauteur: u32, largeur: u32, figures: &Vec<Box<dyn vectorisable>>)
{
    let mut contenu_vec: Vec<String> = Vec::new();

    // Premiere ligne
    // <svg height="200" width="300" xmlns="http://www.w3.org/2000/svg">
    let mut ligne : String = String::new();
    ligne += "<svg height=\"";
    write!(ligne, "{}", hauteur).expect("Erreur dans la conversion (1).");
    ligne += "\" width=\"";
    write!(ligne, "{}", largeur).expect("Erreur dans la conversion (1).");
    ligne += "\" xmlns=\"http://www.w3.org/2000/svg\">";

    contenu_vec.push(ligne);

    for figure in figures.iter()
    {
        let ligne: String = figure.convertit_en_syntaxe_svg();
        contenu_vec.push(ligne);
    }

    // Dernière ligne
    let ligne: String = "</svg>".to_string();
    contenu_vec.push(ligne);

    fichiers::ecrire_fichier_texte_lignes(&fichier_chemin, &contenu_vec);

}


pub fn cree_fichier_svg_depuis_lignes(fichier_chemin: &String, hauteur: u32, largeur: u32, lignes: Vec<Ligne>)
{
    let mut figures: Vec<Box<dyn vectorisable>> = Vec::new();
    for ligne in lignes
    {
        //lignes.push(Box::new(ligne {x1: x1, y1:y1, x2:x2, y2:y2, couleur: couleur.clone(), epaisseur:epaisseur}));
        let figure: Box<dyn vectorisable> = Box::new(ligne);
        figures.push(figure);
    }
    cree_fichier_svg(&fichier_chemin, hauteur, largeur, &figures);
}


pub fn flocon_koch_recursif(lignes : &Vec<Ligne>, n_iter: u32) -> Vec<Ligne>
{
    let racine_3_sur_2 = f32::sqrt(3.) / 2.;
    if (n_iter == 0) {return (*lignes).clone();}

    let mut lignes_retour = Vec::new();

    for ligne in lignes
    {
        // Transforme chaque segment en 4 segments
        let x1 = ligne.x1;
        let y1 = ligne.y1;
        let x2 = (2. / 3. * (ligne.x1 as f32) + 1. / 3. * (ligne.x2 as f32)) as u32;
        let y2 = (2. / 3. * (ligne.y1 as f32) + 1. / 3. * (ligne.y2 as f32)) as u32;

        let x4 = (1. / 3. * (ligne.x1 as f32) + 2. / 3. * (ligne.x2 as f32)) as u32;
        let y4 = (1. / 3. * (ligne.y1 as f32) + 2. / 3. * (ligne.y2 as f32)) as u32;
        let x5 = ligne.x2;
        let y5 = ligne.y2;

        let dx24: i32 = (racine_3_sur_2 * ((x4 as i32 - x2 as i32) as f32)) as i32;
        let dy24: i32 = (racine_3_sur_2 * ((y4 as i32 - y2 as i32) as f32)) as i32;
        let mx24 = (x4 + x2) / 2;
        let my24 = (y4 + y2) / 2;
        let x3 = (mx24 as i32 + dy24) as u32;
        let y3 = (my24 as i32 - dx24) as u32;


        let couleur = ligne.couleur.clone();
        let epaisseur = ligne.epaisseur;
        lignes_retour.push(Ligne {x1: x1, y1:y1, x2:x2, y2:y2, couleur: couleur.clone(), epaisseur:epaisseur});
        lignes_retour.push(Ligne {x1: x2, y1:y2, x2:x3, y2:y3, couleur: couleur.clone(), epaisseur:epaisseur});
        lignes_retour.push(Ligne {x1: x3, y1:y3, x2:x4, y2:y4, couleur: couleur.clone(), epaisseur:epaisseur});
        lignes_retour.push(Ligne {x1: x4, y1:y4, x2:x5, y2:y5, couleur: couleur.clone(), epaisseur:epaisseur});




    }

    return flocon_koch_recursif(&lignes_retour, n_iter - 1);
}


pub fn flocon_koch(hauteur: u32, largeur: u32, n_iter: u32) -> Vec<Ligne>
{
    let taille: f32 = 0.8;
    let racine_3 = f32::sqrt(3.);
    let couleur: String = "blue".to_string();
    let epaisseur: u32 = 3;
    let mut lignes: Vec<Ligne> = Vec::new();

    let m = min(hauteur, largeur);
    let longueur = (taille * (m as f32)) as u32;
    let x1 = largeur / 2 - longueur / 2;
    let y1 = hauteur / 2 - (longueur as f32 * racine_3 / 6.) as u32;
    let x2 = largeur / 2 + longueur / 2;
    let y2 = y1;
    let y3 = hauteur / 2 + (longueur as f32 * racine_3 * 2. / 6.) as u32;
    let x3 = largeur / 2;
    lignes.push(Ligne {x1: x1, y1:y1, x2:x2, y2:y2, couleur: couleur.clone(), epaisseur:epaisseur});

    lignes.push(Ligne {x1: x2, y1:y2, x2:x3, y2:y3, couleur: couleur.clone(), epaisseur:epaisseur});
    lignes.push(Ligne {x1: x3, y1:y3, x2:x1, y2:y1, couleur: couleur.clone(), epaisseur:epaisseur});

    let lignes_retour = flocon_koch_recursif(&lignes, n_iter);

    return lignes_retour;
}


fn calcule_fractale_interne(x_min: f64, x_max: f64, x_n_step: u32, y_min: f64, y_max: f64, y_n_step: u32, max_n_iter: usize, x_fractale: f64, y_fractale: f64) -> Vec<f64>
{
    let mut pixels: Vec<f64> = Vec::new();

    let x_step = (x_max - x_min) / ((x_n_step - 1) as f64);
    let y_step = (y_max - y_min) / ((y_n_step - 1) as f64);

    for y_index in (0u32..y_n_step)
    {
        let y_cour = y_min + y_step * (y_index as f64);

        for x_index in (0u32..x_n_step)
        {
            let x_cour = x_min + x_step * (x_index as f64);

            let mut x_n : f64 = x_cour;
            let mut y_n : f64 = y_cour;
            let mut norm: f64 = 0.;
            for iter_index in (0..max_n_iter)
            {
                let x_np1: f64 = x_n * x_n - y_n * y_n + x_fractale;
                let y_np1: f64 = 2. * x_n * y_n + y_fractale;
                x_n = x_np1;
                y_n = y_np1;
                norm = f64::sqrt(x_n * x_n + y_n * y_n);
                if (norm >= 2.) {break;}
            }
            norm = (2. - norm) / 2.;
            if (norm < 0.) {norm = 0.;}
            pixels.push(norm);
        }
    }

    return pixels;
}

// Convertit une chaine hexadecimale
// avec des octets séparés par des espaces
fn convertit_str_to_vec_u8(chaine_hexa: &String) -> Vec<u8>
{
    let mut octets_retour : Vec<u8> = Vec::new();
    let octet_hexa_vec = chaine_hexa.split_whitespace().collect::<Vec<_>>();
    for octet_hexa in octet_hexa_vec
    {
        let v = u8::from_str_radix(octet_hexa, 16).expect("Erreur avec u8::from_str_radix(): n'a pu faire la conversion.");
        octets_retour.push(v);
    }

    return octets_retour;
}

// Calcule l'image d'une fractale, et l'écrit dans un fichier bmp
// (Format d'image matriciel non compressé; pas de dépendance à des lib externes)
pub fn calcule_fractale_et_ecrit_bmp(x_fractale: f64, y_fractale: f64, f_fractale_bmp: &String)
{
    let mut bmp_octets: Vec<u8> = Vec::new();

    let x_min = -1.;
    let x_max = 1.;
    let y_min = -1.;
    let y_max = 1.;

    let max_n_iter: usize = 50; 
    //let epaisseur_svg = 1;

    // Entete BMP (640x480) - Résolution harcodée (Fixée dans l'entete BMP)
    // TODO: Trouver un moyen d'écrire une image matricielle, sans lib externe,
    //       et en pouvant choisir ses paramètres (résolution) etc.
    let hauteur = 640;
    let largeur = 480;
    let debut_bmp = "42 4d 36 10 0e 00 00 00 00 00 36 00 00 00 28 00";
    let debut_bmp2 = "00 00 80 02 00 00 e0 01 00 00 01 00 18 00 00 00";
    let debut_bmp3 = "00 00 00 10 0e 00 d7 0d 00 00 d7 0d 00 00 00 00";
    let debut_bmp4 = "00 00 00 00 00 00";
    bmp_octets.append(&mut convertit_str_to_vec_u8(&debut_bmp.to_string()));
    bmp_octets.append(&mut convertit_str_to_vec_u8(&debut_bmp2.to_string()));
    bmp_octets.append(&mut convertit_str_to_vec_u8(&debut_bmp3.to_string()));
    bmp_octets.append(&mut convertit_str_to_vec_u8(&debut_bmp4.to_string()));


    let img_pixels:  Vec<f64> = calcule_fractale_interne(x_min, x_max, largeur, y_min, y_max, hauteur, max_n_iter, x_fractale, y_fractale);

    for x_index in (0u32..largeur)
    {
        for y_index in (0u32..hauteur)
        {
            let pixel_index = (y_index * largeur + x_index) as usize;
            let pixel: f64 = img_pixels[pixel_index];
            //println!("{}", pixel);
            let intensite:u8 = (255. * pixel) as u8;
            //println!("{}", intensite);
            bmp_octets.push(intensite);
            bmp_octets.push(0);
            bmp_octets.push(0);

            //let mut couleur_str: String = String::new();
            //write!(couleur_str, "rgb(0,0,{})", intensite).expect("Erreur dans write");

            //rgb(0,255,0)
            //lignes.push(Ligne {x1: x_index, y1:y_index, x2:x_index, y2:y_index, couleur: couleur_str, epaisseur:epaisseur_svg});
        }
    }

    fichiers::ecrire_fichier_binaire(f_fractale_bmp, &bmp_octets);
    println!("Fichier écrit: {}", &f_fractale_bmp);

}
