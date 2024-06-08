
// Module contenant des algorithmes divers et variés

// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Pour l'assembleur
use std::arch::asm;

// Pour le multithreading
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};


// Algorithme résolvant le problème des 8 dames
// https://fr.wikipedia.org/wiki/Probl%C3%A8me_des_huit_dames
// Il ne peut y avoir qu'un dame par colonne et par ligne, et il y a 8 dames à placer
// donc il y a exactement une dame par colonne et par ligne.
//
// Il suffit donc:
// 1) De déterminer, pour chaque colonne i, quel est l'index de la ligne j ou se trouve la dame.
//    Ainsi 1 position = 1 tableau de taille 8 contenant des entiers entre 1 et 8:   t[i] = j
// 2) Tous les index de ligne sont distincts (les t[i]), donc cela donne
//    8x7x6x5x4x3x2x1 = 8! = 40,320 possibilités et positions à tester.
// 3) Pour chacune de ces positions, il reste à vérifier qu'il y a une seule dame par diagonale
//    Pour désigner une case, on note i l'index de colonne, et j l'index de ligne. Avec cette notation
//    toutes les cases appartenant à une meme diagonale principale verifient i+j = constante
//    et toutes les cases d'une diagonale secondaire (autre sens) i - j = constante
//    Il faut donc que notre tableau vérifie : (i + t[i]) est unique pour tout i, et (i - t[i]) aussi.
//
// Implémentation: En mode itératif, avec les 8 boucles imbriquées parcourant les 8! permutations
//                 et en breakant dés que 2 dames se trouvent sur une même diagonale
//                 En pratique, on regroupe les 8 boucles, pour 'simplifier' l'implémentation
//                 Breaker à cause de la diagonale revient à incrémenter l'index de ligne,
//                 de la colonne causant le conflit (i.e. pruning dans le parcours en profondeur).
//
// Remarque: * Pour la première colonne, on explore seulement les 4 premiere lignes.
//           On déduira les autres solutions par symétrie, apres le parcours.
//           Il reste 20,160 possibilités à explorer
//           (Etant donné qu'il y a au moins 4 symétries axiales et 3 rotations, on pourrait réduire
//            encore l'espace de recherche, au prix d'une lisibilité amoindrie de l'algo)
//           * L'algo, suffisamment simple, pourrait meme faire l'objet d'une implémentation en assembleur.
//             On peut stocker 1 position complete de l'échiquier avec les 8 dames sur un registre de 32 bits,
//             avec 3 bits par colonne/dame, donc sans "compression/optimisation", sans la perte de lisibilité associée.

// Fonction utilisée pour les permutations
// Renvoie la k-ieme case libre, i.e. pas prise par les profond
fn trouve_k_ieme_case_libre(k: usize, profondeur: usize, cases_prises: &[usize; 8]) -> usize
{
    let mut k_decrement = k;
    for i in 0..8
    {
        let mut prise: bool = false;
        for j in 0..profondeur
        {
            if cases_prises[j] == i
            {
                prise = true;
                break;
            }

        }
        if (!prise)
        {
            if (k_decrement == 0)
            {
                return i;
            }
            else {
                k_decrement -= 1;
            }
        }
    }

    // On ne devrait jamais arriver ici
    panic!("trouve_k_ieme_case_libre: Erreur interne");
    //return 10;
}

fn avance_a_la_prochaine_position(solution_relative_cour : &mut [usize; 8], index_pruning: usize)
{
    // # Gérer la retenue par récursivité #

    // On itère sur les 8! = 8x7x6x5x4x3x2x1 positions possibles des dames
    // max_index = [0..7] x [0..6] x [0..5] x ... x [0..1] x [0..0]
    let max_index: usize = 7 - index_pruning;

    // Si on déborde, on remet à zéro, et on propage la retenue (vers à gauche)
    if solution_relative_cour[index_pruning] == max_index
    {
        solution_relative_cour[index_pruning] = 0;
        avance_a_la_prochaine_position(solution_relative_cour, index_pruning - 1);
    }
    else {
        // Pas de retenue: Simple incrément
        solution_relative_cour[index_pruning] += 1;
    }
}

// Sortie = ensemble des solutions du problème
pub fn resoud_probleme_des_8_dames() -> Vec<[usize; 8]>
{
    println!("Appel à resoud_probleme_des_8_dames()");
    // Vecteur de solutions
    let mut solutions: Vec<[usize; 8]> = Vec::new();

    let mut solution_relative_cour: [usize; 8];   // Les premiers index de lignes, de la solution en construction
    let mut solution_absolue_cour: [usize; 8] = [0; 8];   // Les premiers index de lignes, de la solution en construction
    let mut diag1: [usize; 8] = [0; 8];   // i+j: Toujours positif
    let mut diag2: [isize; 8] = [0; 8];   // i-j: Peut etre négatif

    // Solution de départ: [0, 0, 0, 0, 0, 0, 0, 0]. Les indices des lignes et colonnes démarrent à zéro en Rust
    // Ce sont des index relatifs. Sur un échiquier, cela correspond aux cases [a1, b2, c3, d4, e5, f6, g7, h8]
    solution_relative_cour = [0; 8];
    let mut n_positions_testees: usize = 0; // Pour verif
    loop
    {
        // Calcul de la position absolue à partir de la position relative
        // La premiere colonne ne pose jamais de conflit de lignes/colonnes
        solution_absolue_cour[0] = solution_relative_cour[0];
        for i in 0..8
        {
            solution_absolue_cour[i] = trouve_k_ieme_case_libre(solution_relative_cour[i], i, &solution_absolue_cour);
        }

        // Vérification des diagonales
        let mut index_pruning = 8;
        'boucle1: for i in 0..8
        {
            diag1[i] = i + solution_absolue_cour[i];
            diag2[i] = (i as isize) - (solution_absolue_cour[i] as isize);
            for j in 0..i
            {
                if (diag1[i] == diag1[j]) || (diag2[i] == diag2[j])
                    {index_pruning = i; break 'boucle1;}
            }
        }
        if (index_pruning == 8)
        {
            // index_pruning = 8 => Pas de conflit de diagonale: On a trouvé une nouvelle solution
            solutions.push(solution_absolue_cour.clone());
            index_pruning = 7; // On incrémentera le dernier index
        }
        n_positions_testees += 1;

        // On passe à la position suivante, mais on élage (prune) à partir du conflit de diagonale
        avance_a_la_prochaine_position(&mut solution_relative_cour, index_pruning);

        // Terminaison de l'algorithme: On a testé les 4 premieres lignes pour la dame de la premiere colonne
        // On pourra déduire les solutions manquantes par symétrie
        if solution_relative_cour[0] > 3 {
            println!("nombre de positions testées avec pruning: {}", n_positions_testees);
            break; 
        }
    }

    // On ajoute les positions manquantes, par symétrie
    let n_sol_sans_symetrie = solutions.len();
    for index_sol in 0..n_sol_sans_symetrie
    {
        let sol_prec = solutions[(n_sol_sans_symetrie - 1) - index_sol];
        let mut sol_new: [usize; 8] = [0 ; 8];
        for i in 0..8
        {
            sol_new[i] = 7 - sol_prec[i];
        }
        solutions.push(sol_new.clone());

    }


    return solutions;
}

pub fn affiche_solutions_probleme_des_8_dames(solutions: &Vec<[usize; 8]>)
{
    // Afficher la dame en couleur, avec les 'escape sequences'
    let dame_en_couleur = "\x1b[93m*\x1b[0m";
    let dame: &str = dame_en_couleur; // "*";
    for sol_index in 0..solutions.len()
    {
        let sol_cour = solutions[sol_index];
        println!("Solution d'index {}", (sol_index + 1));
        println!("");
        println!("   a b c d e f g h");
        for i in 0..8
        {
            let mut rev_index = 8;
            for j in 0..8
            {
                if sol_cour[j] == (7 - i) {rev_index = j; break;}
            }
            let spaces: String = " -".repeat(rev_index);
            let spaces_after: String = " -".repeat(7 - rev_index);
            println!("{} {} {}{}  {}", (8 - i), spaces, dame, spaces_after, (8 - i));
        }
        println!("   a b c d e f g h");
        println!("");
        println!("");
    }

}

pub fn calcule_symetries_rotations(solution: &[usize; 8]) -> Vec<[usize; 8]>
{
    // Trouve toutes les positions déductibles de la solution actuelle
    // par symétrie ou rotation
    let mut solutions_multiples_cour: Vec<[usize; 8]> = Vec::new();

    solutions_multiples_cour.push(solution.clone());


    let mut solution_transformee: [usize; 8] = [0 ; 8];
    // Symétrie échangeant A1 et A8
    for i in 0..8
    {
        solution_transformee[i] = 7 - solution[i];
    }
    solutions_multiples_cour.push(solution_transformee.clone());

    // Symétrie échangeant A1 et H1
    for i in 0..8
    {
        solution_transformee[i] = solution[7 - i];
    }
    solutions_multiples_cour.push(solution_transformee.clone());

    // Symétrie centrale (rotation 180 degres) échangeant A1 et H8
    for i in 0..8
    {
        solution_transformee[i] = 7 - solution[7 - i];
    }
    solutions_multiples_cour.push(solution_transformee.clone());

    // Les 2 symétries axiales selon les diagonales
    for i in 0..8
    {
        solution_transformee[solution[i]] = i;
    }
    solutions_multiples_cour.push(solution_transformee.clone());

    for i in 0..8
    {
        solution_transformee[7 - solution[i]] = 7 - i;
    }
    solutions_multiples_cour.push(solution_transformee.clone());

    // Rotation de 90 degrés dans le sens trigo inverse, amenant A1 en A8.
    for i in 0..8
    {
        let mut bon_index = 8;
        for j in 0..8
        {
            if solution[j] == i
            {
                bon_index = j;
                break;
            }
        }
        solution_transformee[i] = 7 - bon_index;
    }
    solutions_multiples_cour.push(solution_transformee.clone());

    // Rotation de 90 degrés dans le sens trigo, amenant A1 en H1.
    for i in 0..8
    {
        let mut bon_index = 8;
        for j in 0..8
        {
            if solution[j] == i
            {
                bon_index = j;
                break;
            }
        }
        solution_transformee[7 - i] = bon_index;
    }
    solutions_multiples_cour.push(solution_transformee.clone());



    //println!("sols: {:?}", solutions_multiples_cour);
    return solutions_multiples_cour;
}


// Extrait les 12 solutions uniques du probleme de l'ensemble des solutions trouvées
// Rq: Uniques signifie pas déductibles les unes des autres par rotation ou symétrie
// Les 12 solutions uniques sont les meme que wikipédia, a une symétrie/rotation prés
// https://fr.wikipedia.org/wiki/Probl%C3%A8me_des_huit_dames#Les_solutions
// Correspondances:  Wiki[1] = Rust[3],  W[3] = R[1],  W[10] = R[11],  W[11] = R[12],  W[12] = R[10]
// les autres correspondances sont identiques (index 2 , et de 4 à 9)
pub fn calcule_solutions_uniques(solutions: &Vec<[usize; 8]>) -> Vec<[usize; 8]>
{
    let mut solutions_uniques: Vec<[usize; 8]> = Vec::new();
    let mut solutions_multiples: Vec<[usize; 8]> = Vec::new();


    for sol_index in 0..solutions.len()
    {
        //println!("sol_index: {}, nb sol multiples: {}", sol_index, solutions_multiples.len());
        //println!("{:?}", solutions_multiples);
        //if sol_index > 2 {panic!();}
        let sol_cour = solutions[sol_index];
        let mut trouve = false;
        for i in 0..solutions_multiples.len()
        {
            let sol_multiple_cour = solutions_multiples[i];
            let mut identique = true;
            for j in 0..8
            {
                if sol_cour[j] != sol_multiple_cour[j]
                {
                    identique = false;
                    break;
                }
            }
            if identique
            {
                trouve = true;
                break;
            }
        }
        if !trouve
        {
            solutions_uniques.push(sol_cour);

            let solutions_multiples_cour = calcule_symetries_rotations(&sol_cour);
            for j in 0..solutions_multiples_cour.len()
            {
                solutions_multiples.push(solutions_multiples_cour[j]);
            }
        }

    }

    //println!("Nb solutions_multiples: {}", solutions_multiples.len());

    return solutions_uniques;
}



// Calcul itératif du pgcd des entiers a et b
// Entrée: 2 entiers: a et b
// Sortie: a ^ b = PGCD(a, b) - Plus Grand Commun Diviseur
// Voir: https://fr.wikipedia.org/wiki/Plus_grand_commun_diviseur
// Implémentation avec inline assembleur
// https://doc.rust-lang.org/rust-by-example/unsafe/asm.html
pub fn pgcd_asm(a: u64, b: u64) -> u64
{
    println!("Appel à pgcd_asm");
    // On permute a et b si a < b
    if a < b {return pgcd_asm(b, a);}

    let mut pgcd: u64 = a;

    unsafe {
        asm!(
            // while (b != 0)
            // {
                "123:",        // label pour le jmp (boucle principale)
                "cmp ecx, 0",  // Si b = 0, alors a est le pgcd -> break de la boucle
                "je 456f",
                "mov edx, 0",  // (edx apparait aussi comme operande en entrée de la division euclidienne)
                // Division euclidienne de a par b.
                // [ eax ; edx ] = [ int(eax / ecx) ; eax % ecx ]
                "div ecx",
                // a = b
                "mov eax, ecx",
                // b = r, le reste de la division
                "mov ecx, edx",
                // Fin de la boucle
                "jmp 123b",
                // après la boucle
            // } // fin while
            "456:",
            inout("eax") pgcd,
            in("ecx") b
        );
    }
    return pgcd;

}



// Recherche des nombres premiers compris entre min_n (inclus) et max_n (exclu).
pub fn recherche_premiers(min_n : usize, max_n : usize) -> Vec<usize>
{
    let mut premiers_trouves: Vec<usize> = Vec::new();

    // On gère le cas particulier de l'unique nombre premier pair
    if (min_n <=2) && (max_n > 2) {premiers_trouves.push(2);}

    let min_n_impair: usize = if (min_n <= 2) {3} else {min_n + (1 - (min_n % 2))};

    // A partir d'ici, on va tester uniquement des impairs
    for i in (min_n_impair..max_n).step_by(2)
    {
        let max_j: usize  = f64::sqrt(i as f64) as usize;

        let mut trouve: bool = false;
        for j in (3..(max_j + 1)).step_by(2)
        {
            if (i % j == 0) {trouve = true; break;}
        }
        if (!trouve) {premiers_trouves.push(i);}
    }

    return premiers_trouves;
}


// Recherche des nombres premiers compris entre min_n (inclus) et max_n (exclu).
// par paquets de taille 'batch_size', avec multithreading
// Les paquets sont dispatchés sur les différents cores
// Si min_n et max_n ne sont pas du même ordre de grandeur, les premiers paquets seront plus vite traités
// donc il faut choisir un batch_size, par exemple, de l'ordre de (max_n - min_n) / 100,
// pour faire une centaine de paquets, afin de répartir la charge tout en limitant le nombre de threads.
// Plus rapide que la version sans multithreading, pour des index supérieurs à 1 million / 2 millions.
pub fn recherche_premiers_multithreading(min_n: usize, max_n: usize, batch_size: usize) -> Vec<usize>
{
    println!("Appel à la fonction recherche_premiers_multithreading");
    if (batch_size == 0) {panic!("Erreur dans recherche_premiers_multithreading: Il faut un batch_size non nul");}

    let n_element: usize = max_n - min_n + 1;
    let n_batch: usize = ((n_element as f64) / (batch_size as f64)).ceil() as usize;

    static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

    let mut premiers_trouves: Vec<usize> = Vec::new();
    let shared_premiers_trouves = Arc::new(Mutex::new(premiers_trouves));

    for batch_index in 0..n_batch
    {
        let min_n_batch: usize        = min_n + batch_size * batch_index;
        let max_n_batch_complet:usize = min_n + batch_size * (batch_index + 1);
        let max_n_batch: usize        = if (max_n_batch_complet < max_n) {max_n_batch_complet} else {max_n};

        // Le vecteur d'entiers n'est pas cloné, c'est seulement le smart pointeur
        // pointant vers le vecteur, qui l'est (!!)
        let shared_premiers_trouves_batch = shared_premiers_trouves.clone();

        // Partie du code parrallélisée
        GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
        let handle = std::thread::spawn( move ||
        {
            //
            //println!("debug (2) : {} {} {}", batch_index, min_n_batch, max_n_batch);
            let mut premiers_trouves_batch: Vec<usize> = recherche_premiers(min_n_batch, max_n_batch);

            let mut shared_premiers_trouves_batch_val = shared_premiers_trouves_batch.lock().unwrap();

            // Ligne qui nécessite le mutex/arc
            shared_premiers_trouves_batch_val.append(&mut premiers_trouves_batch);

            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(1));
        });
        //handle.join();
    }

    println!("Attente des threads..");
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1)); 
    }

    println!("Attente terminée..");
    premiers_trouves = shared_premiers_trouves.lock().unwrap().clone(); 

    return premiers_trouves;
}



// Algorithmes relatifs du problème de Syracuse
// https://fr.wikipedia.org/wiki/Conjecture_de_Syracuse
// Calcule les termes de la suite de Syracuse depuis le nombre n
// Renvoie le temps de vol et l'altitude maximale.
pub fn calcule_temps_de_vol_et_altitude_max(n: u64) -> (u64, u64)
{
    let mut temps_de_vol: u64 = 0;
    let mut altitude_max: u64 = n;

    let mut n_cour: u64 = n;
    while (n_cour != 1)
    {
        if ((n_cour % 2) == 0)
        {
            n_cour = n_cour / 2;
        }
        else
        {
            n_cour = 3 * n_cour + 1;

        }
        if (n_cour > altitude_max)
        {
            altitude_max = n_cour;
        }

        temps_de_vol = temps_de_vol + 1;
    }

    return (temps_de_vol, altitude_max);
}


// Calcule les suites de Syracuse inférieures ou égales à n
// Renvoie le temps de vol maximal, et l'index de la suite qui a permis de l'atteindre
// Valeurs de confirmation disponibles sur la page anglaise de Wikipédia:
// https://en.wikipedia.org/wiki/Collatz_conjecture
pub fn calcule_temps_de_vol_max(n_max: u64) -> (u64, u64)
{
    let mut temps_de_vol_max: u64 = 0;
    let mut temps_de_vol_max_index: u64 = 0;

    for n in 1..(n_max + 1)
    {
        let (temps_de_vol, _altitude_max) = calcule_temps_de_vol_et_altitude_max(n);
        if (temps_de_vol > temps_de_vol_max)
        {
            temps_de_vol_max = temps_de_vol;
            temps_de_vol_max_index = n;
        }
    }

    return (temps_de_vol_max, temps_de_vol_max_index);
}


// Meme calcul que la fonction au dessus (calcule_temps_de_vol_max)
// Implémentation en assembleur
// Ne compile pas: A debugger:  "error: Undefined temporary symbol .Ltmp8"
pub fn calcule_temps_de_vol_max_asm(n_max: u64) -> (u64, u64)
{
    let mut temps_de_vol_max: u64 = 0;
    let mut temps_de_vol_max_index: u64 = 0;

    unsafe {
        asm!(

            "xor r9, r9",      // Reset des valeurs de sortie
            "xor r10, r10",

            "mov r11, 3",      // Constante multiplicative apparaissant dans Syracuse


            "mov rcx, 1",      // Variable de boucle

            //for n in 1..(n_max + 1)
            //{
            "201:",
            "cmp rcx, r8",
            "ja 207f",

                // let mut n_cour: u64 = n;
                "mov rax, rcx",  // Variable de boucle (interne)
                "xor r12, r12",  // Stockage temporaire du temps de vol (init à 0)


                // while (n_cour != 1)
                // {
                "202:",
                "cmp rax, 1",
                "je 205f",

                    // if ((n_cour % 2) == 0)
                    "test eax, 1",
                    "jnz  203f",
                    // {
                        // n_cour = n_cour / 2;
                        "shr eax, 1",
                    // }
                    "jmp 204f",

                    // else
                    "203:",
                    // {
                        // n_cour = 3 * n_cour + 1;
                        "imul r11",    // rax = rax * 3  ;  rdx est effacé
                        "inc rax",
                    // }
                    "204:",

                    // temps_de_vol = temps_de_vol + 1;
                    "inc r12",

                // }   end while (n_cour != 1)
                "jmp 202f",

                "205:",
                // if (temps_de_vol > temps_de_vol_max)
                "cmp r12, r9",
                "jbe 206f",
                // {

                    // temps_de_vol_max = temps_de_vol;
                    "mov r9, r12",
                    // temps_de_vol_max_index = n;
                    "mov r10, rcx",
                // }
                "206:",


                "inc rcx",
            // }  // end for n in 1..(n_max + 1)
            "jmp 201f",

            "207:",

            out("r9") temps_de_vol_max,
            out("r10") temps_de_vol_max_index,
            in("r8") n_max
        );
    }

    return (temps_de_vol_max, temps_de_vol_max_index);
}

