
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

use std::ops::{Add, Sub, Mul, Div, Rem, Neg, AddAssign, SubAssign}; //, Deref, DivAssign};
use std::cmp::{PartialEq, PartialOrd, Ordering};
//use std::convert::AsMut;// From;
use std::fmt::{Display, Formatter, Result, Debug};

// Le derive trait 'Clone' évite d'avoir à implémenter 'à la main' le trait 'Clone'
#[derive(Clone)]
pub struct Rationnels<T>
//where T : Add<Output = T> + Mul<Output = T> + Clone + Copy,
// En pratique, T = i32, i64, isize..
{
    numerateur: T,
    denominateur: T,
}

impl<'a, T> Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
{
    pub fn new(numerateur: T, denominateur: T) -> Self { 
        let mut ret_num: T = numerateur;
        let mut ret_den: T = denominateur;

        // Gestion de la division par zéro
        let zero: T = T::try_from(0i8).expect("rationnels.rs zero(): Problème dans la conversion du zéro.");
        if (ret_den == zero)
        {
            panic!("Erreur: Division par zéro");
        }

        // Unicité de la représentation: Signes et irréductibilité

        // Le numérateur porte l'éventuel signe
        // Le dénominateur est toujours positif.
        if (ret_den < zero)
        {
            ret_den = -ret_den;
            ret_num = -ret_num;
        }

        // Transformation en fraction irréductible
        let mut abs_num: T = ret_num.clone();
        if (abs_num < zero)
        {
            abs_num = -abs_num;
        }

        if (ret_num != zero)
        {
            let pgcd: T = pgcd_generique(&abs_num, &ret_den);
            let pgcd_clone: T = pgcd.clone(); // Pour éviter de manipuler des références, un seul clone
            ret_num = ret_num / pgcd;
            ret_den = ret_den / pgcd_clone;
        }


        // Valeur de retour
        Self { 
             numerateur : ret_num,
             denominateur : ret_den,
         }
     } 

}


// Trait Add:   c = a + b
// TODO: Utiliser plutôt des références en interne, pour ne pas nécessiter le trait 'Copy'
impl<T> Add for Rationnels<T> 
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Add<Output = T> + Mul<Output = T>  + Copy,   // Pour l'addition
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let ret_num = self.numerateur * other.denominateur + self.denominateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}

// Trait Add (sur refs) :   c = &a + &b  (emprunt: a et b toujours disponibles)
// TODO: Utiliser plutôt des références en interne, pour ne pas nécessiter le trait 'Copy'
impl<T> Add for &Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Add<Output = T> + Mul<Output = T>  + Copy,   // Pour l'addition
{
    type Output = Rationnels<T>;

    fn add(self, other: &Rationnels<T>) -> Rationnels<T> {
        let ret_num = self.numerateur * other.denominateur + self.denominateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}


// Trait AddAssign: Combine addition et affectation: a += b
impl<T> AddAssign for Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Add<Output = T> + Mul<Output = T>  + Copy,   // Pour l'addition
{
    fn add_assign(&mut self, other: Rationnels<T>) {
        let ret_num = self.numerateur * other.denominateur + self.denominateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        let rationnel = Rationnels::<T>::new(ret_num, ret_den);
        self.numerateur = rationnel.numerateur;
        self.denominateur = rationnel.denominateur;
    }
}


// Trait Sub:   c = a - b
impl<T> Sub for Rationnels<T> 
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Sub<Output = T> + Mul<Output = T>  + Copy,   // Pour la soustraction
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let ret_num = self.numerateur * other.denominateur - self.denominateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}

// Trait Sub (sur refs) :   c = &a - &b  (emprunt: a et b toujours disponibles)
impl<T> Sub for &Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Sub<Output = T> + Mul<Output = T>  + Copy,   // Pour la soustraction
{
    type Output = Rationnels<T>;

    fn sub(self, other: &Rationnels<T>) -> Rationnels<T> {
        let ret_num = self.numerateur * other.denominateur - self.denominateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}


// Trait Subssign: Combine soustraction et affectation: a -= b
impl<T> SubAssign for Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Sub<Output = T> + Mul<Output = T>  + Copy,   // Pour la soustraction
{
    fn sub_assign(&mut self, other: Rationnels<T>) {
        let ret_num = self.numerateur * other.denominateur - self.denominateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        let rationnel = Rationnels::<T>::new(ret_num, ret_den);
        self.numerateur = rationnel.numerateur;
        self.denominateur = rationnel.denominateur;
    }
}


// Trait Mul:   c = a * b
impl<T> Mul for Rationnels<T> 
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Mul<Output = T> + Mul<Output = T>  + Copy,   // Pour la multiplication
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let ret_num = self.numerateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}

// Trait Mul (sur refs) :   c = &a * &b
impl<T> Mul for &Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Mul<Output = T> + Mul<Output = T>  + Copy,   // Pour la multiplication
{
    type Output = Rationnels<T>;

    fn mul(self, other: &Rationnels<T>) -> Rationnels<T> {
        let ret_num = self.numerateur * other.numerateur;
        let ret_den = self.denominateur * other.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}


// Trait Div:   c = a / b
impl<T> Div for Rationnels<T> 
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Mul<Output = T> + Mul<Output = T>  + Copy,   // Pour la division
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let ret_num = self.numerateur * other.denominateur;
        let ret_den = self.denominateur * other.numerateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}

// Trait Div (sur refs) :   c = &a / &b
impl<T> Div for &Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Mul<Output = T> + Mul<Output = T>  + Copy,   // Pour la division
{
    type Output = Rationnels<T>;

    fn div(self, other: &Rationnels<T>) -> Rationnels<T> {
        let ret_num = self.numerateur * other.denominateur;
        let ret_den = self.denominateur * other.numerateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}

// Trait Neg:   c = -a
impl<T> Neg for Rationnels<T> 
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Neg<Output = T> + Mul<Output = T>  + Copy,   // Pour la négation
{
    type Output = Self;

    fn neg(self) -> Self {
        let ret_num = -self.numerateur;
        let ret_den = self.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}

// Trait Neg:   c = -a   (sur références)
impl<T> Neg for &Rationnels<T> 
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Neg<Output = T> + Mul<Output = T>  + Copy,   // Pour la négation
{
    type Output = Rationnels<T>;

    fn neg(self) -> Rationnels<T> {
        let ret_num = -self.numerateur;
        let ret_den = self.denominateur;
        return Rationnels::<T>::new(ret_num, ret_den);
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
        return num_diff != zero;
    }

}

// Trait PartialOrd (sur refs):  Implémenter les 4 comparaisons : a > b,  a >= b,  a < b,  a <= b
impl<T> PartialOrd for Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : Sub<Output = T> + Mul<Output = T> + Copy, // Pour le trait PartialOrd
{
    fn partial_cmp(&self, other: &Rationnels<T>) -> Option<Ordering> {
        let sub_num: T = (self - other).numerateur;
        let return_ord: std::cmp::Ordering;
        let zero: T = T::try_from(0i8).expect("rationnels.rs zero(): Problème dans la conversion du zéro.");
        match sub_num {
            tmp if tmp > zero => {return_ord = Ordering::Greater;}
            tmp if tmp < zero => {return_ord = Ordering::Less;}
            _ => {return_ord = Ordering::Equal;}
        }
        return Some(return_ord);
    }

}

// Conversion vers f64:   Fraction -> Flottant (division flottante approchée)
// Marche pour i32, u32, i16, u16, i8, u8.
// Ne marche pas pour i64 et u64, car Rust considère que la conversion vers f64
// risque de se faire avec perte (Considérer le nombre de bit, et la forme des représentations)
impl<T> From<Rationnels<T>> for f64
where f64 : From<T>
{
    fn from(input: Rationnels<T>) -> f64 {
        let num_f64 = f64::from(input.numerateur);
        let den_f64 = f64::from(input.denominateur);
        if (den_f64 == 0.)
        {
            panic!("Erreur dans from: Division par zéro.");
        }

        return num_f64 / den_f64;
    }
}

// Conversion depuis i64:    Entier -> Fraction (num = entier, den = 1)
impl<T> From<i64> for Rationnels<T>
where T : PartialEq + PartialOrd + Clone + Neg<Output = T> + TryFrom<i8> + Div<Output = T> + Rem<T, Output = T>,
<T as TryFrom<i8>>::Error: Debug,
T : From<i64>,  // Pour la conversion du numerateur
{
    fn from(input: i64) -> Rationnels<T> {
        let ret_num: T = T::from(input);
        let un: T = T::try_from(1i8).expect("rationnels.rs: Problème dans la conversion du 'un.'");
        let ret_den = un;
        return Rationnels::<T>::new(ret_num, ret_den);
    }
}


// Traits pour l'affichage
impl<T> Display for Rationnels<T>
where T : Display + Clone
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.numerateur, self.denominateur)
    }
}

//Plus nécessaire car implémenté avec le derive trait directement
//(Le formattage est différent mais cela suffit pour du debug)
impl<T> Debug for Rationnels<T>
where T : Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}", self.numerateur, self.denominateur)
    }
}


// pgcd générique pour le type T
pub fn pgcd_generique<'a, T>(a: &'a T, b: &'a T) -> T
where T : PartialOrd + TryFrom<i8> + Clone,
<T as TryFrom<i8>>::Error: Debug,
T: Rem<T, Output = T>
{
    // On permute a et b si a < b
    if a < b {return pgcd_generique(b, a);}

    let zero: T = T::try_from(0i8).expect("rationnels.rs zero(): Problème dans la conversion du zéro.");
    if (b == &zero)
    {
        panic!("Erreur: Division par zéro dans le pgcd_generique.");
    }
    let m: T = a.clone() % b.clone();
    // Gestion du cas particulier (fin des appels récursifs)
    if m == zero {return b.clone();}

    // Appel récursif
    return pgcd_generique(b, &m);

    // Le nombre de récursion est fini car a et b sont des entiers positifs 
    // et diminuent strictement à chaque appel
}


/*

impl Rationnels<i64> 
{
    pub fn rendre_irreductible(&mut self) {
        let abs_num: u64 = self.numerateur.abs() as u64;
        let abs_den: u64 = self.denominateur.abs() as u64;
        let pgcd: u64 = pgcd_generique(&abs_num, &abs_den);
        self.numerateur /= (pgcd as i64);
        self.denominateur /= (pgcd as i64);
    }
}

*/