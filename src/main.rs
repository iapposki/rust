use core::panic;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::thread;
use std::time::{Duration, Instant};

use crate::garden::vegetables::Asparagus;
use rusttutorial::Tweet;
use rusttutorial::{front_of_house, Summary};

pub mod garden;

fn main() {
    let start_time = Instant::now();

    // ############# mut vs let #########
    {
        let x = 23;
        println!("{x}");

        let x = "some string";
        println!("{x}");

        let mut x = 23;
        println!("{x}");
        x = 34;
        println!("{x}");
        // x = "34"; // will give an error
    }

    // ########### type conversion and destructuring ############
    {
        let temp: u32 = "47".parse().expect("Not a number");
        println!("{temp}");

        let tup = (500, 6.4, 1);
        let (_x, y, _z) = tup;
        println!("The value of y is: {y} and value of x is {_x}");

        let _arr1 = [3, 4, 5, 5];
        let _arr2 = [2.3, 4.0, 3.3];
        let _arr3 = [3; 4]; // same as [3,3,3,3]
                            // array are immutable
    }

    // ############ funciton / if else / loop/ for ##############
    {
        let x = another_function(15);
        println!("The value of x is {x}");
        if x > 10 {
            println!("x is greater than 10");
        } else {
            println!("x is smaller than 10");
        }
        let a = [1, 2, 3, 4, 5, 6, 7, 8];
        for element in a {
            println!("{element}");
        }
        for number in (1..5).rev() {
            println!("{number}");
        }
    }

    // ########## EXERCISE : 1 #########
    {
        let res1 = fahrenheit_to_celcius(-40.);
        let res2 = celcius_to_fahrenheit(-40.);
        println!("{res1}, {res2}");

        for i in 1..15 {
            let res = fibonacci(i);
            println!("{res}");
        }
    }

    // ################## ownership ##############
    {
        {
            let m = "Something";
            println!("{m}");
        }
        // println!("{m}");

        let x = String::from("Some string.");
        let y = x.clone();
        println!("{x}, {y}"); // if clone (deep copy) method not used here, x is simply moved to y such that x is no longer valid thereafter.

        let x = String::from("Some string");
        takes_ownership(x); // function takes ownership (or x moves into the function) and hence x not valid after this line
                            // println!("{x}");
        let y = 32;
        makes_copy(y); // function makes a copy since the input type and stored on the stack and not in the heap. Making copy of something on stack is not taxing as compared to something on heap. Hence, y is still valid after this line
        println!("{y}");
    }

    // ############### references and borrowing ##########
    {
        let mut s = String::from("anotherstring");
        let len = calculate_length(&s); //s is just referenced here. Ownership remains the same. The function needs to be defined accordingly. ampersands represent references.
        println!("Length of the String '{s}' is : {len}");
        change(&mut s);
        println!("The changes string is '{s}'");

        let r1 = &s;
        let r2 = &s;
        println!("{r1} and {r2}");
        let r3 = &mut s;
        println!("{r3}"); // cant have two simultaneous immutable and mutable references but its fine here since r1 and r2 scope ends after last use.

        //rules of references : 1. at any given time , you can have one mutable reference or any number of immutable references. 2. references must always be valid.
    }

    // ################## slice type ###########
    {
        let s = String::from("Hello World");
        let word = first_word(&s);
        println!("{word}");
        // s.clear(); // error, already immutable reference present of s, by rules of borrowing, immutable and mutable reference cant coexist.
        println!("first word: {word}"); //compile error, 's' borrowed earlier with s.clear().

        let a = [1, 2, 3, 4, 5, 6];
        let slice = &a[1..3];
        assert_eq!(slice, [2, 3], "simple equality test.");

        // ############ defining and instantiating structs #########
        // declare struct before main function. will work here too.
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        let mut user1 = User {
            active: true,
            username: String::from("user1"),
            email: String::from("user1@email.com"),
            sign_in_count: 43,
        };
        user1.email = String::from("user1@gmail.com"); // user1 needs to be mutable entirely for assignment.
        println!(
            "{}, {}, {}, {}",
            user1.email, user1.active, user1.username, user1.sign_in_count
        );
        // update syntax
        let _user2 = User {
            email: String::from("user2@email.com"),
            ..user1
        };
        // println!("{}", user1.username);  // cant do this as .username was borrowed from user1 before so its on longer valid.
        struct Colour(i32, i32, i32);
        let black = Colour(0, 0, 0);
        println!("{}", black.0);
    }

    // ############# example program using structs ##########
    // program that calculates the area of a rectangle.
    #[derive(Debug)]
    struct Rectangle {
        width: f32,
        height: f32,
    }
    fn area(rectangle: &Rectangle) -> f32 {
        rectangle.width * rectangle.height
    }
    {
        let rect1 = Rectangle {
            width: 30. * 3.,
            height: 50.,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
        println!("The rectangle is {:?}", rect1); // need to implement #[derive(Debug)] implementation to the Rectangle struct first. :? prints in line whereas :#? pretty prints.
    }

    // ############ structs method syntax ##################
    impl Rectangle {
        fn area(&self) -> f32 {
            self.width * self.height
        }
        fn square(size: f32) -> Self {
            Self {
                width: size,
                height: size,
            }
        } //Self here is alias for what comes after the impl before which is Rectangle here.
    }
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            (self.height > other.height && self.width > other.width)
                || (self.height > other.width && self.width > other.height)
        }
    }
    {
        let rect1 = Rectangle {
            width: 30.,
            height: 40.,
        };
        println!("Area of rectangle is {}", rect1.area());
        let rect2 = Rectangle {
            width: 20.,
            height: 30.,
        };
        let rect3 = Rectangle {
            width: 35.,
            height: 20.,
        };
        println!("rect1 can hold rect2 : {}", rect1.can_hold(&rect2));
        println!("rect1 can hold rect3 : {}", rect1.can_hold(&rect3));
        println!("rect3 can hold rect2 : {}", rect3.can_hold(&rect2));
        let sq = Rectangle::square(3.);
        dbg!(sq);
    }

    // ############  enum #####################
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }
        let four = IpAddrKind::V4;
        let _six = IpAddrKind::V6;
        println!("{:?}", four);
        fn route(_ip_kind: IpAddrKind) {}
        route(IpAddrKind::V4);
        route(IpAddrKind::V6); // works for both this way.
                               // to use it in structs.
        #[derive(Debug)]
        struct IpAddr {
            _kind: IpAddrKind,
            _address: String,
        }
        let addr1 = IpAddr {
            _kind: IpAddrKind::V4,
            _address: String::from("127.0.0.1"),
        };
        println!("{:?}", addr1);
        #[derive(Debug)]
        enum IpAddrWithEnum {
            V4(String),
            V6(String),
        }
        let home = IpAddrWithEnum::V4(String::from("127.0.0.1"));
        let loopback = IpAddrWithEnum::V6(String::from("::1"));
        println!("{:?}, {:?}", home, loopback);
        // another example
        #[derive(Debug)]
        enum Message {
            _Quit,
            _Move { x: i32, y: i32 },
            Write(String),
            _ChangeColour(i32, i32, i32),
        }
        // impl can be used with enum as well
        impl Message {
            fn call(&self) {
                // method body would be defined here
                println!("{:?}", &self)
            }
        }
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // ############## some and none ################
    {
        // by design, null is not present in rust but can be implemented using enum as it is already by default as :
        // enum Option<T> { None, Some(T),}
        // None and Some are from std library std::option::Option. But they can be used directly as well.
        let _some_number = Some(5);
        let _some_char = Some('e');
        let _absent_number: Option<i32> = None;
        // note : absent_number is still of type Option<i32> and not just i32 so such operations can be performed. So the type of the variable needs to be change before performing such operation and thus eliminating the issue of assuming that something isn't null when it actually is. Everywhere that a value has a type that isn't an Option<T>, you can safely assume that the value isn't null.
    }

    // ########## match control flow construct ###############
    #[derive(Debug)]
    enum UsState {
        _Alabama,
        Alaska,
        // --snip--
    }
    #[derive(Debug)]
    enum Coin {
        _Penny,
        _Nicket,
        _Dime,
        Quarter(UsState),
    }
    {
        fn value_in_cents(coin: &Coin) -> u8 {
            match coin {
                Coin::_Penny => {
                    println!("Lucky Penny!");
                    1
                }
                Coin::_Nicket => 5,
                Coin::_Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
        let new_coin = Coin::Quarter(UsState::Alaska);
        value_in_cents(&new_coin);
        println!("{:?}", new_coin);
        //
        fn some_add_or_none(value: Option<i32>) -> Option<i32> {
            match value {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        // if a case was not covered in the match above, compiler will throw an error. Matches in rust are exhaustive.
        let five = Some(5);
        let six = some_add_or_none(five);
        let none = some_add_or_none(None);
        println!("{:?}, {:?}, {:?}", five, six, none);
        // to take care of all the other cases in match, a catch all phrase. Either use that or a placeholder _ to take care of all the cases.
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            other => move_player(other),
        }
        let dice_roll_alt = 3;
        match dice_roll_alt {
            3 => add_fancy_hat(),
            _ => (),
        }
        fn add_fancy_hat() {}
        fn move_player(_num_spaces: u8) {}
    }

    // ############ concise control flow with if let #########
    {
        let config_max = Some(2i32);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => {}
        };
        // instead, as a shortcut
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        };
        // else ; used in place of _ for other cases
        let mut count = 0;
        let another_new_coin = Coin::Quarter(UsState::Alaska);
        if let Coin::Quarter(state) = another_new_coin {
            println!("The quarter is from the state {:?}!", state);
        } else {
            count += 1;
            println!("{count}");
        }
    }

    // ################ managing growing projects with packages, crates and moduels ###################
    {
        // packages and crates : crate is smallest amount of code that rust compiler considers at a time. 2 types, binary and library crates.
        // binary crates are promgrammes that can be compiled to executable, it has a main function. library crate dont and cant ompilte to executable.
        // crate root is a source file that the compiler starts from. default is src/main.rs
        // package is bundle of one or more crates. it can contain as many binary crates as required (in src/bin directory) and only one library crate (src/lib.rs). both can be present in the package at the same time.
    }

    // #################### defining modules ##################
    {
        // to declare a module, either 1. inline curly brackets that replace the semi colon here, 2. in src/garden.rs or 3. in the file src/garden/mod.rs.
        // to declare Submodules, agian either inline in curly brackets `mod vegetables`, or in garden.rs or in mod.rs in the garden directory as `mod vegetables`
        // paths to code in modules : code in module can be reffered anywhere in crate. for example `crate::garden::vegetables;:Asparagus` for an Asparagus type in the garden vegetables module.
        // Private vs Public : codes within modules are private from parent module by default use pub before to make it public. use inside modules as well as needed.
        // keyword : instead of using `crate::garden::vegetables;:Asparagus` every time instead use ` use crate::garden::vegetables;:Asparagus` so you only need to write Asparagus to make use of that type in the scope.
        let plant = Asparagus {};
        println!("I'm growing {:?}", plant);
        front_of_house::hosting::add_to_waitlist();
    }

    // ################### bringing paths into scope with the `use` keyword ##################
    {
        // use only creates shortcut for the particular scope in which the use occurs. move the use to the required scope to make use of the shortcut
    }

    // ############### storing list of values in vectors##################
    {
        // specify which types is going to be fed to the vectore during initialization.
        let _v: Vec<i32> = Vec::new();
        // alternatively use macro which infers the type automatically
        let mut v = vec![1, 2, 3];
        // to update
        v.push(5);
        // to read elements
        let third = &v[2];
        println!("{third}");
        println!("{:?}", v);
        // get can also be used to get the value but it return option
        let third = v.get(2);
        match third {
            Some(number) => println!("the number is {number}"),
            None => println!("the number aat that position doesn't exist"),
        }
        // code will panic if the code below runs, here get is used.
        // let _temp = &v[100];
        let temp = v.get(100);
        println!("{:?}", temp); //None
                                // consider this case where we make an immutable borrow occurs
        let first = &v[0];
        println!("{first}");
        // but when we try to push an element to v, we will get an error stating 'v' cannot be borrowed as mutable because it is also borrowed as immutable and when printing 'first' it says immutable borrow used here.
        // v.push(6);
        // println!("The first element is {first} ");
        // but why is it that when an element is immutably borrowed from that vector, pushing an element becomes restricted? this happens because act of adding might need reallocating vector to new area in case of insufficient memory in the previous allocation.
        // iterating
        for i in &v {
            println!("{i}");
        }
        // mutable iterating
        println!("{:?}", v);
        for i in &mut v {
            *i += 50; // * is the dereference operator to get to the value in i before we can use the += operator. will be talked about more later
        }
        println!("{:?}", v);
        // using enum to store multiple types in vector
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("value")),
            SpreadsheetCell::Float(3.14),
        ];
        println!("{:?}", row);
    }

    // #################### STRING ##############################
    {
        let data = "initial contents";
        let mut s = data.to_string();
        // let s = "initial contents".to_string();
        // let s = String::from("initial contents");
        // strings are UTF-8 encoded
        println!("{s}");
        s.push_str(" added content");
        println!("{s}");
        let s1 = String::from("hello");
        let s2 = String::from(" world!");
        let s3 = s1 + &s2; // note s1 has moved here and can no longer be used. + uses add operator which looks like `fn add(self, s: &str) -> String {...}` and hence takes string literal as the second arguement or any arguement following it for that matter.
                           // alternatively use format! macro. it doesnt take ownership of any of the variables.
        let s = format!("{s3} to formatted {s2}");
        println!("{s}");
        // rust doesn't index string. so s[0] will throw an error. you need to be more specifid when you say s[0] as it could mean first byte, first char, etc.
        println!("{}", &s[0..1]); // this, although valid, might also cause a panic as it takes the first byte but in different languages one character is not always one byte.
                                  // use .chars() to iterate.
        for c in s.chars() {
            println!("{c}");
        }
    }

    // ################### hash map ##########################
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("key1"), 5);
        scores.insert(String::from("key2"), 10);
        // to access
        let key = String::from("key1");
        let temp = scores.get(&key); // takes reference as argument, alternatively use "key" literal here.
        println!("{:?}", temp);
        // by default temp will have Option<&i32> type
        // alternatively
        let temp = scores.get("key1").copied().unwrap_or(0);
        // copied gives Options<i32> instead of Options<&i32> and unwrap_or() will give teh value inside some if present or 0 if absent.
        println!("{temp}");
        // to loop through the keys
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
        // to add key value if present, use entry
        scores.entry(String::from("key3")).or_insert(25);
        scores.entry(String::from("key2")).or_insert(20);
        println!("-----------");
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
        let some_str = "hello world a beautiful world";
        let mut map = HashMap::new();
        for word in some_str.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
        // the hashmap in the standard library is not the fastest as it uses the SipHash function which provides safety from denial of service (DoS) attaacks involving hash tables
    }

    // ################### error handling #######################
    {
        // rust throws two kind of errors, first is resolvable error for which code doesn't needs to be stopped, for eg file not found, having type `Result<T, E>`, and the second is non recoverable error, for eg accessing an array element outside of its range, having macro panic!.
        // panic! can be explicitely called using macro or due to some bug in code.
        // panic!("crash and burn");
        // or
        // let v = vec![1,2,3];
        // v[99];
        // to check backtrace, if using powershell, run `$env:RUST_BACKTRACE=1` to set the variable then run the project.
    }

    //  ################## recoverable errors with result ###################
    {
        // Result is defined as :
        // enum Result<T, E> { Ok(T), Err(E)}
        // T type result returned in case of success and E type (Err) error returned in case of failure, for example :
        let greeting_file_result = File::open("./src/hello.txt");
        // println!("{:?}", greeting_file_result);
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("./src/hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("problem creating the file : {:?}", e),
                },
                other_error => {
                    panic!("problem opening the file: {:?}", other_error);
                }
            },
        };
        println!("{:?}", greeting_file);
        // alternatively, instead of using so many match, :
        let greeting_file_alt = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("./src/hello.txt").unwrap_or_else(|error| {
                    panic!("problem creating the file: {:?}", error);
                })
            } else {
                panic!("problem opening the file: {:?}", error);
            }
        });
        println!("{:?}", greeting_file_alt);
        // if individually handling error kinds is not required, simply use .unwrap(), if the result is Ok, unwrap returns the value in Ok and if it is an error, unwrap automatically calls the panic! macro. the general norm is using .expect(), it works similar to .unwrap() but also takes in a prompt which is displayed in case of an error.
        let _greeting_file_short =
            File::open("./src/hello.txt").expect("hello.txt file is required to run");
        // to propagate an error, i.e., giving back the error instead of raising it in the function block itself.
        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("./src/hello.txt");
            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(e) => return Err(e),
            };
            let mut username = String::new();
            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }
        println!("{:?}", read_username_from_file());
        // shortcut for propagating errors: the ? operator
        fn read_username_from_file_using_operator() -> Result<String, io::Error> {
            let mut username_file = File::open("./src/ello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username)?;
            Ok(username)
        }
        println!("{:?}", read_username_from_file_using_operator());
        // ? operator: it basically works the same as the match expression. if the value generated from result is Ok(_), the value is returned to the expression else when the value is an Err, the Err will be returned from the whole funcitonas if we had used the return keyword so the error value gets propagated to the calling code.
        // how ? is different from the match expression is that the ? operator calls the 'from' function, the error type received is converted to the type defined in the return type of the current function.
        // chaining method calls after the ? operator
        fn read_username_from_file_using_operator_chained() -> Result<String, io::Error> {
            let mut username = String::new();
            File::open("./src/hello.txt")?.read_to_string(&mut username)?;
            Ok(username)
        }
        println!("{:?}", read_username_from_file_using_operator_chained());
        // on the other hand to make this funciton even shorted :
        fn read_username_from_file_shortest() -> Result<String, io::Error> {
            fs::read_to_string("./src/hello.txt")
        }
        println!("{:?}", read_username_from_file_shortest());
        // note: custom errors can be made and used (automatically if ? operator used and the correct type is stated in the reutrn type) by `impl From<io::Error>` for some "CustomError". Also the operator can only be used inside a function which returns Result<> or Option  or another type that implements FromResidual.
    }

    // ################## panic ####################
    {
        #[derive(Debug)]
        struct Guess {
            value: i32,
        }
        impl Guess {
            fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}", value);
                }
                Guess { value }
            }
            fn value(&self) -> i32 {
                self.value
            }
        }
        let guess = Guess::new(19);
        println!("{:?}, {}", guess, guess.value());
    }

    // ############################ generic data types ####################
    {
        // fn largest<T>(list: &[T]) -> &T {
        //     let mut largest = &list[0];
        //     for item in list {
        //         if item > largest {
        //             largest = item;
        //         }
        //     }
        //     largest
        // }
        // let number_list = vec![23,43,45,545643];
        // let result = largest(&number_list);
        // println!("{result}");
        // this wont work yet as > operator is not defined for every type including generic T here.
        struct _Point1<T> {
            x: T,
        }
        struct Point2<T, U> {
            x: T,
            y: U,
        }
        enum _Result<T, E> {
            Ok(T),
            Err(E),
        }
        // in method definition :
        impl<T> _Point1<T> {
            fn _x(&self) -> &T {
                &self.x
            }
        }
        impl<T1, U1> Point2<T1, U1> {
            fn mixup<T2, U2>(&self, other: Point2<T2, U2>) -> Point2<&T1, U2> {
                Point2 {
                    x: &self.x,
                    y: other.y,
                }
            }
        }
        let p1 = Point2 { x: 5, y: 10.4 };
        let p2 = Point2 { x: "hello", y: "c" };
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
        println!("{}", p1.x);
    }

    // ################### traits: defining shared behavior ###################
    {
        // traits are a way to group methods signatures together to define a set of behaviors necessary to accomplish some purpose
        // eg: a trait is defined in lib.rs as
        // pub trait Summary {
        //     fn summarize(&self) -> String;
        // }
        // each type implementation must provide its own custom behavior for the body of the method summarize here.
        // eg:
        // pub struct NewsArticle {
        //     pub headline: String,
        //     pub location: String,
        //     pub author: String,
        //     pub content: String,
        // }

        // impl Summary for NewsArticle {
        //     fn summarize(&self) -> String {
        //         format!("{}, by {} ({})", self.headline, self.author, self.location)
        //     }
        // }

        // pub struct Tweet {
        //     pub username: String,
        //     pub content: String,
        //     pub reply: bool,
        //     pub retweet: bool,
        // }

        // impl Summary for Tweet {
        //     fn summarize(&self) -> String {
        //         format!("{}: {}", self.username, self.content)
        //     }
        // }
        let tweet = Tweet {
            username: String::from("david"),
            content: String::from("this is my first gaming tweet"),
            reply: false,
            retweet: false,
        };
        println!("{}", tweet.summarize());
        // note: default behavior can also be defined for a trait, just provide a method to summarize here in pub trait Summary here for example.
    }

    // ################## traits as parameters; trait bound syntax; specifying multipel trait bounds with + Syntax; clearer trait bounds with where clause ###########################
    {
        pub fn _notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        // this function implements Summary trait without explicitely defining the type of item provided the item being provided has a type which already implements Summary trait. Here, the item can be either Tweet or NewsArticle as defined before.
        // the impl trait syntax here is a syntax sugar for a longer form known as trait bound and it looks like:
        pub fn _notify_expanded<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }
        // to specify multiple trait bounds we use + Syntax:
        pub fn _notify_multi(_item: &(impl Summary + Display)) {}
        // and in trait bound form :
        pub fn _notify_multi_expanded<T: Summary + Display>(_item: &T) {}
        // using multiple trait bounds can get messy sometiems, eg:
        pub fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}
        // instead use where clause:
        pub fn _some_function_where<T, U>(_t: &T, _u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            return 0;
        }
    }

    // ################# returning types that implement traits ##############
    {
        fn _returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
        // note: returning either a Tweet or NewsArticle here as a possible return type wont work with impl Summary due to reestrictions around how the impl Trait syntax is implemented in the compiler.
        // now the largest function can be run with the following modification.
        fn largest<T: Display + PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        let number_list = vec![23, 43, 45, 545643];
        let result = largest(&number_list);
        println!("{result}");
    }

    // ######################### Validating references with lifetimes ############################
    {
        // consider function:
        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }
        // this gives an error saying the return type needs a generic lifetime parameter on it because rust cant tell wheather the reference being returned refers to x or y.
        // examples of lifetime parameter:
        // &i32   // a reference
        // &'a i32   // a reference with an explicit lifetime
        // &'a mut i32  // a mutable reference with an explicit lifetime
        // to use these in functions:
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        // The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
        // struct holding reference nedds a lifetime annotation on every reference.
        struct _ImportantExcerpt<'a> {
            part: &'a str,
        }
        // all in all, references have a lifetime and you need to specify lifetime param for functions or structs that use references. few functions used before didnt require it because rust automatically adds it in some very obvious cases.
        // putting everyting together
        fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        // the function  has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause. This extra parameter will be printed using {}, which is why the Display trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name
    }   

    // ################################## how to write tests ###################################
    {
        // consider this code in lib.rs:
        // pub fn add(left: usize, right: usize) -> usize {
        //     left + right
        // }

        // #[cfg(test)]
        // mod tests {
        //     use super::*;

        //     #[test]
        //     fn it_works() {
        //         let result = add(2, 2);
        //         assert_eq!(result, 4);
        //     }
        // }
        // the #[test] annotation indicates this is a test function. we can also have non-test function in the tests module to help setup common scenarios or perform common operations.
        // refer to lib.rs for examples.
    }

    // ############################### controlling how tests are run ############################
    {
        // { tests by default are run in parallel }. Tests are run simultaneously on separate threads. If some tests depend on each other or on any shared state like shared environments, current working directory or variabbles, the tests gives an error. To remedy this, you can make it so that the test use only one thread effectively running tests consecutively. 
        // e.g. command : 
        // $ cargo test -- --test-threads=1
        // wherer == is called the separator.
        // { when test passes, rust captures any println output }. It only shows up when a test doesn't pass and further information of the failure is shown in the terminal.
        // if we want to see the printed values for passing tests as well, we add ;
        // $ cargo test == ==show-output 
        //  { running a subseto of tests by name }: giving argument after $ cargo test <argument> ; will search for the substring argument in the test cas names and will selectively run them.
        // { ignoring tests unless specifically requested }: adding annotation #[ignore] just below the #[test] annotaiton will make it so that when tests are run, these tests are ignored.
        // to run all tests or tests which are ignored, we used these respectively:
        // $ cargo test -- --include-ignored
        // $ cargo test -- --ignored
    } 

    // ################################# test organization ##############################
    {
        // convention is to make tests modules in each file in src folder and annotate it with cfg(test).
        // #[cfg(test)] runs only on cargo test, not on cargo build.
        // this is true for unit tests, not for integration tests. It goes in different directory and doesn't need the #[cfg(test)] annotation.
        // integration test is done in test directory (project/tests/integration_tests.rs) which is at the same level of src directory.
        // each file in the tests folder is a separate crate. refer to any file in tests directory.
        // for running only integration tests : 
        // $ cargo test --test integration_test
        // for sharing some common code with other integration files and not make it come up in the console when integration tests are run, we need to make a folder (eg: common folder we setup up here) and make keep codes there (mod.rs here) for common code sharing. Thus making folder there tells rust to not treat that module as an integration test.
        // to make use of modules in test, we simply import it into the integration test(eg; using the common module here would make it `mod common;`)
    }

    // ################################## closures: anonymous functions that capture their environment #########################
    {
        // closures are anonymoous functions that can be saved in a variable or be passed as arguments to other functions
        #[derive(Debug, PartialEq, Copy, Clone)]
        enum ShirtColour {
            Red,
            Blue,
        }
        struct Inventory {
            shirts: Vec<ShirtColour>,
        }
        impl Inventory {
            fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
                user_preference.unwrap_or_else(|| self.most_stocked())
            }
            fn most_stocked(&self) -> ShirtColour {
                let mut num_red = 0;
                let mut num_blue = 0;
                for colour in &self.shirts {
                    match colour {
                        ShirtColour::Blue => num_blue += 1,
                        ShirtColour::Red => num_red += 1,
                    }
                }
                if num_red > num_blue {
                    ShirtColour::Red
                } else {
                    ShirtColour::Blue
                }
            }
        }
        let store = Inventory {
            shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
        };
        let user_pref1 = Some(ShirtColour::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!("The user with preference {:?} gets {:?}", user_pref1.unwrap(), giveaway1);
        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
    }
    // giveaway method used closure here. closure expression is the one inside unwrap_or_else which takes in parameter given between || (none here).
    // closures dont usually require you to annotate the types of parameters or the return value like fn functions do.
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // comparison with function:
    fn  _add_one_v1(x: u32) -> u32 {x+1}
    let _add_one_v2 = |x: u32| -> u32 {x+1};
    let _add_one_v3 = |x: u32| {x+1};
    // let add_one_v4 = |x| x+1;  // should work but doesnt dunno why :/

    

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time : {:?}", elapsed_time);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn calculate_length(s: &String) -> i32 {
    s.len() as i32
}

fn change(s: &mut String) {
    s.push_str(" dummystring");
}

fn another_function(x: i32) -> i32 {
    return x + 1;
}

fn takes_ownership(x: String) {
    println!("{x} some string");
}

fn makes_copy(x: i32) {
    println!("{x} some integer");
}

fn fahrenheit_to_celcius(x: f64) -> f64 {
    return (x - 32.) * 5. / 9.;
}

fn celcius_to_fahrenheit(x: f64) -> f64 {
    return x * (9. / 5.) + 32.;
}

fn fibonacci(x: u32) -> u32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}
