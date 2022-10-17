use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let nombre_secret = rand::thread_rng().gen_range(1..101);
    println!("Devinez le nombre mystère !");
    let mut count :u32 = 0;

    // for _i in 0..5 {
    loop {
        count += 1;
        println!("Veuillez entrer un nombre entre 1 et 100 compris");
        let mut supposition = String::new();
        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec lors de l'entrée de l'utilisateur");
        // let supposition: u8 = supposition
        //     .trim()
        //     .parse()
        //     .expect("Veuillez entrer un nombre entre 1 et 100 compris");
        let supposition = match supposition.trim().parse::<u8>() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("*** PLUS ***"),
            Ordering::Greater => println!("*** MOINS ***"),
            Ordering::Equal => {
                println!("Félicitation vous avez trouvé le nombre mystère : {}, Votre Score : {}", nombre_secret, count);
                break;
            }
        }
    }
    // println!("Le nombre secret est : {nombre_secret}");
}
