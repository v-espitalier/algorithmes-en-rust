
// Les implémentations des algorithmes de tri sont dans des fichiers
// séparés algos_tri.rs et algos_tr_variant.rs
// On inclut ces 'module'
mod probabilites;
mod algos_tri;
mod algos_tri_variantes;


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
// * ma_liste: liste d'entiers 'ma_liste'
// * val_recherche: Valeur que l'on recherche au sein de la liste
// Sortie:
// * Option<usize>: Enum Rust qui vaut:
//   -> Soit Some(mon_index) : index du premier élément de la liste dont la valeur est 'val_recherche', s'il est trouvé
//   -> Soit None si l'élément n'a pas été trouvé
// La recherche s'effectue en itérant sur les éléments de 'ma_liste'
// L'index retourné commence à zéro (convention Python & Rust)
// Complexité: linéaire..
// Voir: https://fr.wikipedia.org/wiki/Recherche_s%C3%A9quentielle
fn recherche_lineaire(ma_liste: &mut [i32], val_recherche: i32) -> Option<usize>
{
    let n = ma_liste.len();

    for i in 0..n
    {
        if ma_liste[i] == val_recherche {return Some(i);}
        // Invariant de boucle: A la fin de chaque itération, si les itérations se poursuivent,
        // val_recherche n'a pas été trouvé parmi les (i + 1) premiers éléments de la liste
    }
    return None
}


// Fonction implémentant la recherche dichotomique
// Entrées:
// * ma_liste: liste d'entiers 'ma_liste'
// * val_recherche: Valeur que l'on recherche au sein de la liste
// Sortie:
// * L'index du premier élément de la liste dont la valeur est 'val_recherche', s'il est trouvé
// * renvoie 'None', si pas trouvé
// On élimine la moitié des éléments à chaque fois, en comparant la valeur du milieu de la liste à la valeur cherchée.
// L'index commence à zéro (convention Python & Rust)
// << La fonction nécessite que la liste d'entiers en entrée soit croissante >>
// complexité: log_2(n),
// Voir: https://fr.wikipedia.org/wiki/Recherche_dichotomique
fn recherche_dichotomique(ma_liste: &mut [i32], val_recherche: i32, index_min_opt: Option<usize>, index_max_inclus_opt: Option<usize>) -> Option<usize>
{
    assert!(verif_liste_croissante(&ma_liste), "(recherche_dichotomique) Erreur: la liste n'est pas croissante (Nécessite de la trier d'abord).");

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

    let n = ma_liste.len();

    // Cas particulier du premier appel à la fonction, l'appel général (sans indiquer les bornes de recherche)
    if ((index_min_opt == None) || (index_max_inclus_opt == None))
    {
        return recherche_dichotomique(ma_liste, val_recherche, Some(0), Some(n - 1));
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
    // Liste avec 1 seul élement
    if index_max_inclus == index_min
    {
        if ma_liste[index_min] == val_recherche
            {return Some(index_min);}
        return None;
    }

    // Gestion des cas particuliers (fin des appels récursifs)
    // Liste avec 2 élements
    if index_max_inclus == (index_min + 1)
    {
        if ma_liste[index_min] == val_recherche
            {return Some(index_min);}
        if ma_liste[index_max_inclus] == val_recherche
            {return Some(index_max_inclus);}
        return None
    }

    // Cas général, qui aboutit à un appel récursif
    let index_mid = ((index_min + index_max_inclus) / 2) as usize;
    let index_mid_opt = Some(index_mid);
    let val_mid = ma_liste[index_mid];

    // Appel récursif
    if (val_recherche > val_mid)
    {
        return recherche_dichotomique(ma_liste, val_recherche, index_mid_opt, index_max_inclus_opt);
    }
    else
    {
        return recherche_dichotomique(ma_liste, val_recherche, index_min_opt, index_mid_opt);
    }

} // fn recherche_dichotomique()



// Fonction vérifiant qu'une liste est croissante
// Entrée = ma_liste: liste d'entiers
// Sortie = un booleen. true -> liste croissante..
fn verif_liste_croissante(ma_liste: &[i32]) -> bool
{
    let n = ma_liste.len();
    for i in 0..(n - 1)
    {
        if ma_liste[i + 1] - ma_liste[i] < 0 {return false;}
    }
    return true;
}



fn main() {
    println!("Hello, world!");

    let b_test_fonctions_math = false;
    let b_test_recherche_liste_et_tris = false;
    let b_test_tris_variants = false;
    let b_test_probas = true;

    // Test des fonctions 'mathématiques': Factorielle, pgcd, fibonacci_interatif, fibonacci_recursif
    if (b_test_fonctions_math)
    {
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

    if (b_test_recherche_liste_et_tris)
    {

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

        //let mut ma_liste: Vec<i32> = vec![1, 2, 3, 4, 5];
        //let mut ma_liste: Vec<i32> = vec![5, 4, 3, 2, 1];
        let mut ma_liste: Vec<i32> = Vec::from_iter((0..n));
        //let mut ma_liste: Vec<i32> = Vec::from_iter((0..n).rev());

        let ma_liste2: &mut [i32] = ma_liste.as_mut_slice();


        println!("\nListe départ: \n {:?}", &ma_liste2);
        probabilites::fisher_yates_shuffle(ma_liste2, seed);
        println!("\nListe mélangée: \n {:?}", &ma_liste2);

        let p: i32 = 3;
        println!("\nRecherche lineaire de la valeur {}: index {} \n", p, recherche_lineaire(ma_liste2, p).unwrap());
        //println!("Recherche dichotomique de la valeur {}: index {}", p, recherche_dichotomique(ma_liste2, p, None, None).unwrap());

        //algos_tri::tri_par_insertion(ma_liste2);
        //algos_tri_variantes::tri_par_insertion_generique(ma_liste2);
        //algos_tri::tri_par_selection(ma_liste2);
        
        //algos_tri::tri_rapide(ma_liste2);
        //algos_tri::tri_fusion(ma_liste2);
        //algos_tri::tri_fusion_ameliore(ma_liste2, None, None, None);
        algos_tri::tri_par_tas(ma_liste2);

        println!("Liste triée: \n{:?}", &ma_liste2);
        assert!(verif_liste_croissante(&ma_liste2), "Erreur: la liste n'est pas correctement triée.");
    }


    if (b_test_tris_variants)
    {
        // Tableau de données string
        let mut ma_liste_gen: Vec<String> = vec!["rust".to_string(), "go".to_string(), "shell".to_string(), "ruby".to_string(), "python".to_string()];
        let ma_liste_gen2: &mut [String] = ma_liste_gen.as_mut_slice();

        /*
        // Tableau d'entiers
        let seed: u32 = 1234;
        let n = 29;
        let mut ma_liste_gen: Vec<i32> = Vec::from_iter((0..n));
        let ma_liste_gen2: &mut [i32] = ma_liste_gen.as_mut_slice();
        fisher_yates_shuffle(ma_liste_gen2, seed);
        */

        // Tableau de flottants
        // Ordre pas total sur les flottants (NaN)
        // let mut ma_liste_gen: Vec<f64> = vec![3.1415, 1.4142, 2.718, 1.732, 6.022, -273.15];
        // let ma_liste_gen2: &mut [f64] = ma_liste_gen.as_mut_slice();
 

        println!("\nListe départ: \n {:?}", &ma_liste_gen2);

        //algos_tri_variantes::tri_par_insertion_generique(ma_liste_gen2);

        let permutation = algos_tri_variantes::tri_par_selection_indirect_generique(ma_liste_gen2);
        let ma_liste_gen2 = algos_tri_variantes::permute_copie_liste(ma_liste_gen2 , &permutation);
        println!("Permutation: {:?}", permutation);

        println!("Liste triée: \n{:?}", &ma_liste_gen2);
    }


    if (b_test_probas)
    {
        let seed: u32 = 1234;
        let n: usize = 10000;
        let normales: Vec<f64> = probabilites::box_muller(n, seed);
        let moyenne = probabilites::moyenne(&normales.as_slice()).unwrap();
        let variance = probabilites::variance_non_biaisee(&normales.as_slice()).unwrap();
        //println!("normales: {:?}", &normales);
        println!("moyenne, variance, : {} {}", moyenne, variance);
    }

}
