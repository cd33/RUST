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
(rustc est le compilateur de rust)  
Pour executer: .\main  

## Cargo
Cargo est le package manager de rust, comme npm.
* Créer projet cargo: cargo new hello_cargo
* Ajouter cargo au dossier courant: cargo init
* Compiler: cargo build
* Executer: cargo run
* Compiler avec optimisation (ex: production): cargo build --release
* Compiler sans créer un binaire: cargo check

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

## Character Type
char, s'utilise avec des guillemets simples, par opposition aux littéraux string, qui utilisent des guillemets doubles.  
```
let your_character = '%'
your_character.is_alphabetic() // false
your_character.is_numeric() // false
```

## Tuple
Un tuple est un moyen général de regrouper un certain nombre de valeurs de types différents en un seul type composé. Les tuples ont une longueur fixe : une fois déclarés, leur taille ne peut ni augmenter ni diminuer.  
On crée un tuple en écrivant entre parenthèses une liste de valeurs séparées par des virgules. Chaque position du tuple a un type, et les types des différentes valeurs du tuple ne doivent pas nécessairement être les mêmes.
```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

## Array
Une autre façon d'avoir une collection de valeurs multiples est d'utiliser un array. Contrairement à un tuple, chaque élément d'un tableau doit avoir le même type. Contrairement aux tableaux dans certains autres langages, les tableaux dans Rust ont une longueur fixe.
```
let a: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

## Slice
Les slices permettent de référencer une séquence d'éléments dans une collection plutôt que la collection entière. Une slice est une sorte de référence, elle n'a donc pas de propriété.
```
let a = [1, 2, 3, 4, 5];
let nice_slice = &a[1..4];
assert_eq!([2, 3, 4], nice_slice)
```

## Ownership
Le ownership est un ensemble de règles qui régissent la manière dont un programme Rust gère la mémoire. Tous les programmes doivent gérer la façon dont ils utilisent la mémoire d'un ordinateur pendant leur exécution. Certains langages disposent d'un garbage qui recherche régulièrement la mémoire inutilisée pendant l'exécution du programme; dans d'autres langages, le programmeur doit explicitement allouer et libérer la mémoire.  
Rust utilise une troisième approche: la mémoire est gérée par un système de ownership avec un ensemble de règles que le compilateur vérifie. Si l'une de ces règles est violée, le programme ne compile pas. Aucune des caractéristiques du ownership ne ralentira votre programme en cours d'exécution.
### The Stack and the Heap (la pile et le tas)
La pile et le tas sont tous deux des parties de la mémoire que votre code peut utiliser au moment de l'exécution, mais ils sont structurés de manière différente. La pile stocke les valeurs dans l'ordre où elle les reçoit et les supprime dans l'ordre inverse. C'est ce qu'on appelle le principe du dernier entré, premier sorti. Pensez à une pile d'assiettes : lorsque vous ajoutez des assiettes, vous les mettez sur le dessus de la pile, et lorsque vous avez besoin d'une assiette, vous en prenez une sur le dessus. Ajouter ou retirer des assiettes du milieu ou du bas ne fonctionnerait pas aussi bien ! L'ajout de données est appelé "pousser" (pushing) sur la pile, et le retrait de données est appelé "sortir" (popping) de la pile. Toutes les données stockées sur la pile doivent avoir une taille connue et fixe. Les données dont la taille est inconnue au moment de la compilation ou dont la taille est susceptible de changer doivent être stockées sur le tas.  
  
Le tas est moins organisé : lorsque vous placez des données dans le tas, vous demandez une certaine quantité d'espace. L'allocateur de mémoire trouve un emplacement vide suffisamment grand dans le tas, le marque comme étant utilisé et renvoie un pointeur, qui est l'adresse de cet emplacement. Ce processus est appelé allocation sur le tas et est parfois abrégé en allocation (pousser des valeurs sur la pile n'est pas considéré comme une allocation). Comme le pointeur sur le tas a une taille connue et fixe, vous pouvez le stocker sur la pile, mais lorsque vous voulez les données réelles, vous devez suivre le pointeur. Imaginez que vous êtes assis dans un restaurant. Lorsque vous entrez, vous indiquez le nombre de personnes dans votre groupe, et le personnel trouve une table vide qui convient à tout le monde et vous y conduit. Si un membre de votre groupe arrive en retard, il peut demander où vous avez été assis pour vous trouver.  
  
Le chargement sur la pile est plus rapide que l'allocation sur le tas car l'allocateur n'a jamais à chercher un emplacement pour stocker de nouvelles données ; cet emplacement se trouve toujours au sommet de la pile. En comparaison, l'allocation d'espace sur le tas demande plus de travail, car l'allocateur doit d'abord trouver un espace suffisamment grand pour contenir les données, puis effectuer la comptabilité pour préparer l'allocation suivante.  
  
L'accès aux données du tas est plus lent que l'accès aux données de la pile car il faut suivre un pointeur pour y parvenir. Les processeurs contemporains sont plus rapides s'ils sautent moins dans la mémoire. Pour poursuivre l'analogie, considérons le serveur d'un restaurant qui prend les commandes de plusieurs tables. Il est plus efficace de prendre toutes les commandes d'une table avant de passer à la table suivante. Prendre une commande de la table A, puis une commande de la table B, puis une de la table A à nouveau, puis une de la table B à nouveau serait un processus beaucoup plus lent. De même, un processeur peut mieux faire son travail s'il travaille sur des données proches d'autres données (comme c'est le cas sur la pile) plutôt que plus éloignées (comme cela peut être le cas sur le tas).  
  
Lorsque votre code appelle une fonction, les valeurs transmises à la fonction (y compris, potentiellement, les pointeurs vers les données du tas) et les variables locales de la fonction sont poussées sur la pile. Lorsque la fonction est terminée, ces valeurs sont retirées de la pile.  
  
Garder la trace de quelles parties du code utilisent quelles données sur la pile, minimiser la quantité de données dupliquées sur la pile, et nettoyer les données inutilisées sur la pile pour ne pas manquer d'espace sont tous des problèmes que la propriété aborde. Une fois que vous aurez compris ce qu'est la propriété, vous n'aurez plus besoin de penser à la pile et au tas très souvent, mais le fait de savoir que l'objectif principal de la propriété est de gérer les données du tas peut vous aider à expliquer pourquoi elle fonctionne comme elle le fait.

### Ownership Rules
Les règles de propriété:
* Chaque valeur dans Rust a un propriétaire.
* Il ne peut y avoir qu'un seul propriétaire à la fois.
* Lorsque le propriétaire sort du champ d'application, la valeur est abandonnée.

### Variable Scope
```
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward
    // do stuff with s
}
```
Rust possède un deuxième type de chaîne, String. Ce type gère les données allouées sur le tas et, en tant que tel, est capable de stocker une quantité de texte qui nous est inconnue au moment de la compilation. Vous pouvez créer une chaîne à partir d'un littéral de chaîne en utilisant la fonction from, comme suit :
```
let s = String::from("hello");
```
L'opérateur double point :: nous permet d'attribuer un espace de nom à cette fonction from particulière sous le type String plutôt que d'utiliser une sorte de nom comme string_from.  
  
Ce type de chaîne peut être muté :
```
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`
```
### Memory and Allocation


## References and Borrowing
https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html  
Une référence est comme un pointeur: c'est une adresse que nous pouvons suivre pour accéder aux données stockées à cette adresse; ces données appartiennent à une autre variable. Contrairement à un pointeur, une référence est garantie de pointer vers une valeur valide d'un type particulier pendant toute la durée de vie de cette référence.  
```
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
Nous passons &s1 dans calculate_length et que, dans sa définition, nous prenons &String plutôt que String. Ces esperluettes représentent des références, et elles nous permettent de nous référer à une valeur sans en prendre possession.  
  
Nous appelons l'action de créer une référence "Borrowing". Comme dans la vie réelle, si une personne possède quelque chose, vous pouvez le lui emprunter. Lorsque vous avez terminé, vous devez le lui rendre. Vous ne le possédez pas.  
```
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Vectors
Les vecteurs sont l'une des structures de données Rust les plus utilisées. Dans d'autres langages de
programmation, on les appellerait simplement Arrays, mais comme Rust opère à un niveau
niveau inférieur, un tableau en Rust est stocké sur la pile (=stack, ce qui signifie qu'il ne peut pas
ne peut pas croître ou décroître, et la taille doit être connue au moment de la compilation),
et un vecteur est stocké dans le tas (=heap, où ces restrictions ne s'appliquent pas).  
  
Pour créer un vecteur vide:
```
let v: Vec<i32> = Vec::new();
```
Rust fournit la macro vec!, qui créera un nouveau vecteur contenant les valeurs que vous lui donnez.
```
let v = vec![1, 2, 3];
```
L'exemple crée un nouveau Vec<i32> qui contient les valeurs 1, 2 et 3. Le type d'entier est i32 car c'est le type d'entier par défaut.
### Updating a Vector
```
let mut v = Vec::new();
v.push(5);
v.push(6);
```

### Reading Elements of Vectors
Il y a deux façons de référencer une valeur stockée dans un vecteur : via l'indexation ou en utilisant la méthode get.
```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```
Lorsque nous utilisons la méthode get avec l'indice passé en argument, nous obtenons une Option<&T> que nous pouvons utiliser avec match.  
La raison pour laquelle Rust fournit ces deux façons de référencer un élément est que vous pouvez choisir comment le programme se comporte lorsque vous essayez d'utiliser une valeur d'index en dehors de la plage des éléments existants.  
À titre d'exemple, voyons ce qui se passe lorsque nous avons un vecteur de cinq éléments et que nous essayons d'accéder à un élément à l'indice 100 avec chaque technique, comme indiqué ci-dessous.
```
let v = vec![1, 2, 3, 4, 5];
let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```
Lorsque nous exécutons ce code, la première méthode [] provoquera une panique du programme car elle fait référence à un élément inexistant. Il est préférable d'utiliser cette méthode lorsque vous voulez que votre programme se plante s'il y a une tentative d'accès à un élément au-delà de la fin du vecteur.  
  
Lorsque la méthode get reçoit un index qui se trouve en dehors du vecteur, elle renvoie None sans paniquer. Vous utiliserez cette méthode si l'accès à un élément au-delà de l'étendue du vecteur peut se produire occasionnellement dans des circonstances normales.
  
Nous pouvons également itérer sur les références mutables de chaque élément d'un vecteur mutable afin d'apporter des modifications à tous les éléments.
```
for i in v.iter_mut() {
    *i = *i*2;
}
```
Pour modifier la valeur à laquelle la référence mutable fait référence, nous devons utiliser l'opérateur de déréférencement * pour atteindre la valeur dans i


### Autre Commandes
rustup update  
(rustup est le manager de version de rust)  
rustc --version  
cargo --version  
Lancer Rustling: rustlings watch  