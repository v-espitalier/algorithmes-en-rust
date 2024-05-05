
// Ce fichier module réunit des 'variantes' d'implémentation des algorithmes de tri
// par exemple:
// - Utilise les fonctions dites 'generiques' de Rust
// - Implémentation en tri indirect: Retourne la permutation pour trier les donner, plutot que les données triées
// - Implémentation optimisée/peformante du tri (avec un nombre limité d'allocations par ex)
// - Tri avec option multi-threading etc.
// - Version modifiée de l'algorithme de tri, suivant la littérature


// Implémentation du tri par insertion générique
// Permet de trier n'importe quels tableaux dont le type des éléments implémente les traits:
// - trait Ord (2 éléments peuvent toujours être ordonnés: x <= y ou y <= x)
// - trait Clone (Un élément peut être duppliqué)
// Fonction pour tous les types d'entiers u32, i64, de flottants f32, les chaines de caractères..
pub fn tri_par_insertion_generique<T>(ma_liste : &mut [T])
where T : Ord, T : Clone
{
    println!("tri_par_insertion_generique > appel");

    let n: usize = ma_liste.len();

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        let mut m: T = ma_liste[i].clone();

        // Déplacement des éléments d'index < i,  et plus grands que m en valeur 
        // pour faire l'insertion de l'élément m ( d'origine ma_liste[i] ).
        let mut insert_index = 0;
        for j in (0..i).rev()
        {
            // On s'arrete quand les élements du tableau sont plus petits que m
            // En cas d'égalité, on break => tri stable
            if (ma_liste[j] <= m) {insert_index = j + 1; break;}
            ma_liste[j + 1] = ma_liste[j].clone();
        }

        // Insertion effective de l'élément m, à l'index 'insert_index'
        ma_liste[insert_index] = m;

        // Invariant de boucle:
        // A la fin de chaque itération, les (i+1) premiers éléments
        // du tableau 'ma_liste' sont triés
    }

    // 
    // return ma_liste
} // fn tri_par_insertion_generique ()


// Fonction qui trouve le minimum pour tout tableau
// dont les éléments peuvent être comparés
// Utilisé pour le tri par sélection branchless et indirect en dessous
pub fn min_array_indirect<T : Ord>(ma_liste : &[T], permutation : &[usize]) -> usize
{
    let n = permutation.len();
    let mut min_index: usize = 0;
    for i in 1..n
    {
        if (ma_liste[permutation[i]] < ma_liste[permutation[min_index]])
        {
            min_index = i;
        }
    }

    return min_index;
}

// Tri par sélection: Version tri indirect + sans branche + en générique: Retourne la permutation
// le tableau n'est plus modifié -> plus mutable
pub fn tri_par_selection_indirect_generique<T>(ma_liste : &[T]) -> Vec<usize>
where T : Ord //, T : Clone
{
    println!("tri_par_selection_indirect_generique > appel");

    let n = ma_liste.len();
    let mut permutation: Vec<usize> = Vec::from_iter((0..n));
    //let permutation: &mut [usize] = permutation_vec.as_mut_slice();

    // Cas particulier d'une liste réduite à un seul élément.
    if n <= 1 {return permutation;}

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        //println!("Itération interne: {}", i);
        let min_index = i + min_array_indirect(&ma_liste, &permutation.as_slice()[i..n]);
        //println!("min_index: {}", min_index);

        // Le i-eme élement le plus petit du tableau se trouve en position m_index, et vaut m
        // On permute les élement d'index i et m_index
        if (i != min_index)
        {
            //println!("Echange les index {} et {}", i, m_index);
            let index_swap = permutation[i];
            permutation[i] = permutation[min_index];
            permutation[min_index] = index_swap;
        }

    }

    return permutation;
} // fn tri_par_selection_indirect_generique ()


pub fn permute_copie_liste<T>(ma_liste : &[T], permutation : &[usize]) -> Vec<T>
where T : Ord, T : Clone
{
    // La permutation indique les index des élements du tableau initial,
    // à prendre successivement afin qu'ils apparaissent triés.
    let n = ma_liste.len();
    
    // Allocation de la taille des données triées, que l'on va copier
    let mut ma_liste_triee: Vec<T> = Vec::new();

    for i in 0..n
    {
        ma_liste_triee.push(ma_liste[permutation[i]].clone());
    }

    return ma_liste_triee;
}





// Algorithme du tri fusion - implémentation améliorée (une seule allocation mémoire supplémentaire)
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// Implémentation tri stable et en place
// Complexité: n.log(n), en moyenne et dans le pire cas
// Version optimisée: Une seule allocation interne au premier appel à la fonction, de taille moitié du tableau d'origine
// Voir: https://fr.wikipedia.org/wiki/Tri_fusion
pub fn tri_fusion_ameliore(ma_liste: &mut [i32], index_min_opt : Option<usize>, index_max_opt : Option<usize>, ma_sous_liste_1_opt: Option<&mut [i32]>)
{
    let arguments_manquants: bool = (index_min_opt == None) || (index_max_opt == None) 
                                        || ((ma_sous_liste_1_opt.as_ref()).is_none());
    if arguments_manquants
    {
        println!("Appel à tri_fusion_ameliore");
        // On traite le tout premier appel à la fonction, effectué par l'utilisateur (i.e. non récursif)
        let len_liste = ma_liste.len();
        let len_sous_liste_1: usize  = len_liste / 2 + 1;
        // Unique allocation de cet algorithme - Effectué une seule fois lors de l'appel utilisateur
        let mut ma_sous_liste_1_vec: Vec<i32> = vec![0 as i32; len_sous_liste_1];
        let ma_sous_liste_1: &mut [i32] = &mut ma_sous_liste_1_vec.as_mut_slice();
        // Relance la fonction, avec le tableau alloué, et les bons index cette fois
        return tri_fusion_ameliore(ma_liste, Some(0), Some(len_liste - 1), Some(ma_sous_liste_1));
    }

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
    tri_fusion_ameliore(ma_liste, Some(index_min), Some(mid), Some(ma_sous_liste_1));
    tri_fusion_ameliore(ma_liste, Some(mid_plus_1), Some(index_max), Some(ma_sous_liste_1));


    // Fusion des 2 sous-listes
    // On copie la premiere sous-liste dans un tableau à part
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

} // fn tri_fusion_ameliore

