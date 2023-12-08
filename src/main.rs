use std::io;

fn main() {

    let retour = text_return();
    let modif = text_modif(retour.clone());
    let mut user_input = String::new();

    println!("{}", retour);
    println!("{}", modif);

    println!("saisie i32: ");


    loop {
        
        
        io::stdin()
            .read_line(&mut user_input)
            .expect("error");

        let user_input : i32 = match user_input.trim().parse() {
            Ok(int) => int,
            Err(_) => continue,
        };
        
        break;
    }

    println!("{}", nb_push(modif, user_input));

}

fn text_return() -> String {

    let texte = String::from("texte");

    texte
}

fn text_modif(input: String) -> String {

    let mut modif = input;
    modif.push_str("_modif");

    modif

}

fn nb_push(mut text_input : String, nb_input : i32) -> String {

    let tmp : String = nb_input.to_string();
    text_input.push_str(&tmp);

    text_input
}
