
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Ne pas faire de warning si des fonctions ne sont pas appelées
#![warn(dead_code)]

use std::fs::{self, read_dir, FileType};
use std::path::Path;
use std::io::ErrorKind;
use std::fs::File;
use std::io::Write;

pub fn test_existence_fichier(fichier_chemin: &String) -> bool {
    let existe: bool = Path::new(&fichier_chemin).exists();
    return existe;
}

pub fn lire_fichier_texte(fichier_chemin: &String) -> String
{
    // Ouvrir le fichier texte
    // Voir: https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    // et: https://doc.rust-lang.org/std/fs/struct.File.html
    let contenu = fs::read_to_string(fichier_chemin)
        .expect("Fichier introuvable");

    return contenu;
}

pub fn ecrire_fichier_texte(fichier_chemin: &String, contenu: &String)
{
    // Voir: https://doc.rust-lang.org/std/fs/struct.File.html
    let mut fichier = File::create(fichier_chemin).expect("Erreur: N'a pas pu créer le fichier.");
    fichier.write_all(contenu.as_bytes()).expect("Erreur: N'a pas pu écrire dans le fichier.");
}



pub fn lire_fichier_texte_lignes(fichier_chemin: &String, separateur_opt: Option<&str>) -> Vec<String>
{
    let mut lignes_retour: Vec<String> = Vec::new();

    // Lire le fichier texte
    let contenu = lire_fichier_texte(fichier_chemin);

    // Le lire ligne par ligne et nourrir la structure de sortie
    let separateur_defaut = "\n";
    let contenu_split;
    match (separateur_opt) {

        None =>
        { contenu_split = contenu.split(separateur_defaut); },

        Some(separateur) =>
        { contenu_split = contenu.split(separateur); },
    }

    lignes_retour = contenu_split.map(|s| s.to_string()).collect();

    return lignes_retour;
}


// Voir: https://stackoverflow.com/questions/66577339/collect-file-names-into-vecstr
pub fn liste_dossier(dossier_chemin: &String) -> Vec<String>
{
    let mut liste_fichiers_retour: Vec<String> = Vec::new();

    let paths_res = read_dir(dossier_chemin);
    match (paths_res)
    {
        Err(erreur) if erreur.kind() == ErrorKind::NotFound => {panic!("Dossier non trouvé");},
        Err(erreur) => {panic!("Erreur inattendue: {:?}", erreur)},
        Ok(resultat) => {liste_fichiers_retour = resultat.filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().into_owned())
            //.collect()
            .collect::<Vec<_>>();},
    }

    return liste_fichiers_retour;
}


#[derive(Debug)]
pub enum TypeFichier {
    FichierRegulier,
    Dossier,
    LienSymbolique,
    }

pub fn donne_type_fichier(fichier_chemin: &String) -> TypeFichier
{

    let metadata = fs::metadata(fichier_chemin).expect("Fichier non trouvé.");
    let file_type = metadata.file_type();

    if (file_type.is_file()) {return TypeFichier::FichierRegulier;}
    if (file_type.is_dir()) {return TypeFichier::Dossier;}
    if (file_type.is_symlink()) {return TypeFichier::LienSymbolique;}

    panic!("Type de fichier non reconnu: {}", fichier_chemin);
}