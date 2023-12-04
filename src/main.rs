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
/*     let temp : u32 = "47".parse().expect("Not a number");
    println!("{temp}");

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

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



}

/* fn another_function(x: i32) -> i32 {
    return x+1;
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