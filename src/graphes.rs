
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Ne pas faire de warning si des fonctions ne sont pas appelées
#![warn(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

//use std::intrinsics::discriminant_value;
use std::ops::Add;
use std::fmt::Debug;

use crate::fichiers as fichiers;

// Définition d'un trait pour les sommets:
// A partir d'un sommet, on doit être capable de lister ses voisins + avoir les distances respectives
// Généricité: S = type des Sommets, et A = type des distances entre sommets
pub trait Voisins<S, A>
where A : PartialOrd + Add, S : PartialEq
{
    fn liste_voisins_et_distances(&self, sommet: &S) -> Vec<(S, A)>;
}


fn trouve_clef_min_val_dans_hashmap<S, A>(c : &HashMap<S, A>) -> (S, A)
where S : Eq + Hash + Clone, A : PartialOrd + Add + TryFrom<i8> + Clone + Debug
{
    let mut s_min_opt: Option<&S> = None;
    let mut a_min_opt: Option<&A> = None;
    for (s, a) in c.iter()
    {
        if (a_min_opt.is_some())
        {
            if ( a < &a_min_opt.unwrap())
            {
                s_min_opt = Some(s);
                a_min_opt = Some(a);
            }
        }
        else {
            s_min_opt = Some(s);
            a_min_opt = Some(a);
        
        }
    }

    return (s_min_opt.unwrap().clone(), a_min_opt.unwrap().clone());
}

// Implémentation de l'algorithme de Dijkstra
// https://fr.wikipedia.org/wiki/Algorithme_de_Dijkstra
// Entrées: Sommets de départ, et terminaux
// L'algorithme s'arrête dés que l'une des 2 conditions est remplie:
// - Soit on a trouvé un chemin d'un sommet de départ au sommet d'arrivée
// - Soit la liste courante est vide (on a fini de parcourir la partie connexe du graphe, contenant les sommets initiaux)
pub fn resoud_dijstra<G, S, A>(graphe : &G, s_init : Vec<S>, s_final : Vec<S>) -> (HashMap<S, A>, HashMap<S, S>, Option<S>)
where G : Voisins<S, A>,
S : Eq + Hash + Clone,
A : PartialOrd + Add + TryFrom<i8> + Clone + Debug + Add<Output = A>,
<A as TryFrom<i8>>::Error: Debug,
{
    // Liste passée (HashMap) des sommets déjà parcourus avec leur distance aux sommets initiaux (Vide au départ)
    let mut p: HashMap<S, A> = HashMap::new();

    // Liste courante (HashMap) des sommets avec leur distance aux sommets initiaux
    let mut c: HashMap<S, A> = HashMap::new();

    // Pour un sommet, donne le sommet précédent (en direction des sommets initiaux)
    let mut prec: HashMap<S, S> = HashMap::new();

    // Pour chaque sommet s de départ, d[s] = 0
    // et le rajouter à la file
    let dist_zero: A = A::try_from(0i8).expect("Distance nulle manquante pour le type A.");
    for sommet in s_init.iter()
    {
        let _ = &c.insert(sommet.clone(), dist_zero.clone());
    }

    let mut sommet_final_opt: Option<S> = None;

    // Tant que la file n'est pas vide
    while (&c.len() > &0)
    {
        // Prendre le plus petit élément a de la file de priorité
        let (sommet, dist) = trouve_clef_min_val_dans_hashmap(&c);

        // Retirer 'sommet' de la liste courante
        c.remove(&sommet);

        // L'ajouter 'sommet' de la file
        p.insert(sommet.clone(), dist.clone());

        if (s_final.contains(&sommet))
        {
            // On a trouvé un chemin optimal d'un sommet initial au final
            sommet_final_opt = Some(sommet);
            break;
        }


        //Itérer sur les 'voisins' de 'sommet':
        let voisins: Vec<(S, A)> = graphe.liste_voisins_et_distances(&sommet);
        for (voisin, voisin_dist) in voisins.iter()
        {
            // Si le voisin a déjà été parcouru, on continue
            if p.contains_key(voisin) {continue;}

            let dist_nouveau_possible = dist.clone() + voisin_dist.clone();
            let dist_cour_opt = c.get(voisin);

            // Cas ou le voisin est déjà dans la liste courante, avec une autre distance
            if (dist_cour_opt.is_some())
            {
                let dist_cour = dist_cour_opt.unwrap();
                if (&dist_nouveau_possible < dist_cour)
                {
                    // Si on a amélioré la distance, on met à jour les structures
                    //*C.get_mut(voisin).unwrap() = dist_nouveau_possible;
                    //*prec.get_mut(voisin).unwrap() = sommet.clone();
                    c.insert(voisin.clone(), dist_nouveau_possible);
                    prec.insert(voisin.clone(), sommet.clone());
                }
            }
            else {
                // Si le voisin n'est pas dans la liste courante, on le rajoute
                c.insert(voisin.clone(), dist_nouveau_possible);
                prec.insert(voisin.clone(), sommet.clone());
            }

       }

    }

    return (p, prec, sommet_final_opt);

}



/*
pub fn resoud_dijstra<S, A>(s_init : Vec<S>, s_final : Vec<S>) -> (HashMap<S, A>, HashMap<S, S>, Option<S>)
where S : Eq + Hash + Voisins<S, A> + Clone, A : PartialOrd + Add + TryFrom<i8> + Clone + Debug + Add<Output = A>,
<A as TryFrom<i8>>::Error: Debug,
*/

pub struct Labyrinthe {
    plan: Vec<String>,
    hauteur: u32,
    largeur: u32,
    s_init: Vec<u64>,
    s_final: Vec<u64>,

    caractere_init : char,
    caractere_final  : char,
}

impl Labyrinthe
{
    // Fonctions statiques
    pub fn u64_vers_hauteur_largeur(pos: u64) -> (u32, u32)
    {
        let puissance_separateur: u32 = 32;
        let deux_puiss:   u64 = u64::pow(2, puissance_separateur);
        let deux_puiss_moins_1: u64 = deux_puiss - 1;
        return ((pos / deux_puiss) as u32, (pos & deux_puiss_moins_1) as u32);
    }

    pub fn hauteur_largeur_vers_u64(hauteur: u32, largeur: u32) -> u64
    {
        let puissance_separateur: u32 = 32;
        let deux_puiss:   u64 = u64::pow(2, puissance_separateur);
        return (hauteur as u64) * deux_puiss + (largeur as u64);
    }

    fn trouve_caractere_dans_le_plan(plan: &Vec<String>, largeur: u32, hauteur: u32, caractere_a_trouver: char) -> Vec<u64>
    {
        let mut pos_vec: Vec<u64> = Vec::new();

        for index_hauteur in (0u32..hauteur)
        {
            let ligne_cour: Vec<char> = plan[index_hauteur as usize].chars().collect::<Vec<_>>();
            for index_largeur in (0u32..largeur)
            {
                let caractere_cour = ligne_cour[index_largeur as usize];
                if (caractere_a_trouver == caractere_cour)
                {
                    let pos: u64 = Self::hauteur_largeur_vers_u64(index_hauteur, index_largeur);
                    pos_vec.push(pos);
                }
            }
        }

        return pos_vec;
    }

    // Fonctions publiques
    // Constructeur
    pub fn new(plan : &Vec<String>) -> Self
    {
        let s_plan = plan.clone();

        let s_hauteur = s_plan.len() as u32;
        assert!(s_hauteur >= 1, "Erreur: Le labyrinthe doit avoir au moins une ligne");
        let s_largeur = s_plan[0].len() as u32;
        for i in (1..s_hauteur)
        {
            let largeur_cour: u32 = s_plan[i as usize].len() as u32;
            assert_eq!(largeur_cour, s_largeur, "Erreur: La largeur doit etre la meme pour toutes les lignes");
        }

        let s_caractere_init = '@';
        let s_caractere_final = '$';
        let s_s_init = Self::trouve_caractere_dans_le_plan(&s_plan, s_largeur, s_hauteur, s_caractere_init);
        let s_s_final = Self::trouve_caractere_dans_le_plan(&s_plan, s_largeur, s_hauteur, s_caractere_final);

        return Labyrinthe
                    {
                        plan    : s_plan,
                        hauteur : s_hauteur,
                        largeur : s_largeur,
                        s_init  : s_s_init,
                        s_final : s_s_final,
                        caractere_init  : s_caractere_init,
                        caractere_final : s_caractere_final};
    }


    // Getters/Setters
    pub fn s_init(&self) -> Vec<u64>
    {
        return self.s_init.clone();
    }

    pub fn s_final(&self) -> Vec<u64>
    {
        return self.s_final.clone();
    }

    pub fn caractere_init(&self) -> char
    {
        return self.caractere_init;
    }

    pub fn caractere_final(&self) -> char
    {
        return self.caractere_final;
    }

}


impl Voisins<u64, u64> for Labyrinthe
{
    fn liste_voisins_et_distances(&self, pos: &u64) -> Vec<(u64, u64)>
    {
        let mut voisins : Vec<(u64, u64)> = Vec::new();


        // Voisins possibles = Les 4 directions (haut, bas, gauche, droite) à une distance de 1
        let (hauteur, largeur) : (u32, u32) = Self::u64_vers_hauteur_largeur(*pos);
        let mut voisins_possibles : Vec<(u64, u64)> = Vec::new();

        if (hauteur > 0)
        {
            voisins_possibles.push((Self::hauteur_largeur_vers_u64(hauteur - 1, largeur), 1));
        }
        voisins_possibles.push((Self::hauteur_largeur_vers_u64(hauteur + 1, largeur), 1));
        if (largeur > 0)
        {
            voisins_possibles.push((Self::hauteur_largeur_vers_u64(hauteur, largeur - 1), 1));
        }
        voisins_possibles.push((Self::hauteur_largeur_vers_u64(hauteur, largeur + 1), 1));


        // On peut traverser une case du labyrinthe qui contient:
        // soit un espace ' ', soit le caracteres initial ou final (car n'étant pas des murs)
        let mut caracteres_passage: Vec<char> = Vec::new();
        caracteres_passage.push(' ');
        caracteres_passage.push(self.caractere_init);
        caracteres_passage.push(self.caractere_final);

        for (voisin, dist) in voisins_possibles
        {
            let (hauteur_v, largeur_v) : (u32, u32) = Self::u64_vers_hauteur_largeur(voisin);
            if (hauteur_v >= self.hauteur) {continue;}
            if (largeur_v >= self.largeur) {continue;}

            let case_cour: char = self.plan[hauteur_v as usize].chars().collect::<Vec<_>>()[largeur_v as usize];
            if (!caracteres_passage.contains(&case_cour)) {continue;}
            voisins.push((voisin, dist));
        }

        // Pour debuggage
        if (false)
        {
            println!("\nVoisins de la position: ({}, {})", largeur, hauteur);
            for (pos, _dist) in &voisins
            {
                let (hauteur, largeur) = Labyrinthe::u64_vers_hauteur_largeur(*pos);
                println!("(x,y) = ({},{})", largeur, hauteur);
            }
        }



        return voisins;
    }
}


pub fn resoud_labyrinthe(f_plan_labyrinthe: String, f_plan_solution: String)
{
    let plan_labyrinthe: Vec<String> = fichiers::lire_fichier_texte_lignes(&f_plan_labyrinthe, None);
    let labyrinthe: Labyrinthe = Labyrinthe::new(&plan_labyrinthe);

    let s_init : Vec<u64> = labyrinthe.s_init();
    let s_final : Vec<u64> = labyrinthe.s_final();

    println!("\nPosition(s) initiale(s):");
    for pos in &s_init
    {
        let (hauteur, largeur) = Labyrinthe::u64_vers_hauteur_largeur(*pos);
        println!("(x,y) = ({},{})", largeur, hauteur);
    }

    println!("\nPosition(s) finale(s):");
    for pos in &s_final
    {
        let (hauteur, largeur) = Labyrinthe::u64_vers_hauteur_largeur(*pos);
        println!("(x,y) = ({},{})", largeur, hauteur);
    }



    //let (HashMap<S, A>, HashMap<S, S>, Option<S>)

    let (p, prec, sommet_final_opt) = resoud_dijstra(&labyrinthe, s_init, s_final);
    //println!("P: {:?}", P); 
    //println!("\nSommet_final_opt: {:?}", sommet_final_opt); 

    if (false)
    {
        println!("\nDistance de chaque sommet au(x) point(s) initial(aux):");
        for (pos, dist) in &p
        {
            let (hauteur, largeur) = Labyrinthe::u64_vers_hauteur_largeur(*pos);
            println!("(x,y) = ({},{}) a une distance {}", largeur, hauteur, dist);
        }
    }

    if (sommet_final_opt.is_some())
    {
        let sommet_final = sommet_final_opt.unwrap();
        let dist_final = p[&sommet_final];
        let (hauteur, largeur) = Labyrinthe::u64_vers_hauteur_largeur(sommet_final);
        println!("Sommet final ({}, {}) a une distance de : {}", largeur, hauteur, dist_final);

        // Stocker tous les sommets parcourus
        let mut sommets_parcourus:Vec<(u32, u32)> = Vec::new();
        let s_init : Vec<u64> = labyrinthe.s_init();
        let s_final : Vec<u64> = labyrinthe.s_final();
        for (pos, _dist) in p
        {
            if (s_init.contains(&pos)) {continue;}
            if (s_final.contains(&pos)) {continue;}
            let (hauteur, largeur) = Labyrinthe:: u64_vers_hauteur_largeur(pos);
            sommets_parcourus.push((largeur, hauteur));
        }



        // Construire la solution
        let mut chemin_solution:Vec<(u32, u32)> = Vec::new();
        let mut sommet_cour = sommet_final;
        let s_init : Vec<u64> = labyrinthe.s_init();
        while (prec.contains_key(&sommet_cour))
        {
            //println!("Ajoute un point à la solution");
            sommet_cour = prec[&sommet_cour];
            if (s_init.contains(&sommet_cour)) {continue;}
            let (hauteur, largeur) = Labyrinthe:: u64_vers_hauteur_largeur(sommet_cour);
            chemin_solution.push((largeur, hauteur));
        }

        // Enregistrer la solution sur disque dur
        let mut plan_solution = plan_labyrinthe.clone();

        let caractere_sommet_parcouru = 'o';
        for (largeur, hauteur) in sommets_parcourus
        {
            let mut ligne_cour: Vec<char> = plan_solution[hauteur as usize].chars().collect::<Vec<_>>();
            ligne_cour[largeur as usize] = caractere_sommet_parcouru;
            let ligne_cour_string = ligne_cour.iter().collect::<String>();
            plan_solution[hauteur as usize] = ligne_cour_string;
        }

        let caractere_solution = 'x';
        for (largeur, hauteur) in chemin_solution
        {
            let mut ligne_cour: Vec<char> = plan_solution[hauteur as usize].chars().collect::<Vec<_>>();
            ligne_cour[largeur as usize] = caractere_solution;
            let ligne_cour_string = ligne_cour.iter().collect::<String>();
            plan_solution[hauteur as usize] = ligne_cour_string;
        }

        fichiers::ecrire_fichier_texte_lignes(&f_plan_solution, &plan_solution);
        println!("Fichier écrit: {}", f_plan_solution);


        let caractere_init = labyrinthe.caractere_init();
        let caractere_final = labyrinthe.caractere_final();
        let caractere_sommet_parcouru_couleur = "\x1b[90mo\x1b[0m";
        let chemin_solution_en_couleur = "\x1b[93mx\x1b[0m";
        let caractere_init_en_couleur = "\x1b[94m@\x1b[0m";
        let caractere_final_en_couleur = "\x1b[92m$\x1b[0m";
        let mut plan_solution_couleur: Vec<String> = Vec::new();
        for ligne in plan_solution
        {
            let ligne_couleur = ligne.replace(caractere_sommet_parcouru, &caractere_sommet_parcouru_couleur);
            let ligne_couleur = ligne_couleur.replace(caractere_solution, &chemin_solution_en_couleur);
            let ligne_couleur = ligne_couleur.replace(caractere_init, &caractere_init_en_couleur);
            let ligne_couleur = ligne_couleur.replace(caractere_final, &caractere_final_en_couleur);
            plan_solution_couleur.push(ligne_couleur);
        }
        // Afficher la solution à l'écran en couleur
        println!("Solution (via Dijkstra)");
        println!("{}", plan_solution_couleur.join("\n"));

    }
}
