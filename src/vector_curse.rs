//add debug to print the struct
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn get_curse() {
    println!("Hello, Vector curse!");
    let v : Vec::<i32> = Vec::new();
    //Create vector with vec! macro
    let mut v = vec![1, 2, 3];
    v.push(5);
    //Read element on vector
    let third = &v[2];
    println!("The third element is {}", third);
    //with get method we can get None if index is out of bounds
    let third = v.get(21);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //Iterate over vector
    iterating_vector();
    iterating_vector_mut();
    storing_different_types_in_vector();
}
fn iterating_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
fn iterating_vector_mut() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}
fn storing_different_types_in_vector() {
    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),

    ];
    for i in row {
        println!("{:?}", i);
    }
}