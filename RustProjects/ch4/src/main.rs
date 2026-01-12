fn main() {
    //Ch4.2
    let mut s = String::from("hello");

    change(&mut s);
    //Ch4.3
    println!("{s}");
    let pntr_s = &s;
    let pntr_s1 = pntr_s[0];
    println!("{pntr_s}");
    println!("{pntr_s1}");
    
    
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
