
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

use crate::tri::verif_tableau_croissant;

// Implémentation récursive de la factorielle
// Entrée: n entier
// Sortie: n! = n * (n - 1) * (n - 2) * ... * 1
// Complexité linéaire
// Voir: https://fr.wikipedia.org/wiki/Factorielle
pub fn factorielle(n : u64) -> u64
{
    // Gestion du cas particulier (fin des appels récursifs)
    if n <= 1 {return 1;}

    // Appel récursif
    return n * factorielle(n - 1);
}


// Calcul récursif du pgcd des entiers a et b
// Entrée: 2 entiers: a et b
// Sortie: a ^ b = PGCD(a, b) - Plus Grand Commun Diviseur
// Voir: https://fr.wikipedia.org/wiki/Plus_grand_commun_diviseur
pub fn pgcd(a: u64, b: u64) -> u64
{
    // On permute a et b si a < b
    if a < b {return pgcd(b, a);}

    let m: u64 = a % b;
    // Gestion du cas particulier (fin des appels récursifs)
    if m == 0 {return b;}

    // Appel récursif
    return pgcd(b, m);

    // Le nombre de récursion est fini car a et b sont des entiers positifs 
    // et diminuent strictement à chaque appel
}


// Implémentation itérative calculant le n-ième élément de la suite de Fibonacci
// Entrée: n entier
// Sortie: Fibo(n), qui est le n-ième élément de la suite de Fibonacci
// Complexité linéaire
// Voir: https://fr.wikipedia.org/wiki/Suite_de_Fibonacci#Algorithme_polynomial
pub fn fibonacci_iteratif(n : u64) -> u64
{
    // Gestion des cas particuliers
    if n <= 0 {return 0;}
    if n == 1 {return 1;}

    // On déclare les variables u, v, w  'mutables' pour pouvoir les modifier plus tard
    let mut u: u64 = 0;      // Fibonacci(0) = 0
    let mut v: u64 = 1;      // Fibonacci(1) = 1
    let mut w: u64 = u + v;

    // for i in range(1, n):
    for _i in 1..n
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
pub fn fibonacci_recursif(n : u64) -> u64
{
    match (n) {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursif(n - 1) + fibonacci_recursif(n - 2),
    }
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
pub fn recherche_lineaire(mon_tableau: &[i32], val_recherche: i32) -> Option<usize>
{
    println!("Appel à la fonction recherche_lineaire.");
    let n = mon_tableau.len();

    for i in 0..n
    {
        if mon_tableau[i] == val_recherche {return Some(i);}
        // Invariant de boucle: A la fin de chaque itération, si les itérations se poursuivent,
        // val_recherche n'a pas été trouvé parmi les (i + 1) premiers éléments du tableau
    }
    return None
}

// recherche linéaire implémentée de façon 'générique'
// Fonctionne pour tous les types de données que l'on peut comparer
// au sens 'être égal ou pas' : C'est le trait Rust 'core::cmp::Eq'
pub fn recherche_lineaire_generique<T>(mon_tableau: &[T], val_recherche: T) -> Option<usize>
where T : core::cmp::Eq
{
    println!("Appel à la fonction recherche_lineaire_generique.");
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
pub fn recherche_dichotomique(mon_tableau: &[i32], val_recherche: i32, index_min_opt: Option<usize>, index_max_inclus_opt: Option<usize>) -> Option<usize>
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

