pub fn borrow() {
    let thought = String::from("Je pense donc je suis");
    borrow_thought(&thought);
    move_thought(thought);
//    borrow_thought(&thought); // Error: Value moved before
}

fn borrow_thought(thought: &String) {
    println!("{}", thought);
}

fn move_thought(thought: String) {
    println!("{}", thought);
}