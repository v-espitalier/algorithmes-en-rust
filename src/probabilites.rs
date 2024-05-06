
// Module contenant des algorithmes relatives
// aux probabilités ou contenant une partie aléatoire


// Générateur de nombres pseudo aléatoires de type générateur Congruentiel Linéaire
// Implémentation de l'algorithme 'MINSTD' alias 'standard minimal'
// de Park et Miller (1988)
//
// Attention: Ce générateur est très prédictible
// <<<  NE PAS UTILISER CE GENERATEUR ALEATOIRE POUR LA CRYPTOGRAPHIE OU LES JEUX D'ARGENT >>>
//
// https://fr.wikipedia.org/wiki/G%C3%A9n%C3%A9rateur_congruentiel_lin%C3%A9aire
// https://en.wikipedia.org/wiki/Lehmer_random_number_generator
pub struct rng_minstd
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
// Voir: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
pub fn fisher_yates_shuffle(ma_liste: &mut [i32], seed: u32)
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





// Algorithme de Box Muller
// Permet de générer une paire de lois normales centrées réduites
// Utilise le PRNG MINSTD implémenté au dessus
// Attention: Ce générateur est très prédictible
// <<<  NE PAS UTILISER CE GENERATEUR ALEATOIRE POUR LA CRYPTOGRAPHIE OU LES JEUX D'ARGENT >>>
// Voir: https://fr.wikipedia.org/wiki/M%C3%A9thode_de_Box-Muller
pub fn box_muller_paire(rng : &mut rng_minstd) -> (f64, f64)
//pub fn box_muller_paire(rng : &mut rand::rngs::ThreadRng) -> (f64, f64)
{

    let u1_int: u32 = rng.gen();
    let u2_int: u32 = rng.gen();

    let u1: f64 = (u1_int as f64) / (rng.rng_m as f64);
    let u2: f64 = (u2_int as f64) / (rng.rng_m as f64);

    //pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64

    let rayon: f64 = f64::sqrt(-2. * u1.ln());
    let angle: f64 = 2. * std::f64::consts::PI * u2;
    let z0 = rayon * f64::cos(angle);
    let z1 = rayon * f64::sin(angle);

    return (z0, z1)
}

pub fn box_muller(nb_normales: usize, seed: u32) -> Vec<f64>
{
    // Utiliser l'implémentation locale du RNG MINSTD pour éviter la dépendance au crate 'rand'
    let mut rng: rng_minstd = rng_minstd::new(seed);

    let nb_paires_completes = nb_normales / 2;
    let nb_paires_incompletes = nb_normales - 2 * nb_paires_completes;

    let mut normales: Vec<f64> = Vec::new();
    for i in 0..nb_paires_completes
    {
        let (z0, z1) = box_muller_paire(&mut rng);
        normales.push(z0);
        normales.push(z1);
    }

    for i in 0..nb_paires_incompletes
    {
        let (z0, z1) = box_muller_paire(&mut rng);
        normales.push(z0);
    }

    return normales;
}

// Calculer la moyenne d'un tableau de valeurs
// Implémenté de façon générique, pour tout type de nombre
// qui est clonable, additionnable, divisable..
pub fn moyenne<T>(ma_liste : &[T]) -> Option<T>
where T : Clone + From<u32> + From<<T as std::ops::Div>::Output>  + std::ops::AddAssign + std::ops::Div
{
    let n = ma_liste.len();
    if (n == 0)
    {
        return None;
    }

    let mut somme: T = ma_liste[0].clone();

    for i in 1..n
    {
        somme += ma_liste[i].clone();
    }

    let n_as_u32: u32 = n as u32;
    let n_as_T = T::from(n_as_u32); //.unwrap()

    let moyenne: T = T::from(somme / n_as_T);

    return Some(moyenne);
}


// Calculer la variance non biaisée d'un vecteur
// Implémenté de façon générique, pour tout type de nombre
// qui est clonable, additionnable, multipliable, divisable..
pub fn variance_non_biaisee<T>(ma_liste : &[T]) -> Option<T>
where T : Clone + From<u32> + From<<T as std::ops::Mul>::Output> + From<<T as std::ops::Div>::Output>,
      T : std::ops::AddAssign + std::ops::Mul + std::ops::Div
{
    let n = ma_liste.len();
    if (n == 0)
    {
        return None;
    }

    let mut somme_carres: T = T::from(ma_liste[0].clone() * ma_liste[0].clone());

    for i in 1..n
    {
        somme_carres += T::from(ma_liste[i].clone() * ma_liste[i].clone());
    }

    let n_moins_1_as_u32: u32 = (n - 1) as u32;
    let n_moins_1_as_T = T::from(n_moins_1_as_u32); //.unwrap()

    let moyenne: T = T::from(somme_carres / n_moins_1_as_T);

    return Some(moyenne);
}

