use std::collections::HashMap;

pub fn get_hashmaps_curse() {
    println!("Hello, hashmaps_curse!");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    //Get value on Hashmap
    let score = scores.get(&team_name).copied().unwrap_or(0);
    get_value_on_hashmap(scores, team_name);
    boucle_for_hashmap();
}
    fn get_value_on_hashmap(hashmap: HashMap<String, i32>, key: String) {
        let score = hashmap.get(&key);
        match score {
            Some(score) => println!("The score is {}", score),
            None => println!("There is no score for this key"),
        }
    }

    fn boucle_for_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        for (key, value) in scores {
            println!("{}: {}", key, value);
        }
    }