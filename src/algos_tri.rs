
// Algorithme du tri par insertion
// Entrée = Sortie:
// mon_tableau: tableau d'entiers 'mon_tableau'
// Le tri est stable et en place
// Complexité: n^2
// Voir: https://fr.wikipedia.org/wiki/Tri_par_insertion
pub fn tri_par_insertion(mon_tableau: &mut [i32])
{
    println!("tri_par_insertion > appel");

    let n = mon_tableau.len();

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        let mut m = mon_tableau[i];

        // Déplacement des éléments d'index < i,  et plus grands que m en valeur 
        // pour faire l'insertion de l'élément m ( d'origine mon_tableau[i] ).
        let mut insert_index = 0;
        for j in (0..i).rev()
        {
            // On s'arrete quand les élements du tableau sont plus petits que m
            // En cas d'égalité, on break => tri stable
            if (mon_tableau[j] <= m) {insert_index = j + 1; break;}
            mon_tableau[j + 1] = mon_tableau[j];
        }

        // Insertion effective de l'élément m, à l'index 'insert_index'
        mon_tableau[insert_index] = m;

        // Invariant de boucle:
        // A la fin de chaque itération, les (i+1) premiers éléments
        // du tableau 'mon_tableau' sont triés
    }

} // fn tri_par_insertion ()



// Algorithme du tri par sélection
// Entrée = Sortie:
// mon_tableau: tableau d'entiers 'mon_tableau'
// le tableau est trié en place
// Complexité: n^2
// Plus précisément: (n(n-1)/2 comparaisons exactement, (n-1) permutations maximum)
// Voir: https://fr.wikipedia.org/wiki/Tri_par_s%C3%A9lection
pub fn tri_par_selection(mon_tableau: &mut [i32])
{
    println!("tri_par_selection > appel");

    let n = mon_tableau.len();

    // Cas particulier d'un tableau réduit à un seul élément.
    if n <= 1 {return;}

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        let mut m = mon_tableau[i];
        let mut m_index = i;

        // Recherche du plus petit élément, parmi les éléments non triés
        for j in (i+1)..n
        {
            let v = mon_tableau[j];
            if v < m {m = v; m_index = j;}

        }

        // Le i-eme élement le plus petit du tableau se trouve en position m_index, et vaut m
        // On permute les élement d'index i et m_index
        if (i != m_index)
        {
            //println!("Echange les index {} et {}", i, m_index);
            let v_swap = mon_tableau[i];
            mon_tableau[i] = m;
            mon_tableau[m_index] = v_swap;
        }

        // Invariant de boucle:
        // A la fin de chaque itération, les (i+1) premiers éléments
        // du tableau 'mon_tableau' sont triés, et sont les plus petits de tout le tableau.
    }

} // fn tri_par_selection ()


// Algorithme du tri rapide
// Entrée = Sortie:
// mon_tableau: tableau d'entiers 'mon_tableau'
// Implémentation tri en place, non stable
// Complexité: n.log(n), en moyenne, et n^2 dans le pire cas
// Version standard, non optimisée: Allocation interne à chaque appel pour construire les sous-tableaus
// Voir: https://fr.wikipedia.org/wiki/Tri_rapide
pub fn tri_rapide(mon_tableau: &mut [i32])
{
    let n = mon_tableau.len();

    // Gestion des cas particuliers (fin des appels récursifs)
    // tableau avec 1 seul élément (-> Plus rien à trier)
    if n <= 1 {return;}

    // Cas général, menant à 2 appels récursifs pour trier les 2 sous-tableaus

    let pivot = mon_tableau[0];  // Existe car le tableau a au moins 1 éléments

    let mut mon_tableau_gauche_vec: Vec<i32> = Vec::new();
    let mut mon_tableau_droite_vec: Vec<i32> = Vec::new();

    for index in 1..n
    {
        let valeur = mon_tableau[index];
        if (valeur <= pivot) {mon_tableau_gauche_vec.push(valeur);}
        if (valeur > pivot) {mon_tableau_droite_vec.push(valeur);}

    }

    let mon_tableau_gauche: &mut [i32] = mon_tableau_gauche_vec.as_mut_slice();
    let mon_tableau_droite: &mut [i32] = mon_tableau_droite_vec.as_mut_slice();
    tri_rapide(mon_tableau_gauche);
    tri_rapide(mon_tableau_droite);

    // Rassemble les sous-tableaus triés en les copiant dans le tableau d'origine
    let mut cpt:usize = 0;
    for index in 0..mon_tableau_gauche.len()
    {
        mon_tableau[cpt] = mon_tableau_gauche[index];
        cpt += 1;
    }

    mon_tableau[cpt] = pivot;
    cpt += 1;

    for index in 0..mon_tableau_droite.len()
    {
        mon_tableau[cpt] = mon_tableau_droite[index];
        cpt += 1;
    }

}


// Algorithme du tri fusion
// Entrée = Sortie:
// mon_tableau: tableau d'entiers 'mon_tableau'
// Implémentation tri stable et en place
// Version standard, non optimisée: Allocation interne à chaque appel pour fusionner les sous-tableaus
// Complexité: n.log(n), en moyenne et dans le pire cas
// Voir: https://fr.wikipedia.org/wiki/Tri_fusion
pub fn tri_fusion(mon_tableau: &mut [i32])
{
    let n = mon_tableau.len();

    // Gestion des cas particuliers (fin des appels récursifs)
    // tableau avec 1 seul élément (-> Plus rien à trier)
    if n <= 1 {return;}

    // Gestion des cas particuliers (fin des appels récursifs)
    // tableau avec 2 élements
    if n == 2
    {
        // Permuter les elements d'indice 0 et 1 si nécessaire
        if mon_tableau[0] > mon_tableau[1]
        {
            let v_swap = mon_tableau[1];
            mon_tableau[1] = mon_tableau[0];
            mon_tableau[0] = v_swap;
        }
        // Algorithme 'en place': On modifie directement le tableau en entrée,
        // donc pas de valeur de retour
        return;
    }

    // Cas général, menant à 2 appels récursifs, suivis de la fusion des 2 sous-tableaus
    let mid = n / 2;

    // On extrait les 2 sous-tableaus: Moitiés gauche et droite du tableau
    // https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
    let (mon_sous_tableau_gauche, mon_sous_tableau_droite) = mon_tableau.split_at_mut(mid);


    // Appels récursifs pour trier chacun des 2 sous-tableaus
    tri_fusion(mon_sous_tableau_gauche);
    tri_fusion(mon_sous_tableau_droite);


    // Fusion des 2 sous-tableaus
    let mut mon_tableau_bis_vec: Vec<i32> = Vec::new();

    let mut mon_tableau_gauche_index: usize = 0;
    let mut mon_tableau_droite_index: usize = 0;

    // Fusion (itérative) des 2 tableaus
    // On boucle sur le tableau principal, que l'on remplit.
    //
    // A chaque itération, on prend le plus petit élément des 2 tableaus
    // et on le met dans le tableau principal
    // 
    // Invariant de boucle: A la fin de chaque itération, le tableau principal
    // contient les 'index' élements les plus petits des 2 sous-tableaus.
    for index in 0..n
    {
        if (mon_tableau_gauche_index < mon_sous_tableau_gauche.len()) &&
        (mon_tableau_droite_index < mon_sous_tableau_droite.len())
        {
            // Cas général ou les 2 sous-tableaus contiennent encore des élements à traiter
            let v_gauche = mon_sous_tableau_gauche[mon_tableau_gauche_index];
            let v_droite = mon_sous_tableau_droite[mon_tableau_droite_index];
            if v_gauche <= v_droite
            {
                mon_tableau_bis_vec.push(v_gauche);
                mon_tableau_gauche_index += 1;
            }
            else
            {
                mon_tableau_bis_vec.push(v_droite);
                mon_tableau_droite_index += 1;
            }
        }
        else if (mon_tableau_gauche_index < mon_sous_tableau_gauche.len())
        {
            // le tableau droit a été entièrement traité
            // On prend un élément du tableau gauche
            let v_gauche = mon_sous_tableau_gauche[mon_tableau_gauche_index];
            mon_tableau_bis_vec.push(v_gauche);
            mon_tableau_gauche_index += 1;
        }
        else if (mon_tableau_droite_index < mon_sous_tableau_droite.len())
        {
            // le tableau gauche a été entièrement traité
            // On prend un élément du tableau droite
            let v_droite = mon_sous_tableau_droite[mon_tableau_droite_index];
            mon_tableau_bis_vec.push(v_droite);
            mon_tableau_droite_index += 1;
        }
        else
        {
            // Ce cas ne devrait jamais se produire
            panic!("Erreur interne: le tableau principale n'est pas remplie, mais les 2 sous-tableaus ont été entèrement traitées.")
        }

    }  // for index in 0..n

    let mon_tableau_bis: &mut [i32] = mon_tableau_bis_vec.as_mut_slice();
    mon_tableau.clone_from_slice(&mon_tableau_bis);
    return;

} // fn tri_fusion



// Algorithme du tri par tas
// Entrée = Sortie:
// mon_tableau: tableau d'entiers 'mon_tableau'
// Implémentation tri en place, non stable
// Complexité: n.log(n), en moyenne et dans le pire cas
// Utilise la structure de données 'std::collections::BinaryHeap' de Rust.
// On insère tous les éléments dans le tas, puis on les extrait.
// Ils ressortent triés par ordre décroissant (BinaryHeap = File de priorité = Plus grand élément d'abord)
// On remplit donc le tableau d'entrée/sortie à l'envers
// Voir: https://fr.wikipedia.org/wiki/Tri_par_tas
pub fn tri_par_tas(mon_tableau: &mut [i32])
{
    let n = mon_tableau.len();

    // tableau avec 1 seul élément (-> rien à trier)
    if n <= 1 {return;}

    // Cas général avec au moins 2 éléments à trier
    use std::collections::BinaryHeap ;
    let mut binary_heap = BinaryHeap::new() ;
    for i in 0..n
    {
        // On insère tous les éléments à trier dans le tas
        binary_heap.push(mon_tableau[i]) ;
    }

    for i in (0..n).rev()
    {
        // On retire tous les éléments du tas, qui sortent par ordre décroissant
        // (binary_heap = tas = File de priorité => Les premiers éléments retournés sont de valeur maximale)
        let v_opt = binary_heap.pop();
        assert_ne!(v_opt, None, "tri_par_tas : Erreur interne (1): Il devrait rester des éléments dans le tas.");

        let v: i32 = v_opt.unwrap().clone();
        mon_tableau[i] = v;

        // Invariant de boucle: A la fin de chaque itération:
        // - les (n - i) derniers éléments de mon_tableau sont triés et les plus grands du tableau initial
        //   (i.e. ils sont à leur place finale)
        // - Le tas contient les éléments restants, et la racine pointe vers le plus grand élément du tas
    }
    assert_eq!(binary_heap.pop(), None, "tri_par_tas : Erreur interne (2) : Il ne devrait plus rester d'élément dans le tas.");

} // fn tri_par_tas

