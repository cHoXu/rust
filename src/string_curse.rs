pub fn get_string_curse() {
    println!("Hello, string_curse!");
    create_new_string_with_data();
    appending_to_string();
    concatenation_string();
    concatenation_string_with_format();
    iterating_string();
}

fn create_new_string()-> String {
    let mut s = String::new();
    s
}
fn create_new_string_with_data() {
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents");
}
fn appending_to_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    let mut s1 = String::from("lo");
    s1.push('l');
    println!("{}", s1);
}
fn concatenation_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");
}
fn concatenation_string_with_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let s = s1+"-"+&s2+"-"+&s3;
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
}
fn iterating_string() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}