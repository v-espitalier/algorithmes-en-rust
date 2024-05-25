
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Ne pas faire de warning si des fonctions ne sont pas appelées
#![warn(dead_code)]

use std::fs::{self, read_dir, FileType, Permissions};
use std::path::Path;
use std::io::ErrorKind;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use std::io::Read;

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

pub fn ecrire_fichier_texte(fichier_chemin: &String, contenu: &String)
{
    // Voir: https://doc.rust-lang.org/std/fs/struct.File.html
    let mut fichier = File::create(fichier_chemin).expect("Erreur: N'a pas pu créer le fichier.");
    fichier.write_all(contenu.as_bytes()).expect("Erreur: N'a pas pu écrire dans le fichier.");
}

pub fn ecrire_fichier_texte_lignes(fichier_chemin: &String, contenu_vec: &Vec<String>)
{
    return ecrire_fichier_texte(fichier_chemin, &contenu_vec.join("\n"));
}



// Inspiré de: https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/?rdt=46881
pub fn lire_fichier_binaire(fichier_chemin: &String) -> Vec<u8>
{
    let mut fichier = File::open(&fichier_chemin)
        .expect("Fichier introuvable");
    let taille: usize = donne_taille_fichier(fichier_chemin) as usize;
    let mut buffer: Vec<u8> = vec![0; taille];
    fichier.read(&mut buffer).expect("Dépassement de capacité.");
    return buffer;
}

pub fn ecrire_fichier_binaire(fichier_chemin: &String, contenu: &Vec<u8>)
{
    std::fs::write(fichier_chemin, contenu).expect("Erreur: N'a pas pu écrire le fichier binaire.");
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

#[derive(Debug)]
pub struct InfosFichier {
    pub type_fichier: TypeFichier,
    pub permissions: Permissions,
    pub date_modif: SystemTime,
    pub taille: u64
}


pub fn donne_infos_fichier(fichier_chemin: &String) -> InfosFichier
{
    let metadata = fs::metadata(fichier_chemin).expect("Fichier non trouvé.");

    let file_type = metadata.file_type();
    let mut type_fichier_opt: Option<TypeFichier> = None;
    if (file_type.is_file()) {type_fichier_opt = Some(TypeFichier::FichierRegulier);}
    if (file_type.is_dir()) {type_fichier_opt = Some(TypeFichier::Dossier);}
    if (file_type.is_symlink()) {type_fichier_opt = Some(TypeFichier::LienSymbolique);}
    if (type_fichier_opt.is_none()) {
        panic!("Type de fichier non reconnu: {}", fichier_chemin);
    }

    let permissions: Permissions = metadata.permissions();
    let date_modif: SystemTime = metadata.modified().expect("Erreur avec metadata.modified()");
    let taille: u64 = metadata.len();

    return InfosFichier{ type_fichier : type_fichier_opt.unwrap(), permissions: permissions, date_modif: date_modif, taille: taille };
}

pub fn donne_taille_fichier(fichier_chemin: &String) -> u64
{
    let metadata = fs::metadata(fichier_chemin).expect("Fichier non trouvé.");
    let taille = metadata.len();
    return taille;
}