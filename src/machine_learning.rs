
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Ne pas faire de warning si des fonctions ne sont pas appelées
#![warn(dead_code)]

use std::vec;
use std::str::FromStr;
use std::fmt::Write;
use crate::fichiers as fichiers;
use crate::probabilites as probabilites;

// Algorithmes de machine learning (clustering)

pub fn calcule_similarite_cosinus(f_csv_in: String, f_similarite_cosinus_out: String)
{
    if (fichiers::test_existence_fichier(&f_similarite_cosinus_out))
    {
        return;
    }

    // 1) Charger le fichier .csv
    let lignes: Vec<String> = fichiers::lire_fichier_texte_lignes(&f_csv_in, None);
    
    let nb_vecteurs: usize = lignes[0].chars().filter(|c| *c == ';').count() + 1;
    let nb_composantes: usize = lignes.len() - 1;  // Entete pas pris en compte
    println!("Nombre de vecteurs:{}", nb_vecteurs);
    println!("Nombre de composantes:{}", nb_composantes);

    // 2) Reconstruire les vecteurs formés de 768 composantes
    // On les stocke 'stackés' de façon contigue en mémoire (vecteur1, vecteur2..)
    let mut vecteurs: Vec<f64> = Vec::new();  // 768 premiers indices = Premier vecteur, puis second vecteur etc.
    vecteurs.resize(nb_vecteurs * nb_composantes, 0f64);
    for (ligne_index, ligne) in (lignes[1..]).into_iter().enumerate()
    {
        //println!("ligne_index:{}, longueur:{}", ligne_index, ligne.len());
        let composante_index = ligne_index;
        let champs_list = ligne.split(";");
        for (champs_index, champs) in champs_list.enumerate()
        {
            let vecteur_index = champs_index;
            let index = nb_composantes * vecteur_index + composante_index;
            let valeur: f64 = f64::from_str(champs).expect("Erreur dans la conversion");
            vecteurs[index] = valeur
        }
    }

    // 3) Centrer-réduire
    for vecteur_index in (0..nb_vecteurs)
    {
        let v_index_min = vecteur_index * nb_composantes;
        let v_index_max = (vecteur_index + 1) * nb_composantes;

        // 3a) Centrer
        let m: f64 = probabilites::moyenne(&vecteurs[v_index_min..v_index_max]).expect("Vecteur vide");
        for component_index in (0..nb_composantes)
        {
            vecteurs[v_index_min + component_index] -= m;
        }
        
        // 3b) Réduire
        let racine_768: f64 = f64::sqrt(nb_composantes as f64);
        let correction: Option<usize> = Some(0); // delta_n = 0 => Variance biaisée classique
        let v: f64 = probabilites::variance(&vecteurs[v_index_min..v_index_max], correction).expect("Vecteur vide");
        let f: f64 = 1. / racine_768 / f64::sqrt(v);
        for component_index in (0..nb_composantes)
        {
            vecteurs[v_index_min + component_index] *= f;
        }
    }

    // Calculer les similarités-cosinus, en faisant simplement les produits scalaires
    println!("Calcul des similarités cosinus..");
    let mut similarites_cosinus: Vec<f64> = Vec::new();
    for vecteur_index in (0..nb_vecteurs)
    {
        let v_index_min = vecteur_index * nb_composantes;
        let v_index_max = (vecteur_index + 1) * nb_composantes;
        //println!("vecteur_index:{}", vecteur_index);
        for vecteur_index2 in (0..vecteur_index)
        {
            let v_index2_min = vecteur_index2 * nb_composantes;
            let v_index2_max = (vecteur_index2 + 1) * nb_composantes;

            let mut s: f64 = 0.;
            for component_index in (0..nb_composantes)
            {
                s += vecteurs[v_index_min + component_index] * vecteurs[v_index2_min + component_index];
            }
            //println!("{}", s);
            similarites_cosinus.push(s);
        }
        //println!("\n");
        if (vecteur_index > 100) {break;}  // On étudie seulement les 100 premières composantes pour l'instant
    }
    println!("Calcul Terminé..");

    let mut similarites_cosinus_str: Vec<String> = Vec::new();
    for sim_cos in similarites_cosinus
    {
        let mut s: String = String::new();;
        write!(s, "{}", sim_cos).expect("Erreur dans write");
        similarites_cosinus_str.push(s);
    }

    fichiers::ecrire_fichier_texte_lignes(&f_similarite_cosinus_out, &similarites_cosinus_str);

}


// DBSCAN
// https://fr.wikipedia.org/wiki/DBSCAN

