# RUST MEMO
Apprentissage et entrainement au langage Rust avec rustlings, en cours...

## Base
```
fonction: fn main() {
    (4 espaces à gauche)
}
```
fonction main est la première fonction executé dans un programme rust  
  
Pour compiler: rustc main.rs  
Pour executer: .\main  
Pour verifier le code sans créer d'executable: cargo check  

## Cargo
Créer projet cargo: cargo new hello_cargo  
Pour compiler: cargo build  
Pour executer: cargo run  
Pour compiler avec optimisation (ex: production): cargo build --release  

## Variables and Mutability
Dans rust les variables sont immutables par défaut.  
Pour les rendre mutables, il faut ajouter mut devant le nom de la variable.  
```
let mut x = 3;
println!("Number {}", x);
x = 5;
println!("Number {}", x);
```
Une variable doit toujours être initialisée !  
Il existe également const, qui est toujours immutable, son type doit toujours être annoté.

## Data Types
On peut typer les variables.
```
let x: i8 = -10;
let x: u8 = 10;
```
Ainsi x peut etre négatif quand il est typé en i8 mais en u8 non.

## Fonctions avec une valeur de return
On ne nomme pas les valeurs de retour, mais on doit déclarer leur type après une flèche (->).
```
fn is_even(num: i32) -> bool {
    num % 2 == 0
}
```
Prend un nombre en entrée et retourne un boolean en sortie.  
  
Info: Boucle for: for i in 0..num

## If
```
fn bigger(a: i32, b: i32) -> i32 {
    if a < b {
        b
    } else {
        a
    }
}
```
```
let condition = true;
let number = if condition { 5 } else { 6 };
```


### Autre Commandes
rustup update
rustc --version
cargo --version
Lancer Rustling: rustlings watch