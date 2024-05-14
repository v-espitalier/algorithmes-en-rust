
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Les implémentations des algorithmes de tri sont dans des fichiers
// séparés algos_tri.rs et algos_tr_variant.rs
// On inclut ces 'module'
mod classiques;
mod probabilites;
mod tri;
mod tri_variantes;
mod divers;
mod rationnels;

#[cfg(test)]
mod tests;

use std::time::{SystemTime, UNIX_EPOCH};

// Connaitre le temps en secondes depuis l'epoch
fn get_curr_time_epoch() -> f64 {
    return (SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as f64) / 1000.;
}


fn main() {
    println!("Hello, world!");

    let b_test_fonctions_math = false;
    let b_test_recherche_tableau_et_tris = false;
    let b_test_tris_variants = false;
    let b_test_probas = false;
    let b_test_algos_divers = false;
    let b_test_rationnels = true;

    // Test des fonctions 'mathématiques': Factorielle, pgcd, fibonacci_interatif, fibonacci_recursif
    if (b_test_fonctions_math)
    {
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
        for i in 0..n
        {
            println!("Fibonacci_iteratif({}) = {}", i, classiques::fibonacci_iteratif(i));
            println!("Fibonacci_recursif({}) = {}", i, classiques::fibonacci_recursif(i));
        }

    }

    if (b_test_recherche_tableau_et_tris)
    {
        println!("");

        // Tester le générateur aléatoire (MINSTD)
        if (false)
        {
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
        println!("Recherche lineaire de la valeur {}: index {} \n", p, classiques::recherche_lineaire(mon_tableau2, p).unwrap());
        let p: i32 = 12;
        println!("Recherche lineaire générique de la valeur {}: index {} \n", p, classiques::recherche_lineaire_generique(mon_tableau2, p).unwrap());
        //println!("Recherche dichotomique de la valeur {}: index {}", p, recherche_dichotomique(mon_tableau2, p, None, None).unwrap());

        //algos_tri::tri_par_insertion(mon_tableau2);
        //algos_tri::tri_par_selection(mon_tableau2);
        
        //algos_tri::tri_rapide(mon_tableau2);
        //algos_tri::tri_fusion(mon_tableau2);
        tri::tri_par_tas_generique(mon_tableau2);

        println!("tableau trié: \n{:?}", &mon_tableau2);
        assert!(tri::verif_tableau_croissant(&mon_tableau2), "Erreur: le tableau n'est pas correctement trié.");
    }


    if (b_test_tris_variants)
    {
        println!("");
        // Tableau de données string
        let mut mon_tableau_gen: Vec<String> = vec!["rust".to_string(), "go".to_string(), "shell".to_string(), "ruby".to_string(), "python".to_string()];
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

        //algos_tri_variantes::tri_par_insertion_generique(mon_tableau_gen2);
        // Tri fusion: Implémenté uniquement sur les entiers 'i32' (pas générique)
        //algos_tri_variantes::tri_fusion_ameliore(mon_tableau_gen2, None, None, None);

        // Pour tester le tri par selection, qui est implémenté en 'indirect',
        // et ne modifie pas directement le tableau.
        // => nécessite d'appliquer la permutation à postériori.
        if (true)
        {
            let permutation = tri_variantes::tri_par_selection_indirect_generique(mon_tableau_gen2);
            let mut mon_tableau_gen2_vec = tri_variantes::permute_copie_tableau(mon_tableau_gen2 , &permutation);
            mon_tableau_gen2.clone_from_slice(mon_tableau_gen2_vec.as_mut_slice()); 
            println!("Permutation: {:?}", permutation);
        }

        println!("tableau trié: \n{:?}", &mon_tableau_gen2);
        assert!(tri::verif_tableau_croissant(&mon_tableau_gen2), "Erreur: le tableau n'est pas correctement trié.");
    }


    if (b_test_probas)
    {
        println!("");
        let seed: u32 = 1234;
        let n: usize = 10000;
        let normales: Vec<f64> = probabilites::box_muller(n, seed);
        let moyenne = probabilites::moyenne(&normales.as_slice()).unwrap();
        let variance = probabilites::variance_non_biaisee(&normales.as_slice()).unwrap();
        //println!("normales: {:?}", &normales);
        println!("moyenne, variance, : {} {}", moyenne, variance);
    }

    if (b_test_algos_divers)
    {
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
        let min_n:usize  = 0;
        //let min_n:usize  = 1000000;
        //let max_n:usize  = 100;
        let max_n:usize  = 2000000;
        //let max_n:usize  = 10000000;
        //let max_n:usize  = 1800000;

        let chrono_start = get_curr_time_epoch();
        println!("Test recherche_premiers() min_n:{}, max_n:{}", min_n, max_n);
        let premiers: Vec<usize> = divers::recherche_premiers(min_n, max_n);
        //println!("Nombres premiers : {:?}", premiers);
        println!("Nombre total de nombres premiers trouvés: {}", premiers.len());
        let chrono_end = get_curr_time_epoch();
        let duree_recherche_premiers = (chrono_end - chrono_start);
        println!("Durée des calculs: {}", duree_recherche_premiers);

        let chrono_start = get_curr_time_epoch();
        println!("\n");
        let batch_size:usize = (max_n - min_n) / 40;
        println!("Test recherche_premiers_multithreading() min_n:{}, max_n:{}, batch_size:{}", min_n, max_n, batch_size);
        let premiers: Vec<usize> = divers::recherche_premiers_multithreading(min_n, max_n, batch_size);
        println!("Nombre total de nombres premiers trouvés: {}", premiers.len());
        let chrono_end = get_curr_time_epoch();
        let duree_recherche_premiers_mutithreading = (chrono_end - chrono_start);
        println!("Durée des calculs: {}", duree_recherche_premiers_mutithreading);
        println!("ratio de durée avec multithreading / sans multithreading : {}", duree_recherche_premiers / duree_recherche_premiers_mutithreading);

    }

    if (b_test_rationnels)
    {
        // Test addition
        let r1 = rationnels::Rationnels { numerateur : 2i64, denominateur : 3i64};
        let r2 = rationnels::Rationnels { numerateur : 5i64, denominateur : 6i64};
        let mut r3 = &r1 + &r2; 
        r3.rendre_irreductible();
        println!("{} + {} = {} (somme avec références)", &r1, &r2, &r3);

        let mut r3 = r1 + r2; 
        r3.rendre_irreductible();
        println!("meme somme = {} (somme sans référence)", &r3);

        // Test soustraction
        let r1 = rationnels::Rationnels { numerateur : 2i64, denominateur : 3i64};
        let r2 = rationnels::Rationnels { numerateur : 5i64, denominateur : 6i64};
        let mut r3 = &r1 - &r2; 
        r3.rendre_irreductible();
        println!("{} - {} = {} (soustraction avec références)", &r1, &r2, &r3);

        let mut r3 = r1 - r2; 
        r3.rendre_irreductible();
        println!("meme soustraction = {} (sans référence)", &r3);


        /*
        let mut r4 = &r1;
        r4 += &r2;
        println!("{}", &r4);
        */

    }

}
