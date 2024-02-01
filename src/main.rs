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
}

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
