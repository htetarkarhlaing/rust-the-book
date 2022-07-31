fn main() {
    let x = 35; // x is owner of the value "Hello"
    let y = x; // y is a copy of x
    println!("{}", y);

    let s1 = String::from("Hello"); // s1 is ther owner of the string on the heap "Hello"
    let s2 = s1; // s2 is not the shallow copy of the s1, it just move the access to it own
    println!("{}", s2);
    // println!("{}", s1) will be error because s1 is no longer exist

    let s3 = String::from("Hello Cloner");
    let s4 = s3.clone();

    // in this case, both s3 and s4 is valid because s4 is a clone of s3;
    // clone() is take new place to store the value, so its expensive.

    println!("s4 = {}", s4);
    println!("s3 = {}", s3);

    //------------------------------------ Work with functions -------------------------------------------//
    takes_ownership(s3);
    // println!("s3 = {}", s3); s3 is no longer exist right now.

    let new_string = give_ownership();
    println!("{}", new_string);

    let another_string = give_ownership();
    println!("{}", another_string);

    let to_take_back = String::from("Give Me Back");
    let my_string = take_ownership_and_give_it_back(to_take_back);
    println!("{}", my_string);

    //------------------------------------ refrencing -------------------------------------------//
    let my_new_string = String::from("Count my lenght");
    let (my_string_is, len) = caculate_length(my_new_string);
    println!("String is {}, lenght is {}", my_string_is, len);

    let my_new_string_to_ref = String::from("This is refrence");
    let lenght_refrenced_string = caculate_length_by_referncing(&my_new_string_to_ref);
    println!(
        "My string is {}, and its length is {}",
        my_new_string_to_ref, lenght_refrenced_string
    );

    let mut mutable_string = String::from("This value will be change");
    mutalbe_refrencer(&mut mutable_string);
    println!("{}", mutable_string);

    let mut s = String::from("hello world");
    // let hello = &s[..5];
    // let world = &s[6..];
    let world = first_word(&s);
    println!("{}", world);
    s.clear();
}

fn takes_ownership(parameter_string: String) {
    println!("take_ownership() => {}", parameter_string);
}

fn give_ownership() -> String {
    let some_string = String::from("Take this");
    some_string
}

fn take_ownership_and_give_it_back(a_string: String) -> String {
    a_string
}

//------------------------------------ for refrencing -------------------------------------------//

fn caculate_length(s: String) -> (String, usize) {
    let lenght = s.len();
    (s, lenght)
}

fn caculate_length_by_referncing(s: &String) -> usize {
    // s is just borrow the value of parameter not moved
    // refrences are immutables by default we can't change the value 😤
    let lenght = s.len();
    lenght
}

fn mutalbe_refrencer(s: &mut String) {
    s.push_str(" to blah blah");
}

fn first_word(s: &String) -> &str {
    let byte = s.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// notes
/*
    mutable refrences cannot mutate multiple time,
    once a time

    but refrences can use multiple time

    we can't have mutable refrence if immutable refrence already exists.
*/
