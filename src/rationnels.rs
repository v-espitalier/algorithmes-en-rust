
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

use crate::classiques as classiques;
use std::ops::{Add, Sub, AddAssign, Mul, Deref}; //, Rem, DivAssign};
//use std::convert::AsMut;// From;
use std::fmt::{Display, Formatter, Result};
// Traits à implémenter: Add, Sub, Mul, Div, Eq, Ord, fmt/affiche

// En pratique, T = i32, i64, isize..
pub struct Rationnels<T>
//where T : Add<Output = T> + Mul<Output = T> + Clone + Copy,
{
    pub numerateur: T,
    pub denominateur: T,
}


// Trait Add:   c = a + b
// TODO: Utiliser plutôt des références en interne, pour ne pas nécessiter le trait 'Copy'
impl<T : Ord> Add for Rationnels<T> 
where T : Add<Output = T> + Mul<Output = T>  + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            numerateur : self.numerateur * other.denominateur + self.denominateur * other.numerateur,
            denominateur : self.denominateur * other.denominateur,
        }
    }
}

// Trait Add (sur refs) :   c = &a + &b  (emprunt: a et b toujours disponibles)
// TODO: Utiliser plutôt des références en interne, pour ne pas nécessiter le trait 'Copy'
impl<T : Ord> Add for &Rationnels<T>
where T : Add<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Rationnels<T>;

    fn add(self, other: &Rationnels<T>) -> Rationnels<T> {
        Rationnels {
            numerateur : self.numerateur * other.denominateur + self.denominateur * other.numerateur,
            denominateur : self.denominateur * other.denominateur,
        }
    }
}

/*
// Trait AddAssign: Combine addition et affectation: a += &b
impl AddAssign for &Rationnels<u64>
{
    fn add_assign(&mut self, rhs: &Rationnels<u64>) {
        let output_num = self.numerateur * rhs.denominateur + self.denominateur * rhs.numerateur;
        let output_den = self.denominateur * rhs.denominateur;
        self.numerateur = output_num; 
        self.denominateur = output_den;
    }
}
*/

// Trait Sub:   c = a - b
impl<T : Ord> Sub for Rationnels<T> 
where T : Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            numerateur : self.numerateur * other.denominateur - self.denominateur * other.numerateur,
            denominateur : self.denominateur * other.denominateur,
        }
    }
}

// Trait Sub (sur refs) :   c = &a - &b  (emprunt: a et b toujours disponibles)
impl<T : Ord> Sub for &Rationnels<T>
where T : Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Rationnels<T>;

    fn sub(self, other: &Rationnels<T>) -> Rationnels<T> {
        Rationnels {
            numerateur : self.numerateur * other.denominateur - self.denominateur * other.numerateur,
            denominateur : self.denominateur * other.denominateur,
        }
    }
}


impl<T> Display for Rationnels<T>
where T : Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.numerateur, self.denominateur)
    }
}

impl Rationnels<u64> 
{
    pub fn rendre_irreductible(&mut self) {
        let pgcd: u64 = classiques::pgcd(self.numerateur, self.denominateur);
        self.numerateur /= pgcd;
        self.denominateur /= pgcd;
    }
}

impl Rationnels<i64> 
{
    pub fn rendre_irreductible(&mut self) {
        let num: u64 = i64::abs(self.numerateur) as u64;
        let den: u64 = i64::abs(self.denominateur) as u64;
        let pgcd: i64 = classiques::pgcd(num, den) as i64;
        self.numerateur /= pgcd;
        self.denominateur /= pgcd;
    }
}



//trait Trait_NumDen: Ord + Eq + Clone {}
//impl<T> Trait_NumDen for T where T: Ord + Eq + Clone + Rem<Output = T> + From<u32> {}

/*
pub fn pgcd_generique<T>(a: isize, b: T) -> T
where T : Ord + Eq + Clone + Rem<Output = T> + From<u32>
{
    // On permute a et b si a < b
    if a < b {return pgcd_generique(b, a);}

    let m: T = a % b;
    // Gestion du cas particulier (fin des appels récursifs)
    let zero: T = T::from(0u32);
    if m == zero {return b;}

    // Appel récursif
    return pgcd_generique(b, m);

    // Le nombre de récursion est fini car a et b sont des entiers positifs 
    // et diminuent strictement à chaque appel
}
*/

