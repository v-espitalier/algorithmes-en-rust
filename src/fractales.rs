
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
pub struct ligne
{
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    couleur: String,
    epaisseur: u32 
}

impl vectorisable for ligne
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

pub fn cree_fichier_svg(fichier_chemin: String, hauteur: u32, largeur: u32, figures: &Vec<Box<dyn vectorisable>>)
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

pub fn flocon_koch_recursif(lignes : &Vec<ligne>, n_iter: u32) -> Vec<ligne>
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
        lignes_retour.push(ligne {x1: x1, y1:y1, x2:x2, y2:y2, couleur: couleur.clone(), epaisseur:epaisseur});
        lignes_retour.push(ligne {x1: x2, y1:y2, x2:x3, y2:y3, couleur: couleur.clone(), epaisseur:epaisseur});
        lignes_retour.push(ligne {x1: x3, y1:y3, x2:x4, y2:y4, couleur: couleur.clone(), epaisseur:epaisseur});
        lignes_retour.push(ligne {x1: x4, y1:y4, x2:x5, y2:y5, couleur: couleur.clone(), epaisseur:epaisseur});




    }

    return flocon_koch_recursif(&lignes_retour, n_iter - 1);
}

pub fn flocon_koch(hauteur: u32, largeur: u32, n_iter: u32) -> Vec<ligne>
{
    let taille: f32 = 0.8;
    let racine_3 = f32::sqrt(3.);
    let couleur: String = "blue".to_string();
    let epaisseur: u32 = 3;
    let mut lignes: Vec<ligne> = Vec::new();

    let m = min(hauteur, largeur);
    let longueur = (taille * (m as f32)) as u32;
    let x1 = largeur / 2 - longueur / 2;
    let y1 = hauteur / 2 - (longueur as f32 * racine_3 / 6.) as u32;
    let x2 = largeur / 2 + longueur / 2;
    let y2 = y1;
    let y3 = hauteur / 2 + (longueur as f32 * racine_3 * 2. / 6.) as u32;
    let x3 = largeur / 2;
    lignes.push(ligne {x1: x1, y1:y1, x2:x2, y2:y2, couleur: couleur.clone(), epaisseur:epaisseur});

    lignes.push(ligne {x1: x2, y1:y2, x2:x3, y2:y3, couleur: couleur.clone(), epaisseur:epaisseur});
    lignes.push(ligne {x1: x3, y1:y3, x2:x1, y2:y1, couleur: couleur.clone(), epaisseur:epaisseur});

    let lignes_retour = flocon_koch_recursif(&lignes, n_iter);

    return lignes_retour;
}

