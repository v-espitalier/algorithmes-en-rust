
// Ce fichier module réunit des 'variantes' d'implémentation des algorithmes de tri
// par exemple:
// - Utilise les fonctions dites 'generiques' de Rust, basées sur les traits.
// - Implémentation en tri indirect: Retourne la permutation pour trier les donner, plutot que les données triées
// - Implémentation optimisée/peformante du tri (avec un nombre limité d'allocations par ex)
// - Version modifiée de l'algorithme de tri, suivant la littérature


// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Ne pas faire de warning si des fonctions ne sont pas appelées
#![warn(dead_code)]

// Implémentation du tri par insertion, de façon générique (au sens de Rust)
// Permet de trier n'importe quels tableaux dont le type des éléments implémente les traits:
// - trait PartialOrd (On dispose de la comparaison d'éléments: x <= y ou y <= x)
// - trait Clone (Un élément peut être duppliqué)
// Fonction pour tous les types d'entiers u32, i64, de flottants f32, les chaines de caractères..
pub fn tri_par_insertion_generique<T>(mon_tableau : &mut [T])
where T : PartialOrd, T : Clone
{
    println!("tri_par_insertion_generique > appel");

    let n: usize = mon_tableau.len();

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        let m: T = mon_tableau[i].clone();

        // Déplacement des éléments d'index < i,  et plus grands que m en valeur 
        // pour faire l'insertion de l'élément m ( d'origine mon_tableau[i] ).
        let mut insert_index = 0;
        for j in (0..i).rev()
        {
            // On s'arrete quand les élements du tableau sont plus petits que m
            // En cas d'égalité, on break => tri stable
            if (mon_tableau[j] <= m) {insert_index = j + 1; break;}
            mon_tableau[j + 1] = mon_tableau[j].clone();
        }

        // Insertion effective de l'élément m, à l'index 'insert_index'
        mon_tableau[insert_index] = m;

        // Invariant de boucle:
        // A la fin de chaque itération, les (i+1) premiers éléments
        // du tableau 'mon_tableau' sont triés
    }

} // fn tri_par_insertion_generique ()


// Fonction qui trouve le minimum pour tout tableau
// dont les éléments peuvent être comparés
// Utilisé pour le tri par sélection branchless et indirect en dessous
pub fn min_array_indirect<T : Ord>(mon_tableau : &[T], permutation : &[usize]) -> usize
{
    let n = permutation.len();
    let mut min_index: usize = 0;
    for i in 1..n
    {
        if (mon_tableau[permutation[i]] < mon_tableau[permutation[min_index]])
        {
            min_index = i;
        }
    }

    return min_index;
}

// Tri par sélection: Version tri indirect + sans branche + en générique: Retourne la permutation
// le tableau n'est plus modifié -> plus mutable
pub fn tri_par_selection_indirect_generique<T>(mon_tableau : &[T]) -> Vec<usize>
where T : Ord //, T : Clone
{
    println!("tri_par_selection_indirect_generique > appel");

    let n = mon_tableau.len();
    let mut permutation: Vec<usize> = Vec::from_iter((0..n));
    //let permutation: &mut [usize] = permutation_vec.as_mut_slice();

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        //println!("Itération interne: {}", i);
        let min_index = i + min_array_indirect(&mon_tableau, &permutation.as_slice()[i..n]);
        //println!("min_index: {}", min_index);

        // Le i-eme élement le plus petit du tableau se trouve en position m_index, et vaut m
        // On permute les élement d'index i et m_index
        //println!("Echange les index {} et {}", i, m_index);
        let index_swap = permutation[i];
        permutation[i] = permutation[min_index];
        permutation[min_index] = index_swap;

    }

    return permutation;
} // fn tri_par_selection_indirect_generique ()


pub fn permute_copie_tableau<T>(mon_tableau : &[T], permutation : &[usize]) -> Vec<T>
where T : Ord, T : Clone
{
    // La permutation indique les index des élements du tableau initial,
    // à prendre successivement afin qu'ils apparaissent triés.
    let n = mon_tableau.len();
    
    // Allocation de la taille des données triées, que l'on va copier
    let mut mon_tableau_trie: Vec<T> = Vec::new();

    for i in 0..n
    {
        mon_tableau_trie.push(mon_tableau[permutation[i]].clone());
    }

    return mon_tableau_trie;
}





// Algorithme du tri fusion - implémentation améliorée (une seule allocation mémoire supplémentaire)
// Entrée = Sortie:
// mon_tableau: tableau d'entiers 'mon_tableau'
// Implémentation tri stable et en place
// Complexité: n.log(n), en moyenne et dans le pire cas
// Version optimisée: Une seule allocation interne au premier appel à la fonction, de taille moitié du tableau d'origine
// Voir: https://fr.wikipedia.org/wiki/Tri_fusion
#[allow(dead_code)]
pub fn tri_fusion_ameliore(mon_tableau: &mut [i32], index_min_opt : Option<usize>, index_max_opt : Option<usize>, mon_sous_tableau_1_opt: Option<&mut [i32]>)
{
    let arguments_manquants: bool = (index_min_opt == None) || (index_max_opt == None) 
                                        || ((mon_sous_tableau_1_opt.as_ref()).is_none());
    if arguments_manquants
    {
        println!("Appel à tri_fusion_ameliore");
        // On traite le tout premier appel à la fonction, effectué par l'utilisateur (i.e. non récursif)
        let len_tableau = mon_tableau.len();
        let len_sous_tableau_1: usize  = len_tableau / 2 + 1;
        // Unique allocation de cet algorithme - Effectué une seule fois lors de l'appel utilisateur
        let mut mon_sous_tableau_1_vec: Vec<i32> = vec![0 as i32; len_sous_tableau_1];
        let mon_sous_tableau_1: &mut [i32] = &mut mon_sous_tableau_1_vec.as_mut_slice();
        // Relance la fonction, avec le tableau alloué, et les bons index cette fois
        return tri_fusion_ameliore(mon_tableau, Some(0), Some(len_tableau - 1), Some(mon_sous_tableau_1));
    }

    // A partir d'ici, on sait que les arguments optionnels sont effectivement fournis (pas None)
    // index_min_opt, index_max_opt et mon_sous_tableau_1_opt
    // On peut unwrap() sans risque
    let index_min: usize = index_min_opt.unwrap();
    let index_max: usize = index_max_opt.unwrap();
    let mon_sous_tableau_1: &mut [i32] = mon_sous_tableau_1_opt.unwrap();

    let n = index_max - index_min + 1;

    // Gestion des cas particuliers (fin des appels récursifs)
    // tableau avec 1 seul élément (-> Plus rien à trier)
    if n <= 1 {return;}

    // Gestion des cas particuliers (fin des appels récursifs)
    // tableau avec 2 élements
    if n == 2
    {
        // Permuter les elements d'indice 0 et 1 si nécessaire
        if mon_tableau[index_min] > mon_tableau[index_max]
        {
            let v_swap = mon_tableau[index_max];
            mon_tableau[index_max] = mon_tableau[index_min];
            mon_tableau[index_min] = v_swap;
        }
        return;
    }

    // Cas général, menant à 2 appels récursifs, suivis de la fusion des 2 sous-tableaus
    let mid = (index_min + index_max) / 2;
    let mid_plus_1 = mid + 1;

    // Appels récursifs pour trier chacun des 2 sous-tableaus
    tri_fusion_ameliore(mon_tableau, Some(index_min), Some(mid), Some(mon_sous_tableau_1));
    tri_fusion_ameliore(mon_tableau, Some(mid_plus_1), Some(index_max), Some(mon_sous_tableau_1));


    // Fusion des 2 sous-tableaus
    // On copie le premier sous-tableau dans un tableau à part
    // puis on effectue la fusion dans le tableau principal
    let len_sous_tableau_1 = mid - index_min + 1;

    mon_sous_tableau_1[..len_sous_tableau_1].copy_from_slice(&mut mon_tableau[index_min..(mid + 1)]);

    let mut fusion_index_input_1 = 0;
    let mut fusion_index_input_2 = mid_plus_1;

    for fusion_index_output in index_min..(index_max + 1)
    {
        let tableau_1_non_epuise = (fusion_index_input_1 <= (mid - index_min));
        let tableau_2_non_epuise = (fusion_index_input_2 <= index_max);

        if (tableau_1_non_epuise)
        {
            if (tableau_2_non_epuise)
            {
                if (mon_tableau[fusion_index_input_2] < mon_sous_tableau_1[fusion_index_input_1])
                {
                    // On copie un élément du tableau 2
                    mon_tableau[fusion_index_output] = mon_tableau[fusion_index_input_2];
                    fusion_index_input_2 += 1;
                }
                else
                {
                    // En cas d'égalité, on prend en priorité un élement du premier tableau
                    // => Tri stable

                    // On copie un élément du tableau 1
                    mon_tableau[fusion_index_output] = mon_sous_tableau_1[fusion_index_input_1];
                    fusion_index_input_1 += 1;
                }
            }
            else {
                // tableau 2 épuisé
                // On copie un élément du tableau 1
                mon_tableau[fusion_index_output] = mon_sous_tableau_1[fusion_index_input_1];
                fusion_index_input_1 += 1;
            
            }
        }
        else if (tableau_2_non_epuise)
        {
            // tableau 1 épuisé
            // On copie un élément du tableau 2
            mon_tableau[fusion_index_output] = mon_tableau[fusion_index_input_2];
            fusion_index_input_2 += 1;
        }

    }

} // fn tri_fusion_ameliore

