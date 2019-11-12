pub fn pattern_matching(input: String) {
    match input.as_str() {
        "coucou" => println!("Bonjour les HumanTalks"),
        "ca va" => {
            println!("Oui");
            println!("et");
            println!("toi");
        }
        "ByeBye" => println!("Il est temps de partir"),
        _ => println!("Je suis le cas par défaut"),
    }
}