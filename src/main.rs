use std::io;

struct Timecode {
    h : i32,
    m : i32,
    s : i32,
    f : i32,
}

fn main() {
    /*
    let retour = text_return();
    let modif = text_modif(retour.clone());
    let mut numero = String::from(&modif);

    println!("{}", retour);
    println!("{}", modif);

    println!("saisie i32: ");

    while numero.len() < 13 {
        let integer_input: i32 = read_user_input();
        numero = nb_push(numero, integer_input);
        println!("{}", numero);
    }

    let baba = slice_at_underscore(&numero);
    println!("{}", baba);
    */

    let ref_tc = Timecode {
        h: 2,
        m: 37,
        s: 16,
        f: 8,
    };

    println!("enter timecode value (hh:mm:ss:ff format)");
    let user_tc_tuple=user_input_breakdown();
    let user_tc = create_tc_value(user_tc_tuple);
    println!("{}:{}:{}:{}", user_tc.h, user_tc.m, user_tc.s, user_tc.f);


}

// hh:mm:ss:ff


fn slice_tc (source: &String) -> [i32; 3]{
    let octets = source.as_bytes();
    let mut point: [i32; 3] = [0,0,0];

    #[allow(unused_assignments)]
    let mut index = 0;
    for (i, &element) in octets.iter().enumerate() {
        
        if element == b':' {
           point[index]= i.try_into().unwrap();
           index += 1;
        }
    }
    
    return point;

}

fn user_input_breakdown() -> (i32, i32, i32, i32) {

    let user_tc = loop {      
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("error");

        let split = slice_tc(&user_input);

        if split[1] == split[2] {
            println!("invalid tc");
            continue;
        };

        let h: i32 = match user_input[0..split[0] as usize].parse() {
            Ok(int) => int,
            Err(_) => continue,
        }; 
        let m: i32 = match user_input[(split[0]+1) as usize..split[1] as usize].parse() {
            Ok(int) => int,
            Err(_) => continue,
        };
        let s: i32 = match user_input[(split[1]+1) as usize..split[2] as usize].parse() {
            Ok(int) => int,
            Err(_) => continue,
        };
        let f: i32 = match user_input.trim()[(split[2]+1) as usize..].parse() {
            Ok(int) => int,
            Err(_) => continue,
        };  

        if h <= 24 && m <= 60 && s <= 60 && f <= 24 {
            break (h, m, s, f);
        } else {
            println!("invalid tc");
        }

        
    };

    user_tc
}

fn create_tc_value (tc:(i32, i32, i32, i32)) -> Timecode {
    Timecode { h: tc.0, m: tc.1, s: tc.2, f: tc.3}
}

fn slice_at_underscore (source : &String) -> &str {
    let octets = source.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b'_' {
           return &source[i..source.len()];
        }
    
    }

    &source[..]
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

fn read_user_input() -> i32 {
    let mut user_input = String::new();

    loop {       
        io::stdin()
            .read_line(&mut user_input)
            .expect("error");

        let user_input : i32 = match user_input.trim().parse() {
            Ok(int) => int,
            Err(_) => continue,
        };
        break user_input; //return loop result ouside the loop 
    }
}
