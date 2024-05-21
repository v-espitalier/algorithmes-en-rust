
// Ne pas faire de warning s'il y a des parenthèses en trop autour des conditions des if
#![allow(unused_parens)]

// Ne pas faire de warning si des fonctions ne sont pas appelées
#![warn(dead_code)]

use std::fmt::{Write};

pub fn conversions_entier()
{

  // Ecriture de valeurs décimales / hexadécimales / binaires / octales
  // dans le code Rust
  let v_dec: u64 = 37;
  let v_hex: u64 = 0x25;
  let v_bin: u64 = 0b100101;
  let v_oct: u64 = 0o45;
  assert_eq!(v_dec, v_hex, "Erreur dans la conversion (1)");
  assert_eq!(v_dec, v_bin, "Erreur dans la conversion (2)");
  assert_eq!(v_dec, v_oct, "Erreur dans la conversion (3)");


  // Conversion décimal -> string hexa, binaire, octale
  let s_hex_verif: String = "0x25".to_string();
  let s_bin_verif: String = "0b100101".to_string();
  let s_oct_verif: String = "0o45".to_string();
  let mut s: String = String::new();

  write!(s, "{:#x}", v_dec).expect("Erreur dans writeln (1)");
  assert_eq!(s, s_hex_verif, "Erreur dans la conversion (4)");

  s = "".to_string();
  write!(s, "{:#b}", v_dec).expect("Erreur dans writeln (2)");
  assert_eq!(s, s_bin_verif, "Erreur dans la conversion (5)");

  s = "".to_string();
  write!(s, "{:#o}", v_dec).expect("Erreur dans writeln (3)");
  assert_eq!(s, s_oct_verif, "Erreur dans la conversion (6)");


  // Conversion string hexa, binaire, octale -> décimal
  let v = u64::from_str_radix(&s_hex_verif[2..], 16).expect("Erreur dans u64::from_str_radix (1)");
  assert_eq!(v, v_dec, "Erreur dans la conversion (7)");

  let v = u64::from_str_radix(&s_bin_verif[2..], 2).expect("Erreur dans u64::from_str_radix (2)");
  assert_eq!(v, v_dec, "Erreur dans la conversion (8)");

  let v = u64::from_str_radix(&s_oct_verif[2..],8).expect("Erreur dans u64::from_str_radix (3)");
  assert_eq!(v, v_dec, "Erreur dans la conversion (9)");

}