
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

use crate::classiques as classiques;
use std::ops::{Add, Sub, Mul, Div, Rem}; //, AddAssign, Deref, DivAssign};
use std::cmp::{PartialEq, PartialOrd};
//use std::convert::AsMut;// From;
use std::fmt::{Display, Formatter, Result, Debug};

// En pratique, T = i32, i64, isize..
#[derive(Debug)]
pub struct Rationnels<T>
//where T : Add<Output = T> + Mul<Output = T> + Clone + Copy,
{
    pub numerateur: T,
    pub denominateur: T,
}



// Trait Add:   c = a + b
// TODO: Utiliser plutôt des références en interne, pour ne pas nécessiter le trait 'Copy'
impl<T> Add for Rationnels<T> 
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
impl<T> Add for &Rationnels<T>
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
impl<T> Sub for Rationnels<T> 
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
impl<T> Sub for &Rationnels<T>
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

// Trait Mul:   c = a * b
impl<T> Mul for Rationnels<T> 
where T : Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            numerateur : self.numerateur * other.numerateur,
            denominateur : self.denominateur * other.denominateur,
        }
    }
}

// Trait Mul (sur refs) :   c = &a * &b
impl<T> Mul for &Rationnels<T>
where T : Mul<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Rationnels<T>;

    fn mul(self, other: &Rationnels<T>) -> Rationnels<T> {
        Rationnels {
            numerateur : self.numerateur * other.numerateur,
            denominateur : self.denominateur * other.denominateur,
        }
    }
}


// Trait Div:   c = a / b
impl<T> Div for Rationnels<T> 
where T : Mul<Output = T> + Copy, // + Eq,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        //assert_ne!(other.numerateur, 0, "Division par zéro");
        Self {
            numerateur : self.numerateur * other.denominateur,
            denominateur : self.denominateur * other.numerateur,
        }
    }
}

// Trait Div (sur refs) :   c = &a / &b
impl<T> Div for &Rationnels<T>
where T : Mul<Output = T> + Copy,
{
    type Output = Rationnels<T>;

    fn div(self, other: &Rationnels<T>) -> Rationnels<T> {
        Rationnels {
            numerateur : self.numerateur * other.denominateur,
            denominateur : self.denominateur * other.numerateur,
        }
    }
}


// Trait PartialEq (sur refs):   test d'égalité (&a == &b)
impl<T> PartialEq for Rationnels<T>
where T : Sub<Output = T> + Mul<Output = T> + Copy + PartialEq + TryFrom<i8>,
<T as TryFrom<i8>>::Error: Debug
{
    fn eq(&self, other: &Rationnels<T>) -> bool {
        let num_diff = self.numerateur * other.denominateur - self.denominateur * other.numerateur;
        let zero: T = T::try_from(0i8).expect("rationnels.rs zero(): Problème dans la conversion du zéro.");
        return num_diff == zero;
    }

    fn ne(&self, other: &Rationnels<T>) -> bool {
        let num_diff = self.numerateur * other.denominateur - self.denominateur * other.numerateur;
        let zero: T = T::try_from(0i8).expect("rationnels.rs zero(): Problème dans la conversion du zéro.");
        return num_diff == zero;
    }

}




// Traits pour l'affichage

impl<T> Display for Rationnels<T>
where T : Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.numerateur, self.denominateur)
    }
}

/*
impl<T> Debug for Rationnels<T>
where T : Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.numerateur, self.denominateur)
    }
}
*/

//trait Trait_NumDen: Ord + Eq + Clone {}
//impl<T> Trait_NumDen for T where T: Ord + Eq + Clone + Rem<Output = T> + From<u32> {}

//where T : Ord + Eq + Clone + Rem<Output = T> + From<u32>
/*
pub fn pgcd_generique<'a, T>(a: &'a T, b: &'a T) -> T
where T : Rem<Output = T> + PartialOrd +  TryFrom<i8> + Mul<T, Output = T> + Clone,
<T as TryFrom<i8>>::Error: Debug,
&'a T: Mul<&'a T, Output = T>,
&'a T: Rem<&'a T, Output = T>
//fn test<T: for<'a> ParseFrom<&'a str>>(from: String) -> Result<T,P
{
    // On permute a et b si a < b
    if a < b {return pgcd_generique(b, a);}
    let moins_un = T::try_from(-1i8).expect("pgcd_generique: Probleme de conversion (1)");
    let zero = T::try_from(0i8).expect("pgcd_generique: Probleme de conversion (2)");
    let moins_a = &moins_un * a;
    let moins_b = &moins_un * b;
    if a < &zero {return pgcd_generique(&moins_a, b);}
    if b < &zero {return pgcd_generique(a, &moins_b);}

    let m: T = a % b;
    // Gestion du cas particulier (fin des appels récursifs)
    if m == zero {return b.clone();}

    // Appel récursif
    return pgcd_generique(b, &m);

    // Le nombre de récursion est fini car a et b sont des entiers positifs 
    // et diminuent strictement à chaque appel
}
*/


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

/*
impl<'a, T> Rationnels<T> 
where T : Rem<Output = T> + PartialOrd +  TryFrom<i8> + Mul<T, Output = T> + Div<Output = T> + Clone + 'a,
<T as TryFrom<i8>>::Error: Debug,
&'a T: Mul<&'a T, Output = T>,
&'a T: Rem<&'a T, Output = T>,
&'a T: Div<&'a T, Output = T>
{
    pub fn rendre_irreductible(&mut self) {
        let pgcd = pgcd_generique(&self.numerateur, &self.denominateur);
        self.numerateur   = &self.numerateur / &pgcd;
        self.denominateur = &self.denominateur / &pgcd;
    }
}

*/