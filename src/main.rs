
// Les implémentations des algorithmes de tri sont dans des fichiers
// séparés algos_tri.rs et algos_tr_variant.rs
// On inclut ces 'module'
mod probabilites;
mod algos_tri;
mod algos_tri_variantes;
mod algos_divers;


// Implémentation récursive de la factorielle
// Entrée: n entier
// Sortie: n! = n * (n - 1) * (n - 2) * ... * 1
// Complexité linéaire
// Voir: https://fr.wikipedia.org/wiki/Factorielle
fn factorielle(n : u64) -> u64
{
    // Gestion du cas particulier (fin des appels récursifs)
    if n <= 1 {return 1;}
    return n * factorielle(n - 1);
}


// Calcul récursif du pgcd des entiers a et b
// Entrée: 2 entiers: a et b
// Sortie: a ^ b = PGCD(a, b) - Plus Grand Commun Diviseur
// Voir: https://fr.wikipedia.org/wiki/Plus_grand_commun_diviseur
fn pgcd(a: u64, b: u64) -> u64
{
    // On permute a et b si a < b
    if a < b {return pgcd(b, a);}

    let m: u64 = a % b;
    // Gestion du cas particulier (fin des appels récursifs)
    if m == 0 {return b;}

    // Appel récursif
    return pgcd(b, m);
}


// Implémentation itérative calculant le n-ième élément de la suite de Fibonacci
// Entrée: n entier
// Sortie: Fibo(n), qui est le n-ième élément de la suite de Fibonacci
// Complexité linéaire
// Voir: https://fr.wikipedia.org/wiki/Suite_de_Fibonacci#Algorithme_polynomial
fn fibonacci_iteratif(n : u64) -> u64
{
    // Gestion des cas particuliers
    if n <= 0 {return 0;}
    if n == 1 {return 1;}

    // On déclare les variables u, v, w  'mutables' pour pouvoir les modifier plus tard
    let mut u: u64 = 0;      // Fibonacci(0) = 0
    let mut v: u64 = 1;      // Fibonacci(1) = 1
    let mut w: u64 = u + v;

    // for i in range(1, n):
    for i in 1..n
    {
        w = u + v;
        u = v;
        v = w;
        // Invariant de boucle: A la fin de chaque itération, w = Fibo(i + 1)
    }

    return w;
}


// Implémentation récursive calculant le n-ième élément de la suite de Fibonacci
// Entrée: n entier
// Sortie: Fibo(n), qui est le n-ième élément de la suite de Fibonacci
// Complexité exponentielle (implémentation naïve - nombreux calculs répétitifs)
// Voir: https://fr.wikipedia.org/wiki/Suite_de_Fibonacci#Algorithme_r%C3%A9cursif_na%C3%AFf
fn fibonacci_recursif(n : u64) -> u64
{
    // Gestion des cas particuliers (fin des appels récursifs)
    if n <= 0 {return 0;}
    if n == 1 {return 1;}

    // Appels récursifs
    return fibonacci_recursif(n - 1) + fibonacci_recursif(n - 2);
}


// Fonction implémentant la recherche linéaire
// Entrées:
// * mon_tableau: tableau d'entiers sous la forme d'une 'slice' Rust de i32
// * val_recherche: Valeur que l'on recherche au sein du tableau
// Sortie:
// * Option<usize>: Enum Rust qui vaut:
//   -> Soit Some(mon_index) : index du premier élément du tableau dont la valeur est 'val_recherche', s'il est trouvé
//   -> Soit None si l'élément n'a pas été trouvé
// La recherche s'effectue en itérant sur les éléments de 'mon_tableau'
// L'index retourné commence à zéro (convention Python & Rust)
// Complexité: linéaire..
// Voir: https://fr.wikipedia.org/wiki/Recherche_s%C3%A9quentielle
fn recherche_lineaire(mon_tableau: &mut [i32], val_recherche: i32) -> Option<usize>
{
    let n = mon_tableau.len();

    for i in 0..n
    {
        if mon_tableau[i] == val_recherche {return Some(i);}
        // Invariant de boucle: A la fin de chaque itération, si les itérations se poursuivent,
        // val_recherche n'a pas été trouvé parmi les (i + 1) premiers éléments du tableau
    }
    return None
}


// Fonction implémentant la recherche dichotomique
// Entrées:
// * mon_tableau: tableau d'entiers
// * val_recherche: Valeur que l'on recherche au sein du tableau
// Sortie:
// * L'index du premier élément du tableau dont la valeur est 'val_recherche', s'il est trouvé
// * renvoie 'None', si pas trouvé
// On élimine la moitié des éléments à chaque fois, en comparant la valeur du milieu du tableau à la valeur cherchée.
// L'index commence à zéro (convention Python & Rust)
// << La fonction nécessite que le tableau d'entiers en entrée soit croissante >>
// complexité: log_2(n),
// Voir: https://fr.wikipedia.org/wiki/Recherche_dichotomique
fn recherche_dichotomique(mon_tableau: &mut [i32], val_recherche: i32, index_min_opt: Option<usize>, index_max_inclus_opt: Option<usize>) -> Option<usize>
{
    assert!(verif_tableau_croissant(&mon_tableau), "(recherche_dichotomique) Erreur: le tableau n'est pas croissant (Nécessite de le trier d'abord).");

    if (false)
    {
        if ((index_min_opt != None) && (index_max_inclus_opt != None))
        {
            println!("Appel à recherche_dichotomique: {} {}", index_min_opt.unwrap(), index_max_inclus_opt.unwrap());
        }
        else {
            println!("Appel à recherche_dichotomique");
        }
    }

    let n = mon_tableau.len();

    // Cas particulier du premier appel à la fonction, l'appel général (sans indiquer les bornes de recherche)
    if ((index_min_opt == None) || (index_max_inclus_opt == None))
    {
        return recherche_dichotomique(mon_tableau, val_recherche, Some(0), Some(n - 1));
    }

    // A partir d'ici, on sait que index_min_opt et index_max_opt sont des 'vraies' valeurs (pas None)
    // donc on les unwrap() sans risque
    let index_min:usize        = index_min_opt.unwrap();
    let index_max_inclus:usize = index_max_inclus_opt.unwrap();

    if index_min > index_max_inclus
    {
        println!("Erreur: Recherche_dichotomique: Erreur interne, les bornes sont inversées");
        // On implémente pas l'inversion des bornes ici, car cela ne derait jamais arriver.
    }

    // Gestion des cas particuliers (fin des appels récursifs)
    // tableau avec 1 seul élement
    if index_max_inclus == index_min
    {
        if mon_tableau[index_min] == val_recherche
            {return Some(index_min);}
        return None;
    }

    // Gestion des cas particuliers (fin des appels récursifs)
    // tableau avec 2 élements
    if index_max_inclus == (index_min + 1)
    {
        if mon_tableau[index_min] == val_recherche
            {return Some(index_min);}
        if mon_tableau[index_max_inclus] == val_recherche
            {return Some(index_max_inclus);}
        return None
    }

    // Cas général, qui aboutit à un appel récursif
    let index_mid = ((index_min + index_max_inclus) / 2) as usize;
    let index_mid_opt = Some(index_mid);
    let val_mid = mon_tableau[index_mid];

    // Appel récursif
    if (val_recherche > val_mid)
    {
        return recherche_dichotomique(mon_tableau, val_recherche, index_mid_opt, index_max_inclus_opt);
    }
    else
    {
        return recherche_dichotomique(mon_tableau, val_recherche, index_min_opt, index_mid_opt);
    }

} // fn recherche_dichotomique()



// Fonction vérifiant qu'un tableau est croissant
// Entrée = mon_tableau: tableau d'entiers
// Sortie = un booleen. true -> tableau croissant..
// Implémenté de façon générique
// pour tous les types de données triables
fn verif_tableau_croissant<T>(mon_tableau: &[T]) -> bool
where T : Ord   // Le type T doit avoir le 'trait' Rust 'Ord': Les éléments doivent être ordonnés donc comparables
{
    let n = mon_tableau.len();
    for i in 0..(n - 1)
    {
        if mon_tableau[i + 1] < mon_tableau[i] {return false;}
    }
    return true;
}



fn main() {
    println!("Hello, world!");

    let b_test_fonctions_math = false;
    let b_test_recherche_tableau_et_tris = true;
    let b_test_tris_variants = true;
    let b_test_probas = false;
    let b_test_algos_divers = true;

    // Test des fonctions 'mathématiques': Factorielle, pgcd, fibonacci_interatif, fibonacci_recursif
    if (b_test_fonctions_math)
    {
        println!("");
        let n: u64 = 5;
        println!("Factorielle({}) = {}", n, factorielle(n));

        let a: u64 = 42;
        let b: u64 = 48;
        println!("pgcd({}, {}) = {}", a, b, pgcd(a, b));

        for i in 0..100
        {
            println!("Fibonacci_iteratif({}) = {}", i, fibonacci_iteratif(i));
            println!("Fibonacci_recursif({}) = {}", i, fibonacci_recursif(i));
        }

    }

    if (b_test_recherche_tableau_et_tris)
    {
        println!("");

        // Tester le générateur aléatoire (MINSTD)
        if (false)
        {
            let seed: u32 = 1234;
            let mut rng: probabilites::rng_minstd = probabilites::rng_minstd::new(seed);
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
        println!("\ntableau mélangé: \n {:?}", &mon_tableau2);

        let p: i32 = 3;
        println!("\nRecherche lineaire de la valeur {}: index {} \n", p, recherche_lineaire(mon_tableau2, p).unwrap());
        //println!("Recherche dichotomique de la valeur {}: index {}", p, recherche_dichotomique(mon_tableau2, p, None, None).unwrap());

        //algos_tri::tri_par_insertion(mon_tableau2);
        //algos_tri::tri_par_selection(mon_tableau2);
        
        //algos_tri::tri_rapide(mon_tableau2);
        //algos_tri::tri_fusion(mon_tableau2);
        algos_tri::tri_par_tas_generique(mon_tableau2);

        println!("tableau trié: \n{:?}", &mon_tableau2);
        assert!(verif_tableau_croissant(&mon_tableau2), "Erreur: le tableau n'est pas correctement trié.");
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
            let permutation = algos_tri_variantes::tri_par_selection_indirect_generique(mon_tableau_gen2);
            let mut mon_tableau_gen2_vec = algos_tri_variantes::permute_copie_tableau(mon_tableau_gen2 , &permutation);
            mon_tableau_gen2.clone_from_slice(mon_tableau_gen2_vec.as_mut_slice()); 
            println!("Permutation: {:?}", permutation);
        }

        println!("tableau trié: \n{:?}", &mon_tableau_gen2);
        assert!(verif_tableau_croissant(&mon_tableau_gen2), "Erreur: le tableau n'est pas correctement trié.");
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
        let solutions = algos_divers::resoud_probleme_des_8_dames();
        println!("Nb solutions: {}", solutions.len());
        //println!("Solutions: {:?}", solutions);
        let solutions_uniques = algos_divers::calcule_solutions_uniques(&solutions);
        println!("Nb solutions uniques: {}", solutions_uniques.len());
        println!("Solutions uniques: {:?}", solutions_uniques);
        println!(" ");
        algos_divers::affiche_solutions_probleme_des_8_dames(&solutions_uniques);
    }

}
