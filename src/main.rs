// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Les implémentations des algorithmes de tri sont dans des fichiers
// séparés algos_tri.rs et algos_tr_variant.rs
// On inclut ces 'module'
mod classiques;
mod conversions_hexa_bin_dec;
mod divers;
mod fichiers;
mod fractales;
mod graphes;
mod probabilites;
mod rationnels;
mod tri;
mod tri_variantes;

#[cfg(test)]
mod tests;

use std::time::{SystemTime, UNIX_EPOCH};

use classiques::resoud_tours_de_hanoi;

use crate::rationnels::Rationnels;

// Connaitre le temps en secondes depuis l'epoch
fn get_curr_time_epoch() -> f64 {
    return (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as f64)
        / 1000.;
}

fn main() {
    println!("Hello, world!");

    let b_test_classiques = false;
    let b_test_recherche_tableau_et_tris = false;
    let b_test_tris_variants = false;
    let b_test_probas = false;
    let b_test_algos_divers = false;
    let b_test_rationnels = true;
    let b_test_fichiers = false;
    let b_test_conversions_entiers = false;
    let b_test_graphes = false;
    let b_test_fractales = false;

    // Test des fonctions 'mathématiques': Factorielle, pgcd, fibonacci_interatif, fibonacci_recursif
    if (b_test_classiques) {
        println!("");
        let n: u64 = 5;
        println!("Factorielle({}) = {}", n, classiques::factorielle(n));

        //let a: u64 = 48;
        //let b: u64 = 42;
        let a: u64 = 90;
        let b: u64 = 28;
        println!("pgcd({}, {}) = {}\n", a, b, classiques::pgcd(a, b));
        println!("pgcd_asm({}, {}) = {}\n", a, b, divers::pgcd_asm(a, b));

        let n = 5;
        //let n = 100;   Pour comparer les temps de calcul des 2 implémentations de Fibonacci
        for i in 0..n {
            println!(
                "Fibonacci_iteratif({}) = {}",
                i,
                classiques::fibonacci_iteratif(i)
            );
            println!(
                "Fibonacci_recursif({}) = {}",
                i,
                classiques::fibonacci_recursif(i)
            );
        }

        for n in 1..6 {
            resoud_tours_de_hanoi(n);
            println!("\n");
        }
    }

    if (b_test_recherche_tableau_et_tris) {
        println!("");

        // Tester le générateur aléatoire (MINSTD)
        if (false) {
            let seed: u32 = 1234;
            let mut rng: probabilites::RngMinstd = probabilites::RngMinstd::new(seed);
            println!("PRNG: {}", rng.gen());
        }

        let seed: u32 = 1234;

        let n = 13;
        //let n = 40000;

        //let mut mon_tableau: Vec<i32> = vec![1, 2, 3, 4, 5];
        //let mut mon_tableau: Vec<i32> = vec![5, 4, 3, 2, 1];
        let mut mon_tableau: Vec<i32> = Vec::from_iter((0..n));
        //let mut mon_tableau: Vec<i32> = Vec::from_iter((0..n).rev());

        let mon_tableau2: &mut [i32] = mon_tableau.as_mut_slice();

        println!("\ntableau départ: \n {:?}", &mon_tableau2);
        probabilites::fisher_yates_shuffle(mon_tableau2, seed);
        println!("\ntableau mélangé: \n {:?}\n", &mon_tableau2);

        let p: i32 = 8;
        println!(
            "Recherche lineaire de la valeur {}: index {} \n",
            p,
            classiques::recherche_lineaire(mon_tableau2, p).unwrap()
        );
        let p: i32 = 12;
        println!(
            "Recherche lineaire générique de la valeur {}: index {} \n",
            p,
            classiques::recherche_lineaire_generique(mon_tableau2, p).unwrap()
        );
        if (false) {
            println!(
                "Recherche dichotomique de la valeur {}: index {}",
                p,
                classiques::recherche_dichotomique(mon_tableau2, p, None, None).unwrap()
            );
        }

        //algos_tri::tri_par_insertion(mon_tableau2);
        //algos_tri::tri_par_selection(mon_tableau2);

        //algos_tri::tri_rapide(mon_tableau2);
        //algos_tri::tri_fusion(mon_tableau2);
        tri::tri_par_tas_generique(mon_tableau2);

        println!("tableau trié: \n{:?}", &mon_tableau2);
        assert!(
            tri::verif_tableau_croissant(&mon_tableau2),
            "Erreur: le tableau n'est pas correctement trié."
        );
    }

    if (b_test_tris_variants) {
        println!("");
        // Tableau de données string
        let mut mon_tableau_gen: Vec<String> = vec![
            "rust".to_string(),
            "go".to_string(),
            "shell".to_string(),
            "ruby".to_string(),
            "python".to_string(),
        ];
        let mon_tableau_gen2: &mut [String] = mon_tableau_gen.as_mut_slice();

        /*
        // Tableau d'entiers
        let seed: u32 = 1234;
        let n = 17;
        let mut mon_tableau_gen: Vec<i32> = Vec::from_iter((0..n));
        let mon_tableau_gen2: &mut [i32] = mon_tableau_gen.as_mut_slice();
        probabilites::fisher_yates_shuffle(mon_tableau_gen2, seed);
        */

        // Tableau de flottants
        // Ordre pas total sur les flottants (car valeur NaN possible)
        // let mut mon_tableau_gen: Vec<f64> = vec![3.1415, 1.4142, 2.718, 1.732, 6.022, -273.15];
        // let mon_tableau_gen2: &mut [f64] = mon_tableau_gen.as_mut_slice();

        println!("\ntableau départ: \n {:?}", &mon_tableau_gen2);

        tri_variantes::tri_par_insertion_generique(mon_tableau_gen2);

        // Tri fusion: Implémenté uniquement sur les entiers 'i32' (pas générique)
        //tri_variantes::tri_fusion_ameliore(mon_tableau_gen2, None, None, None);

        // Pour tester le tri par selection, qui est implémenté en 'indirect',
        // et ne modifie pas directement le tableau.
        // => nécessite d'appliquer la permutation à postériori.
        if (false) {
            let permutation = tri_variantes::tri_par_selection_indirect_generique(mon_tableau_gen2);
            let mut mon_tableau_gen2_vec =
                tri_variantes::permute_copie_tableau(mon_tableau_gen2, &permutation);
            mon_tableau_gen2.clone_from_slice(mon_tableau_gen2_vec.as_mut_slice());
            println!("Permutation: {:?}", permutation);
        }

        println!("tableau trié: \n{:?}", &mon_tableau_gen2);
        assert!(
            tri::verif_tableau_croissant(&mon_tableau_gen2),
            "Erreur: le tableau n'est pas correctement trié."
        );
    }

    if (b_test_probas) {
        println!("");
        let seed: u32 = 1234;
        let n: usize = 10000;
        let normales: Vec<f64> = probabilites::box_muller(n, seed);
        let moyenne = probabilites::moyenne(&normales.as_slice()).unwrap();
        let variance = probabilites::variance(&normales.as_slice(), None).unwrap();
        //println!("normales: {:?}", &normales);
        println!("moyenne, variance, : {} {}", moyenne, variance);
    }

    if (b_test_algos_divers) {
        println!("");
        let solutions = divers::resoud_probleme_des_8_dames();
        println!("Nb solutions: {}", solutions.len());
        //println!("Solutions: {:?}", solutions);
        let solutions_uniques = divers::calcule_solutions_uniques(&solutions);
        println!("Nb solutions uniques: {}", solutions_uniques.len());
        println!("Solutions uniques: {:?}", solutions_uniques);
        println!(" ");
        divers::affiche_solutions_probleme_des_8_dames(&solutions_uniques);

        println!(" ");

        // Test des fonctions de recherche de nombres premiers
        let min_n: usize = 0;
        //let min_n:usize  = 1000000;
        //let max_n:usize  = 100;
        let max_n: usize = 2000000;
        //let max_n:usize  = 10000000;
        //let max_n:usize  = 1800000;

        let chrono_start = get_curr_time_epoch();
        println!("Test recherche_premiers() min_n:{}, max_n:{}", min_n, max_n);
        let premiers: Vec<usize> = divers::recherche_premiers(min_n, max_n);
        //println!("Nombres premiers : {:?}", premiers);
        println!(
            "Nombre total de nombres premiers trouvés: {}",
            premiers.len()
        );
        let chrono_end = get_curr_time_epoch();
        let duree_recherche_premiers = (chrono_end - chrono_start);
        println!("Durée des calculs: {}", duree_recherche_premiers);

        let chrono_start = get_curr_time_epoch();
        println!("\n");
        let batch_size: usize = (max_n - min_n) / 40;
        println!(
            "Test recherche_premiers_multithreading() min_n:{}, max_n:{}, batch_size:{}",
            min_n, max_n, batch_size
        );
        let premiers: Vec<usize> =
            divers::recherche_premiers_multithreading(min_n, max_n, batch_size);
        println!(
            "Nombre total de nombres premiers trouvés: {}",
            premiers.len()
        );
        let chrono_end = get_curr_time_epoch();
        let duree_recherche_premiers_mutithreading = (chrono_end - chrono_start);
        println!(
            "Durée des calculs: {}",
            duree_recherche_premiers_mutithreading
        );
        println!(
            "ratio de durée avec multithreading / sans multithreading : {}",
            duree_recherche_premiers / duree_recherche_premiers_mutithreading
        );

        // Test de la conjecture de Syracuse
        println!("\nTest de la conjecture de Syracuse");
        let n: u64 = 15;
        let (temps_de_vol, altitude_max) = divers::calcule_temps_de_vol_et_altitude_max(n.clone());
        println!(
            "n = {}; Temps de vol = {}; Altitude max = {}",
            n, temps_de_vol, altitude_max
        );
        let n: u64 = 27;
        let (temps_de_vol, altitude_max) = divers::calcule_temps_de_vol_et_altitude_max(n.clone());
        println!(
            "n = {}; Temps de vol = {}; Altitude max = {}",
            n, temps_de_vol, altitude_max
        );
        let n_max: u64 = 100;
        let (temps_de_vol_max, temps_de_vol_max_index) =
            divers::calcule_temps_de_vol_max(n_max.clone());
        println!(
            "n_max = {}; Temps de vol max = {}; index associé = {}",
            n_max, temps_de_vol_max, temps_de_vol_max_index
        );
        let n_max: u64 = 1000000;
        let (temps_de_vol_max, temps_de_vol_max_index) =
            divers::calcule_temps_de_vol_max(n_max.clone());
        println!(
            "n_max = {}; Temps de vol max = {}; index associé = {}",
            n_max, temps_de_vol_max, temps_de_vol_max_index
        );
        //let (temps_de_vol_max, temps_de_vol_max_index) = divers::calcule_temps_de_vol_max_asm(n_max.clone());
        //println!("n_max = {}; Temps de vol max (asm) = {}; index associé = {}", n_max, temps_de_vol_max, temps_de_vol_max_index);
    }

    if (b_test_rationnels) {
        // Test addition
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let r2 = rationnels::Rationnels::new(5i64, 6i64);
        let r3 = &r1 + &r2;
        println!("{} + {} = {} (somme avec références)", &r1, &r2, &r3);

        let r3 = r1 + r2;
        println!("meme somme = {} (somme sans référence)", &r3);

        // Test AddAssign
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let mut r2 = rationnels::Rationnels::new(5i64, 6i64);
        r2 += r1;
        println!("meme somme = {} (somme avec l'opérateur '+=' )\n", &r3);

        // Test soustraction
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let r2 = rationnels::Rationnels::new(5i64, 6i64);
        let r3 = &r1 - &r2;
        println!("{} - {} = {} (soustraction avec références)", &r1, &r2, &r3);

        let r3 = r1 - r2;
        println!("meme soustraction = {} (sans référence)", &r3);

        // Test SubAssign
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let mut r2 = rationnels::Rationnels::new(5i64, 6i64);
        r2 -= r1;
        println!(
            "meme soustraction = {} (soustraction avec l'opérateur '-=' )\n",
            &r3
        );

        // Test multiplication
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let r2 = rationnels::Rationnels::new(5i64, 6i64);
        let r3 = &r1 * &r2;
        println!(
            "{} * {} = {} (multiplication avec références)",
            &r1, &r2, &r3
        );

        let r3 = r1 * r2;
        println!("meme multiplication = {} (sans référence)\n", &r3);

        // Test division
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let r2 = rationnels::Rationnels::new(5i64, 6i64);
        let r3 = &r1 / &r2;
        println!(
            "{} / {} = {} (multiplication avec références)",
            &r1, &r2, &r3
        );

        let r3 = r1 / r2;
        println!("meme division = {} (sans référence)\n", &r3);

        // Test négation
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        //let r2 = rationnels::Rationnels::new(5i64, 6i64);
        let r1neg = -&r1;
        println!("-({}) = {} (négation avec référence)", &r1, &r1neg);
        //let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let r1neg = -r1;
        println!("    = {} (sans référence)\n", &r1neg);

        // Test comparaison
        let r1 = rationnels::Rationnels::new(2i64, 3i64);
        let r2 = rationnels::Rationnels::new(5i64, 6i64);
        println!("{} >= {} ? Réponse: {}", &r1, &r2, r1 >= r2);
        println!("{} > {} ? Réponse: {}", &r1, &r2, r1 > r2);
        println!("{} <= {} ? Réponse: {}", &r1, &r2, r1 <= r2);
        println!("{} < {} ? Réponse: {}", &r1, &r2, r1 < r2);
        println!("{} >= {} ? Réponse: {}", &r1, &r1, r1 >= r1);
        println!("{} <= {} ? Réponse: {}", &r1, &r1, r1 <= r1);

        // Test comparison, via un des algos de tri générique
        let mut mon_tableau_de_rationnels: Vec<Rationnels<i64>> = Vec::new();
        mon_tableau_de_rationnels.push(rationnels::Rationnels::new(2, 3));
        mon_tableau_de_rationnels.push(rationnels::Rationnels::new(-2, 3));
        mon_tableau_de_rationnels.push(rationnels::Rationnels::new(-1, 2));
        mon_tableau_de_rationnels.push(rationnels::Rationnels::new(2, 5));
        mon_tableau_de_rationnels.push(rationnels::Rationnels::new(-7, 5));
        let mon_tableau_de_rationnels2: &mut [Rationnels<i64>] =
            &mut mon_tableau_de_rationnels.as_mut_slice();
        println!("\ntableau départ: \n {:?}", &mon_tableau_de_rationnels2);
        tri_variantes::tri_par_insertion_generique(mon_tableau_de_rationnels2);
        println!("\ntableau trié: \n {:?}\n", &mon_tableau_de_rationnels2);

        // Test de conversion rationnel vers flottant
        let r1 = rationnels::Rationnels::new(2i32, 3i32);
        // On clone r1, car il est consommé par la conversion qui suit, mais on veut quand meme l'afficher..
        let r1_clone = r1.clone();
        let r1_flottant: f64 = f64::from(r1);
        println!("{} = {}", &r1_clone, r1_flottant);

        // Test de conversion entier vers rationnel
        let entier_i64: i64 = 5;
        let r1 = Rationnels::<i64>::from(entier_i64);
        println!("{} = {}", &entier_i64, r1);
    }

    if (b_test_fichiers) {
        // Test des accés aux fichiers
        let fichier_chemin = "./divers/mon_fichier.txt".to_string();

        let contenu_a_ecrire: String = "Première ligne\nSeconde ligne".to_string();
        println!("Ecriture dans le fichier {}", fichier_chemin);
        fichiers::ecrire_fichier_texte(&fichier_chemin, &contenu_a_ecrire);
        println!("Contenu écrit: {}", contenu_a_ecrire);
        println!(" ");

        let existe: bool = fichiers::test_existence_fichier(&fichier_chemin);
        println!("Le fichier {} existe: {}", fichier_chemin, existe);
        println!(" ");

        let contenu_fichier: String = fichiers::lire_fichier_texte(&fichier_chemin);
        println!("Contenu du fichier:");
        println!("{:?}", &contenu_fichier);
        println!(" ");

        let lignes: Vec<String> = fichiers::lire_fichier_texte_lignes(&fichier_chemin, None);
        println!("Liste de lignes lues avec le sépateur par défaut:");
        println!("{:?}", &lignes);
        println!(" ");

        let contenu_binaire: Vec<u8> = vec![0x42, 0x6f, 0x6e, 0x6a, 0x6f, 0x75, 0x72];
        //                            idem [66, 111, 110, 106, 111, 116, 114];
        let fichier_binaire_chemin = "./divers/mon_fichier.dat".to_string();
        println!("Ecriture dans le fichier {}", fichier_binaire_chemin);
        fichiers::ecrire_fichier_binaire(&fichier_binaire_chemin, &contenu_binaire);
        println!("Contenu écrit: {:?}", contenu_binaire);
        println!(" ");

        let contenu_binaire_lu: Vec<u8> = fichiers::lire_fichier_binaire(&fichier_binaire_chemin);
        assert_eq!(
            contenu_binaire, contenu_binaire_lu,
            "Erreur: Le contenu diffère de celui attendu"
        );
        println!("Contenu du fichier:");
        println!("{:?}", &contenu_binaire);
        let contenu: String = String::from_utf8(contenu_binaire.clone()).unwrap();
        println!("{}", &contenu);
        println!(" ");

        let dossier_chemin = "./".to_string();
        let contenu_dossier: Vec<String> = fichiers::liste_dossier(&dossier_chemin);
        println!("Contenu du dossier '{}' :", dossier_chemin);
        println!("{:?}", &contenu_dossier);
        println!(" ");

        let fichier_chemin = "./divers/mon_fichier.txt".to_string();
        let taille = fichiers::donne_taille_fichier(&fichier_chemin);
        println!("Taille du fichier '{}' : {:?}", fichier_chemin, taille);
        println!(" ");

        let fichier_chemin = "./divers/mon_fichier.txt".to_string();
        let type_fichier = fichiers::donne_infos_fichier(&fichier_chemin);
        println!("Type du fichier '{}' : {:?}", fichier_chemin, type_fichier);
        println!(" ");

        let fichier_chemin = "./.git".to_string();
        let type_fichier = fichiers::donne_infos_fichier(&fichier_chemin);
        println!("Type du fichier '{}' : {:?}", fichier_chemin, type_fichier);
        println!(" ");
    }

    if (b_test_conversions_entiers) {
        conversions_hexa_bin_dec::conversions_entier();
    }

    if (b_test_graphes) {
        let d_labyrinthes: String = "divers/labyrinthes".to_string();
        let f_liste_plan_labyrinthes: Vec<String> = fichiers::liste_dossier(&d_labyrinthes);

        for f_plan_labyrinthe in f_liste_plan_labyrinthes {
            if (f_plan_labyrinthe.contains("solution")) {
                continue;
            }

            let f_plan_solution =
                f_plan_labyrinthe.split(".").collect::<Vec<_>>()[0].to_string() + "_solution.txt";
            //println!("{},{}", f_plan_labyrinthe, f_plan_solution);
            graphes::resoud_labyrinthe(f_plan_labyrinthe, f_plan_solution);
        }
    }

    if (b_test_fractales) {
        let hauteur: u32 = 1024;
        let largeur: u32 = 1920;
        let f_flocon_svg: String = "images/flocon_Koch.svg".to_string();
        let n_iter: u32 = 4;
        let lignes: Vec<fractales::Ligne> = fractales::flocon_koch(hauteur, largeur, n_iter);
        fractales::cree_fichier_svg_depuis_lignes(&f_flocon_svg, hauteur, largeur, lignes);

        let x_fractale = 0.3;
        let y_fractale = 0.5;
        let f_fractale_bmp: String = "images/fractale.bmp".to_string();
        fractales::calcule_fractale_et_ecrit_bmp(x_fractale, y_fractale, &f_fractale_bmp);
    }
}
