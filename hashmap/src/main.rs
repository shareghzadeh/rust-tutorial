use std::collections::HashMap;
fn main() {
    // we create empty hash map then we insert the key and value
    let mut hash_map = HashMap::new();
    // has map is like dictionary in python it has "key and value"
    // value in hash map has to be same type
    hash_map.insert(String::from("random"), 20);
    hash_map.insert(String::from("Age"), 19);
    println!("{:?}", hash_map);

    // we loop in hash_map to get key and value
    for (key, value) in &hash_map {
        println!("{}: {}", key, value)
    }

    // for get the specified element in hash_map
    // we use get() to get the value of a key
    match hash_map.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match")
    }

    // we can remove element in hash map with remove()
    hash_map.remove(&String::from("Age"));
    println!("{:?}", hash_map);
}
