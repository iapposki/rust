fn main() {
    // ############# mut vs let #########
    /* let x = 23;
    println!("{x}");

    let x = "some string";
    println!("{x}");

    let mut x = 23;
    println!("{x}");
    x = 34;
    println!("{x}");
    x = "34";
    println!("{x}"); */

    // ########### type conversion and destructuring ############
    /* let temp : u32 = "47".parse().expect("Not a number");
    println!("{temp}");

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y} and value of x is {_x}");

    let _arr1 = [3,4,5,5];
    let _arr2 = [2.3,4.0,3.3];
    let _arr3 = [3;4]; // same as [3,3,3,3]
    // array are immutable */

    // ############ funciton / if else / loop/ for ##############
    /* let x = another_function(15);
    println!("The value of x is {x}");
    if x > 10 {
        println!("x is greater than 10");
    } else {
        println!("x is smaller than 10");
    }
    let a = [1,2,3,4,5,6,7,8];
    for element in a {
        println!("{element}");
    }
    for number in (1..5).rev() {
        println!("{number}");
    } */

    // ########## EXERCISE : 1 #########
    /* let res1 = fahrenheit_to_celcius(-40.);
    let res2 = celcius_to_fahrenheit(-40.);
    println!("{res1}, {res2}");

    for i in 1..15 {
        let res = fibonacci(i);
        println!("{res}");
    } */

    // ################## ownership ##############
    /* {
        let m = "Something";
        println!("{m}");
    }
    // println!("{m}");

    let x = String::from("Some string.");
    let y = x.clone();
    println!("{x}, {y}"); // if clone (deep copy) method not used here, x is simply moved to y such that x is no longer valid thereafter. 

    let x = String::from("Some string");
    takes_ownership(x);     // function takes ownership (or x moves into the function) and hence x not valid after this line
    // println!("{x}");
    let y = 32;
    makes_copy(y);          // function makes a copy since the input type and stored on the stack and not in the heap. Making copy of something on stack is not taxing as compared to something on heap. Hence, y is still valid after this line
    println!("{y}"); */

    // ############### references and borrowing ##########
    /* let mut s = String::from("anotherstring");
    let len = calculate_length(&s); //s is just referenced here. Ownership remains the same. The function needs to be defined accordingly. ampersands represent references.
    println!("Length of the String '{s}' is : {len}");
    change(&mut s);
    println!("The changes string is '{s}'");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    let r3 = &mut s;
    println!("{r3}");  // cant have two simultaneous immutable and mutable references but its fine here since r1 and r2 scope ends after last use.

    //rules of references : 1. at any given time , you can have one mutable reference or any number of immutable references. 2. references must always be valid. */

    // ################## slice type ###########
    /* let s = String::from("Hello World");
    let word = first_word(&s);
    println!("{word}");
    // s.clear(); // error, already immutable reference present of s, by rules of borrowing, immutable and mutable reference cant coexist.
    println!("first word: {word}"); //compile error, 's' borrowed earlier with s.clear().

    let a = [1,2,3,4,5,6];
    let slice = &a[1..3];
    assert_eq!(slice, [2,3], "simple equality test."); */

}

/* fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
} */

/* fn calculate_length(s: &String) -> i32 {
    s.len() as i32
} */

/* fn change(s: &mut String){
    s.push_str(" dummystring");
} */

/* fn another_function(x: i32) -> i32 {
    return x+1;
} */

/* fn takes_ownership(x: String) {
    println!("{x} some string");
} */

/* fn makes_copy(x: i32) {
    println!("{x} some integer");
} */

/* fn fahrenheit_to_celcius(x:f64) -> f64 {
    return (x - 32.) * 5. / 9.;
}

fn celcius_to_fahrenheit(x:f64) -> f64 {
    return x * (9./5.) + 32.;
}

fn fibonacci(x:u32) -> u32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        return fibonacci(x-1) + fibonacci(x-2);
    }
} */
