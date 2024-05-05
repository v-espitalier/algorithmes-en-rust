
// Algorithme du tri par insertion
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// Le tri est stable et en place
// Complexité: n^2
// Voir: https://fr.wikipedia.org/wiki/Tri_par_insertion
pub fn tri_par_insertion(ma_liste: &mut [i32])
{
    println!("tri_par_insertion > appel");

    let n = ma_liste.len();

    // On trie les élements du tableau, successivement
    // for i in range(0, n):
    for i in 0..n
    {
        let mut m = ma_liste[i];

        // Déplacement des éléments d'index < i,  et plus grands que m en valeur 
        // pour faire l'insertion de l'élément m ( d'origine ma_liste[i] ).
        let mut insert_index = 0;
        for j in (0..i).rev()
        {
            // On s'arrete quand les élements du tableau sont plus petits que m
            // En cas d'égalité, on break => tri stable
            if (ma_liste[j] <= m) {insert_index = j + 1; break;}
            ma_liste[j + 1] = ma_liste[j];
        }

        // Insertion effective de l'élément m, à l'index 'insert_index'
        ma_liste[insert_index] = m;

        // Invariant de boucle:
        // A la fin de chaque itération, les (i+1) premiers éléments
        // du tableau 'ma_liste' sont triés
    }

    // 
    // return ma_liste
} // fn tri_par_insertion ()



// Algorithme du tri par sélection
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// La liste est triée en place
// Complexité: n^2
// Plus précisément: (n(n-1)/2 comparaisons exactement, (n-1) permutations maximum)
// Voir: https://fr.wikipedia.org/wiki/Tri_par_s%C3%A9lection
pub fn tri_par_selection(ma_liste: &mut [i32])
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

        // Invariant de boucle:
        // A la fin de chaque itération, les (i+1) premiers éléments
        // du tableau 'ma_liste' sont triés, et sont les plus petits de tout le tableau.
    }

    // 
    // return ma_liste
} // fn tri_par_selection ()


// Algorithme du tri rapide
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// Implémentation tri en place, non stable
// Complexité: n.log(n), en moyenne, et n^2 dans le pire cas
// Version standard, non optimisée: Allocation interne à chaque appel pour construire les sous-listes
// Voir: https://fr.wikipedia.org/wiki/Tri_rapide
pub fn tri_rapide(ma_liste: &mut [i32])
{
    let n = ma_liste.len();

    // Gestion des cas particuliers (fin des appels récursifs)
    // Liste avec 1 seul élément (-> Plus rien à trier)
    if n <= 1 {return;}

    // Cas général, menant à 2 appels résursifs pour trier les 2 sous-listes

    let pivot = ma_liste[0];  // Existe car la liste a au moins 2 éléments

    let mut ma_liste_gauche_vec: Vec<i32> = Vec::new();
    let mut ma_liste_droite_vec: Vec<i32> = Vec::new();

    for index in 1..n
    {
        let valeur = ma_liste[index];
        if (valeur <= pivot) {ma_liste_gauche_vec.push(valeur);}
        if (valeur > pivot) {ma_liste_droite_vec.push(valeur);}

    }

    let ma_liste_gauche: &mut [i32] = ma_liste_gauche_vec.as_mut_slice();
    let ma_liste_droite: &mut [i32] = ma_liste_droite_vec.as_mut_slice();
    tri_rapide(ma_liste_gauche);
    tri_rapide(ma_liste_droite);

    // Rassemble les sous-listes triées en les copiant dans le tableau d'origine
    let mut cpt:usize = 0;
    for index in 0..ma_liste_gauche.len()
    {
        ma_liste[cpt] = ma_liste_gauche[index];
        cpt += 1;
    }

    ma_liste[cpt] = pivot;
    cpt += 1;

    for index in 0..ma_liste_droite.len()
    {
        ma_liste[cpt] = ma_liste_droite[index];
        cpt += 1;
    }

}


// Algorithme du tri fusion
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// Implémentation tri stable et en place
// Version standard, non optimisée: Allocation interne à chaque appel pour fusionner les sous-listes
// Complexité: n.log(n), en moyenne et dans le pire cas
// Voir: https://fr.wikipedia.org/wiki/Tri_fusion
pub fn tri_fusion(ma_liste: &mut [i32])
{
    let n = ma_liste.len();

    // Gestion des cas particuliers (fin des appels récursifs)
    // Liste avec 1 seul élément (-> Plus rien à trier)
    if n <= 1 {return;}

    // Gestion des cas particuliers (fin des appels récursifs)
    // Liste avec 2 élements
    if n == 2
    {
        // Permuter les elements d'indice 0 et 1 si nécessaire
        if ma_liste[0] > ma_liste[1]
        {
            let v_swap = ma_liste[1];
            ma_liste[1] = ma_liste[0];
            ma_liste[0] = v_swap;
        }
        // Algorithme 'en place': On modifie directement le tableau en entrée,
        // donc pas de valeur de retour
        return;
    }

    // Cas général, menant à 2 appels récursifs, suivis de la fusion des 2 sous-listes
    let mid = n / 2;

    // On extrait les 2 sous-listes: Moitiés gauche et droite du tableau
    // https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
    let (ma_sous_liste_gauche, ma_sous_liste_droite) = ma_liste.split_at_mut(mid);


    // Appels récursifs pour trier chacune des 2 sous-listes
    tri_fusion(ma_sous_liste_gauche);
    tri_fusion(ma_sous_liste_droite);


    // Fusion des 2 sous-listes
    let mut ma_liste_bis_vec: Vec<i32> = Vec::new();

    let mut ma_liste_gauche_index: usize = 0;
    let mut ma_liste_droite_index: usize = 0;

    // Fusion (itérative) des 2 listes
    // On boucle sur le tableau principal, que l'on remplit.
    //
    // A chaque itération, on prend le plus petit élément des 2 listes
    // et on le met dans le tableau principal
    // 
    // Invariant de boucle: A la fin de chaque itération, le tableau principal
    // contient les 'index' élements les plus petits des 2 sous-listes.
    for index in 0..n
    {
        if (ma_liste_gauche_index < ma_sous_liste_gauche.len()) &&
        (ma_liste_droite_index < ma_sous_liste_droite.len())
        {
            // Cas général ou les 2 sous-listes contiennent encore des élements à traiter
            let v_gauche = ma_sous_liste_gauche[ma_liste_gauche_index];
            let v_droite = ma_sous_liste_droite[ma_liste_droite_index];
            if v_gauche <= v_droite
            {
                ma_liste_bis_vec.push(v_gauche);
                ma_liste_gauche_index += 1;
            }
            else
            {
                ma_liste_bis_vec.push(v_droite);
                ma_liste_droite_index += 1;
            }
        }
        else if (ma_liste_gauche_index < ma_sous_liste_gauche.len())
        {
            // La liste droite a été entièrement traitée
            // On prend de la liste gauche
            let v_gauche = ma_sous_liste_gauche[ma_liste_gauche_index];
            ma_liste_bis_vec.push(v_gauche);
            ma_liste_gauche_index += 1;
        }
        else if (ma_liste_droite_index < ma_sous_liste_droite.len())
        {
            // La liste gauche a été entièrement traitée
            // On prend de la liste droite
            let v_droite = ma_sous_liste_droite[ma_liste_droite_index];
            ma_liste_bis_vec.push(v_droite);
            ma_liste_droite_index += 1;
        }
        else
        {
            // Ce cas ne devrait jamais se produire
            println!("Erreur interne: La liste principale n'est pas remplie, mais les 2 sous-listes ont été entèrement traitées.")
        }

    }  // for index in 0..n

    let ma_liste_bis: &mut [i32] = ma_liste_bis_vec.as_mut_slice();
    ma_liste.clone_from_slice(&ma_liste_bis);
    return;

} // fn tri_fusion




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


// Algorithme du tri par tas
// Entrée = Sortie:
// ma_liste: liste d'entiers 'ma_liste'
// Implémentation tri en place, non stable
// Complexité: n.log(n), en moyenne et dans le pire cas
// Utilise la structure de données 'std::collections::BinaryHeap' de Rust.
// Voir: https://fr.wikipedia.org/wiki/Tri_par_tas
pub fn tri_par_tas(ma_liste: &mut [i32])
{
    let n = ma_liste.len();

    // Liste avec 1 seul élément (-> rien à trier)
    if n <= 1 {return;}

    // Cas général avec au moins 2 éléments à trier
    use std::collections::BinaryHeap ;
    let mut binary_heap = BinaryHeap::new() ;
    for i in 0..n
    {
        // On insère tous les éléments à trier dans le tas
        binary_heap.push(ma_liste[i]) ;
    }

    for i in (0..n).rev()
    {
        // On retire tous les éléments du tas, qui sortent par ordre décroissant
        // (binary_heap = tas = File de priorité => Les premiers éléments retournés sont de valeur maximale)
        let v_opt = binary_heap.pop();
        if v_opt == None
        {
            println!("tri_par_tas : Erreur interne: Il devrait rester des éléments dans le tas.");
            panic!();
        }
        let v: i32 = v_opt.unwrap().clone();
        ma_liste[i] = v;

        // Invariant de boucle: A la fin de chaque itération:
        // - les (n - i) derniers éléments de ma_liste sont triés et les plus grands du tableau initial
        //   (i.e. ils sont à leur place finale)
        // - Le tas contient les éléments restants, et la racine pointe vers le plus grand élément du tas
    }

} // fn tri_par_tas

