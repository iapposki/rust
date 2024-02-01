use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
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
    // packages and crates : crate is smallest amount of code that rust compiler considers at a time. 2 types, binary and library crates.
    // binary crates are promgrammes that can be compiled to executable, it has a main function. library crate dont and cant ompilte to executable.
    // crate root is a source file that the compiler starts from. default is src/main.rs
    // package is bundle of one or more crates. it can contain as many binary crates as required (in src/bin directory) and only one library crate (src/lib.rs). both can be present in the package at the same time.

    // #################### defining modules ##################
    // to declare a module, either 1. inline curly brackets that replace the semi colon here, 2. in src/garden.rs or 3. in the file src/garden/mod.rs.
    // to declare Submodules, agian either inline in curly brackets `mod vegetables`, or in garden.rs or in mod.rs in the garden directory as `mod vegetables`
    // paths to code in modules : code in module can be reffered anywhere in crate. for example `crate::garden::vegetables;:Asparagus` for an Asparagus type in the garden vegetables module.
    // Private vs Public : codes within modules are private from parent module by default use pub before to make it public. use inside modules as well as needed.
    // keyword : instead of using `crate::garden::vegetables;:Asparagus` every time instead use ` use crate::garden::vegetables;:Asparagus` so you only need to write Asparagus to make use of that type in the scope.
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);

    // ################### bringing paths into scope with the `use` keyword ##################
    // use only creates shortcut for the particular scope in which the use occurs. move the use to the required scope to make use of the shortcut

    // ############### storing list of values in vectors##################
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
    let _temp = v.get(100);
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

    // #################### STRING ##############################
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
