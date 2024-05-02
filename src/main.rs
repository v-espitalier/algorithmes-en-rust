
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



// Algorithme du tri par sélection
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// La liste est triée en place
// Complexité: n^2
// Plus précisément: (n(n-1)/2 comparaisons exactement, (n-1) permutations maximum)
// Voir: https://fr.wikipedia.org/wiki/Tri_par_s%C3%A9lection
fn tri_par_selection(ma_liste: &mut [i32])
{
    println!("tri_par_selection > appel");

    let n = ma_liste.len();

    // Cas particulier d'une liste réduite à un seul élément.
    if n <= 1 {return;}

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        let mut m = ma_liste[i];
        let mut m_index = i;

        // Recherche du plus petit élément, parmi les éléments non triés
        for j in (i+1)..n
        {
            let v = ma_liste[j];
            if v < m {m = v; m_index = j;}

        }

        // Le i-eme élement le plus petit du tableau se trouve en position m_index, et vaut m
        // On permute les élement d'index i et m_index
        if (i != m_index)
        {
            //println!("Echange les index {} et {}", i, m_index);
            let v_swap = ma_liste[i];
            ma_liste[i] = m;
            ma_liste[m_index] = v_swap;
        }
    }

    // 
    // return ma_liste
} // fn tri_par_selection ()


// Algorithme du tri fusion
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// Implémentation tri stable et en place
// Complexité: n.log(n)
// Alloue un tableau de taille moitié (une seule fois) pour stocker les valeurs intermédiaires
// et permettre la stabilité du tri
// https://fr.wikipedia.org/wiki/Tri_fusion
fn tri_fusion(ma_liste: &mut [i32], index_min_opt : Option<usize>, index_max_opt : Option<usize>, ma_sous_liste_1_opt: Option<&mut [i32]>)
{
    if ((index_min_opt != None) && (index_max_opt != None))
    {
        //println!("Appel à tri_fusion: {} {}", index_min_opt.unwrap(), index_max_opt.unwrap());
    }
    else {
        println!("Appel à tri_fusion");
    }

    let arguments_manquants: bool = (index_min_opt == None) || (index_max_opt == None) || ((ma_sous_liste_1_opt.as_ref()).is_none());
    match (arguments_manquants)
    {
        true => {
            let len_liste = ma_liste.len();
            let len_sous_liste_1: usize  = len_liste / 2 + 1;
            // Allocation sous tableau
            let mut ma_sous_liste_1_vec: Vec<i32> = vec![0 as i32; len_sous_liste_1];
            let ma_sous_liste_1: &mut [i32] = &mut ma_sous_liste_1_vec.as_mut_slice();
            return tri_fusion(ma_liste, Some(0), Some(len_liste - 1), Some(ma_sous_liste_1));
        }
        false => {

            // A partir d'ici, on sait que les arguments optionnels sont effectivement fournis (pas None)
            // index_min_opt, index_max_opt et ma_sous_liste_1_opt
            // On peut unwrap() sans risque
            let index_min: usize = index_min_opt.unwrap();
            let index_max: usize = index_max_opt.unwrap();
            let ma_sous_liste_1: &mut [i32] = ma_sous_liste_1_opt.unwrap();

            let n = index_max - index_min + 1;

            // Gestion des cas particuliers (fin des appels récursifs)
            // Liste avec 1 seul élément (-> Plus rien à trier)
            if n <= 1 {return;}

            // Gestion des cas particuliers (fin des appels récursifs)
            // Liste avec 2 élements
            if n == 2
            {
                // Permuter les elements d'indice 0 et 1 si nécessaire
                if ma_liste[index_min] > ma_liste[index_max]
                {
                    let v_swap = ma_liste[index_max];
                    ma_liste[index_max] = ma_liste[index_min];
                    ma_liste[index_min] = v_swap;
                }
                return;
            }

            // Cas général, menant à 2 appels récursifs, suivis de la fusion des 2 sous-listes
            let mid = (index_min + index_max) / 2;
            let mid_plus_1 = mid + 1;

            // Appels récursifs pour trier chacune des 2 sous-listes
            tri_fusion(ma_liste, Some(index_min), Some(mid), Some(ma_sous_liste_1));
            tri_fusion(ma_liste, Some(mid_plus_1), Some(index_max), Some(ma_sous_liste_1));


            // Fusion des 2 sous-listes
            // On copie la premiere sous-liste (la plus "grosse") dans un tableau à part
            // puis on effectue la fusion dans le tableau principal
            let len_sous_liste_1 = mid - index_min + 1;

            ma_sous_liste_1[..len_sous_liste_1].copy_from_slice(&mut ma_liste[index_min..(mid + 1)]);

            let mut fusion_index_input_1 = 0;
            let mut fusion_index_input_2 = mid_plus_1;

            for fusion_index_output in index_min..(index_max + 1)
            {
                let liste_1_non_epuisee = (fusion_index_input_1 <= (mid - index_min));
                let liste_2_non_epuisee = (fusion_index_input_2 <= index_max);

                if (liste_1_non_epuisee)
                {
                    if (liste_2_non_epuisee)
                    {
                        if (ma_liste[fusion_index_input_2] < ma_sous_liste_1[fusion_index_input_1])
                        {
                            // On copie un élément de la liste 2
                            ma_liste[fusion_index_output] = ma_liste[fusion_index_input_2];
                            fusion_index_input_2 += 1;
                        }
                        else
                        {
                            // En cas d'égalité, on prend en priorité un élement de la première liste
                            // => Tri stable

                            // On copie un élément de la liste 1
                            ma_liste[fusion_index_output] = ma_sous_liste_1[fusion_index_input_1];
                            fusion_index_input_1 += 1;
                        }
                    }
                    else {
                        // Liste 2 épuisée
                        // On copie un élément de la liste 1
                        ma_liste[fusion_index_output] = ma_sous_liste_1[fusion_index_input_1];
                        fusion_index_input_1 += 1;
                    
                    }
                }
                else if (liste_2_non_epuisee)
                {
                    // Liste 1 épuisée
                    // On copie un élément de la liste 2
                    ma_liste[fusion_index_output] = ma_liste[fusion_index_input_2];
                    fusion_index_input_2 += 1;
                }


            }
            
        }
    }

} // fn tri_fusion


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


// Générateur de nombres pseudo aléatoires de type générateur Congruentiel Linéaire
// Implémentation de l'algorithme 'MINSTD' alias 'standard minimal'
// de Park et Miller (1988)
//
// Attention: Ce générateur est très prédictible
// <<<  NE PAS UTILISER CE GENERATEUR ALEATOIRE POUR LA CRYPTOGRAPHIE OU LES JEUX D'ARGENT >>>
//
// https://fr.wikipedia.org/wiki/G%C3%A9n%C3%A9rateur_congruentiel_lin%C3%A9aire
// https://en.wikipedia.org/wiki/Lehmer_random_number_generator
struct rng_minstd
{
    rng_a: u64,
    rng_m: u64,
    state: u32
}

impl rng_minstd {

    // Création d'un nouvelle instance avec une seed = état initial du RNG
    pub fn new(seed: u32) -> rng_minstd
    {
        assert!(seed != 0, "La seed doit être différente de zéro.");
        // Constantes du MINSTD a.k.a Park-Miller RNG
        let a: u64 = 16807;
        let m: u64 = 0x7FFFFFFF;   // 2^31 - 1
        rng_minstd {rng_a: a, rng_m: m, state: seed}
    }

    // Une itération du RNG
    // Renvoie la valeur du nouvel état = Un entier dans l'intervalle [0 , (rng_m - 1)]
    pub fn gen(&mut self) -> u32
    {
        let new_state: u32 = (((self.state as u64) * self.rng_a) % self.rng_m) as u32;
        self.state = new_state;
        return new_state;
    }

    // Renvoie un entier dans l'intervalle 'range'
    pub fn gen_range(&mut self, range: std::ops::Range<usize>) -> u32
    {
        let range_start: u32 = range.start as u32;
        let range_end: u32   = range.end as u32;
        let range_size: u32  = range_end - range_start;
        assert!(range_size >= 1, "La taille du range doit être supérieure ou égale à 1.");

        // On effectue une transformation non biaisée de la sortie du rng
        // au prix de possibles rejets (peu genant si la taille du range <= (2^32 / 10) )
        let max_accepted_without_reject: u32 = range_size * ((self.rng_m as u32) / range_size);
        let mut rng_val: u32 = self.gen();
        while (rng_val > max_accepted_without_reject) {rng_val = self.gen();}

        return range_start + (rng_val % range_size);
    }

}


// Algorithme de Fisher Yates
// Permutation aléatoire (équidistribuée i.e. non biaisée) des élements de la liste
// https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
fn fisher_yates_shuffle(ma_liste: &mut [i32], seed: u32)
{
    // Ancien code faisant appel à un RNG externe
    //let mut rng = thread_rng();
    // https://rust-random.github.io/book/guide-seeding.html
    // https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
    // Ancien code (2) faisant appel à un RNG externe
    //use rand::prelude::*;
    //let mut rng = StdRng::seed_from_u64(seed);

    // Utiliser l'implémentation locale du RNG MINSTD pour éviter la dépendance au crate 'rand'
    let mut rng: rng_minstd = rng_minstd::new(seed);

    let n: usize = ma_liste.len();
    //for i from n−1 down to 1 do
    for i in (0..n).rev()
    {
        // https://rust-random.github.io/book/quick-start.html
        // j ← random integer such that 0 ≤ j ≤ i
        let j: usize = rng.gen_range(0..(i + 1)) as usize;

        // exchange a[j] and a[i]
        let v_swap = ma_liste[i];
        ma_liste[i] = ma_liste[j] ;
        ma_liste[j] = v_swap;
    }
}



fn main() {
    println!("Hello, world!");

    let b_test_fonctions_math = false;
    let b_test_recherche_liste_et_tris = true;

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
            let mut rng: rng_minstd = rng_minstd::new(seed);
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
        fisher_yates_shuffle(ma_liste2, seed);
        println!("\nListe mélangée: \n {:?}", &ma_liste2);

        let p: i32 = 3;
        println!("\nRecherche lineaire de la valeur {}: index {} \n", p, recherche_lineaire(ma_liste2, p).unwrap());
        //println!("Recherche dichotomique de la valeur {}: index {}", p, recherche_dichotomique(ma_liste2, p, None, None).unwrap());

        //tri_par_selection(ma_liste2);
        tri_fusion(ma_liste2, None, None, None);

        println!("Liste triée: \n{:?}", &ma_liste2);
        assert!(verif_liste_croissante(&ma_liste2), "Erreur: la liste n'est pas correctement triée.");
        
    }

}
