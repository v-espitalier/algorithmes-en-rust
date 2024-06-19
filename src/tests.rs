use crate::classiques as classiques;
use crate::divers as divers;
use crate::rationnels as rationnels;

#[test]
fn test_classiques_factorielle() {
    assert_eq!(classiques::factorielle(4), 24);
    assert_eq!(classiques::factorielle(5), 120);
}

#[test]
fn test_classiques_pgcd() {
    assert_eq!(classiques::pgcd(15, 18), 3);
    assert_eq!(classiques::pgcd(90, 28), 2);
}

#[test]
fn test_classiques_fibonacci_iteratif() {
    assert_eq!(classiques::fibonacci_iteratif(8), 21);
    assert_eq!(classiques::fibonacci_iteratif(15), 610);
}

#[test]
fn test_classiques_fibonacci_recursif() {
    assert_eq!(classiques::fibonacci_recursif(8), 21);
    assert_eq!(classiques::fibonacci_recursif(15), 610);
}

#[test]
fn test_classiques_recherche_lineaire() {
    let mon_tableau: &[i32] = &[5, 10, 3, 7, 15];
    assert_eq!(classiques::recherche_lineaire(mon_tableau, 3), Some(2));
    assert_eq!(classiques::recherche_lineaire(mon_tableau, 7), Some(3));
    assert_eq!(classiques::recherche_lineaire(mon_tableau, 19), None);
}

#[test]
fn test_classiques_recherche_lineaire_generique() {
    let mon_tableau: &[i32] = &[5, 10, 3, 7, 15];
    assert_eq!(classiques::recherche_lineaire_generique(mon_tableau, 3), Some(2));
    assert_eq!(classiques::recherche_lineaire_generique(mon_tableau, 7), Some(3));
    assert_eq!(classiques::recherche_lineaire_generique(mon_tableau, 19), None);

    let mon_tableau: &[String] = &["aa".to_string(), "bb".to_string(), "cc".to_string(), "dd".to_string()];
    assert_eq!(classiques::recherche_lineaire_generique(mon_tableau, "cc".to_string()), Some(2));
    assert_eq!(classiques::recherche_lineaire_generique(mon_tableau, "aa".to_string()), Some(0));
    assert_eq!(classiques::recherche_lineaire_generique(mon_tableau, "ee".to_string()), None);
}

#[test]
fn test_classiques_recherche_dichotomique() {
    let mon_tableau: &[i32] = &[5, 10, 17, 24, 29, 37, 50];
    assert_eq!(classiques::recherche_dichotomique(mon_tableau, 17, None, None), Some(2));
    assert_eq!(classiques::recherche_dichotomique(mon_tableau, 50, None, None), Some(6));
    assert_eq!(classiques::recherche_dichotomique(mon_tableau, 13, None, None), None);
}

#[test]
fn test_pgcd_asm() {
    assert_eq!(divers::pgcd_asm(15, 18), 3, "Echec test_pgcd_asm (1)");
    assert_eq!(divers::pgcd_asm(90, 28), 2, "Echec test_pgcd_asm (2)");
}

#[test]
fn test_rationnels() {
    let r1 = rationnels::Rationnels::new(2i64, 3i64);
    let r2 = rationnels::Rationnels::new(5i64, 6i64);
    let r1_plus_r2 = rationnels::Rationnels::new(3i64, 2i64);
    let r1_moins_r2 = rationnels::Rationnels::new(-1i64, 6i64);
    let r1_mult_r2 = rationnels::Rationnels::new(5i64, 9i64);
    let r1_div_r2 = rationnels::Rationnels::new(4i64, 5i64);
    assert_eq!(&r1 + &r2, r1_plus_r2, "Echec test_rationnels (1) : Addition de références.");
    assert_eq!(&r1 - &r2, r1_moins_r2, "Echec test_rationnels (2) : Soustraction de références.");
    assert_eq!(&r1 * &r2, r1_mult_r2, "Echec test_rationnels (3) : Multiplication de références.");
    assert_eq!(&r1 / &r2, r1_div_r2, "Echec test_rationnels (4) : Division de références.");
}
