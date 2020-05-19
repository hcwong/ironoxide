fn main() {
    // Type inference needed if we have no values cos cannot infer type :D
    let v: Vec<i32> = Vec::new();
    let v_inferred = vec![1, 2, 3];

    // Pushing
    // Note that you can only push to mut variable
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // As with all references,
    // we access vectors using two ways
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // Note, you cannot hold an immutable reference to third and then make v immutable,
    // because rust dosent allow mutable and immutable references in the same scope

    // Need to dereference if we want to make changes to mutable vector value
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // push_str takes a reference to avoid taking ownership of the bar string
    let mut s = String::from("foo");
    s.push_str("bar");

    // No string indexing due to UTF-8 size shennanigans
    // This also makes slicing a bit hard, consider using the .chars function

     use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Use collect to define hashhap
    // HashMap type needed for collect type inference, but not the K,V values, those can be
    // inferred
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // For owned vars, hashmap takes ownership
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
}
