// Chapter 4 - Ownership 

fn main() {
    
   // let s = "hello"; // String literal

    {
        let s = "hello"; // String literal inside a specific scope. 
        println!("String literal. {}", s);
    }
    
    {
        let s = "hello"; // String literal inside a specific scope. 
        
    }
    // println!("{}", s); // <---This is no longer in scope and will cause an error. 
 
    // String literals are immutable and live on the stack because they are of known size. 

    let s = String::from("hello");

    // String::from() created a mutable string that exists in the heap because this string is of unknown size. 

    println!("String Heap:{}", s);

    let mut s = String::from("hello \n");

    s.push_str("String Heap, World!");
    println!("{}",s);

    /*
        With a String type becuase the size is not fixed an amount of memory needs to be allcated to hold an unknown sized string. 
        This results in the String being stored in the Heap. We need to make sure that we clear the heap in languages without automatic
        garbage collection. 

        Rust handles this be making sure that items stored in the Heap have a single owner who is responsible for removal.
        This ownership is verified at compile time. 
    */
    let s = String::from("Before Inside Scope");
    {
        let s = String::from("Hello Inside Scope"); 
    }
    // s inside the braces is deallocated automatically once the scope is over. 
    println!("{}", s);

    println!("Moves --------------------------------------\n");

    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}", s2);
    
    // In this case both s1 and s2 are pointing to the same memory location on the Heap. If both of 
    // these go out of scope at the same time, this will lead to a double free memory error. 
    // A double free can lead to memory corruption and security issues. 
    // To prevent this, Rust considers s1 to no longer be valid. 
    
    // println!("{}", s1); // This line results in an error because s1 no longer has a valid pointer to the data. 

    // Because Rust invalidates the first reference in favor of the second, this is called a move. 
    // Now only s2 can free the memory and a double free is automatically prevented. 

    println!("\n\nClones --------------------------------------\n");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s1 = {}", s1, s2);

    println!("\n\nStack Only --------------------------------------\n");

    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y);

    // integers, booleans, floating-point types, chars, tuples (with the preceeding) are processed directly on the stack
    // and can be easily and quickly copied. 

    println!("\n\nOwnership and Functions --------------------------------------\n");

    {
        let s = String::from("hello"); 
        takes_ownership(s);

        // println!("{}", s); // This line will error out because s is no longer valid after passing to the function. 

        let x = 5; 

        println!("Still valid because it's on the stack. {}", x);

        makes_copy(x);    
    }

    fn takes_ownership(some_string: String){
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32){
        println!("{}", some_integer);
    }

    println!("\n\n Return Values and Scopes --------------------------------------\n");

        {
            let s1 = gives_ownership();
            let s2 = String::from("hello");
            let s3 = takes_and_gives_back(s2);

            // println!("{}", s2); // s2 is no longer valid.

            // s2 is moved into takes_and_gives_back and the moved into s3
        }

        fn gives_ownership() -> String {
            let some_string = String::from("yours");
            some_string
        }

        fn takes_and_gives_back(a_string: String) -> String  {
            a_string
        }


        // To use a value again when it's fed to a function it must also be returned. 
/* 
        {
            let s1 = String::from("hello");
            let(s2, len) = calculate_length(s1);
            println!("The length of '{}' is {}.", s2, len);
            // We fed in s1 and then brought it back as s2. 
        }
        fn calculate_length(s:String)-> (String, usize){
            let length = s.len();
            (s, length)
        }
*/

        println!("\n\n References and Borrowing --------------------------------------\n");

        {
            let s1 = String::from("hello");
            let len = calculate_length(&s1);

            // The & means passing by reference. The ownership of s1 is not moved. 

            println!("The length of '{}' is {}.", s1, len);
        }

        fn calculate_length(s: &String)-> usize {
            s.len() // Have to remember to leave off the semi colon for return values in Rust. 
        }

        // This concept of referencing also prevents the double free situation. Although &s1 points to the same value as s1,
        // it does not assume responsibility for removing it from the Heap. That is still s1's job. 

        // The action of creating a reference is called borrowing. Borrowers don't have Heap clearing responsibility. 

        /* 
        { 
            let s = String::from("hello");
            change (&s);
        }
        fn change( some_string: &String) {
            some_string.push_str(",world");
        }
        */
        // The change function can't modify the borrowed value. The compiler gives an error. References are immutable. 

        println!("\n\nMutable References --------------------------------------\n");

        {
            let mut s = String::from("hello");
            change(&mut s);
            println!("{}", s);

            // The &mut keyword means that change will alter the value it's borrowing.

        }
        
        fn change(some_string: &mut String){
            some_string.push_str(", world");
        }

        /* let mut s = String::from("hello");

        let r1 = &mut s;
        let r2 = &mut s;

        println!("{},{}", r1, r2);
        */

        // You can't burrow a value twice if it's mutable. The above code won't compile. 
        // This prevents data races at compile time. Data races can cause undefined behavior in code. 

        let mut s = String::from("hello, no race");
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;

        println!("{}", r2);

        // Multiple burrows can be made in separate scopes. Just not together in the same scope.

        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        //let r3 = &mut s; // Can't borrow s as mutable (changeable) because it's already borrowed in the same scope as immutable.

        println!("{},{}", r1, r2);

        println!("\n\nDangling References --------------------------------------\n");

        /* 
        {
            let referece_to_nothing = dangle();
        }
        fn dangle() -> &String{
            let s = String::from("hello");

            &s // This part returns a reference to s. But the scope of s ends after dangle is complete. This means 
               // that the reference is now pointing to nothing. Rust sees this possibility and blocks it at compile time.
        }

        */

        fn no_dangle() -> String {
            let s = String::from("hello");
            s // This is ok because the ownership is being tranferred out of the scope. 
        }

        let new_s = no_dangle();
        println!("{}", new_s);

        println!("\n\nSlice Types --------------------------------------\n");

        // A slice references a sequence of elements in a collection. It does not have ownership. 

        let s = String::from("hello world");
        let hello = &s[..5];
        let world = &s[6..];
        println!("{}:{}", hello, world);

        println!("\n\nChapter 5--------------------------------------\n");

        println!("\n\n Defining and Instantiating Structs --------------------------------------\n");

        struct User {
            active: bool,
            username: String,
            email: String, 
            sign_in_count: u64,
        }

        {
            let user1 = User {
                active: true,
                username: String::from("someusername123"),
                email: String::from("someone@example.com"),
                sign_in_count: 1, 
            };

            println!("Account Active: {}", user1.active);

            fn build_user(email: String, username: String) -> User {
                User {
                    active: true, 
                    username: username,
                    email: email, 
                    sign_in_count: 1, 
                }
            }

            let test_user = build_user(String::from("sample@test.com"), String::from("samplePerson"));
            println!("Email Address: {}", test_user.email);
        }

        {
            fn build_user(email: String, username: String) -> User {
                User {
                    active: true, 
                    username: username,
                    email: email, 
                    sign_in_count: 1, 
                }
            }

            let user1 = build_user(String::from("someuser@email.com"), String::from("someuser"));
            let user2 = User {
                active: user1.active,
                username: user1.username, 
                email: String::from("another@example.com"),
                sign_in_count: user1.sign_in_count,
            };

            println!("{}", user2.active);
        }

        {
            struct Color(i32, i32, i32);
            struct Point(i32, i32, i32);

 
            let black = Color(1, 0, 0);
            let origin = Point(1, 0, 0);
            
            println!("{}",black.0)
        }

        println!("\n\n An Example Program Using Structs --------------------------------------\n");

        // Without tuple
        {
            let width1 = 30;
            let height1 = 50; 

            println!("The area of the rectangle is {} square pixels.", area(width1, height1));

            fn area(width: u32, height: u32) -> u32 {
                width * height
            }
        }

        //With tuple
        {
            let rect1 = (30, 50);

            println!( "The area of the rectangle is {} square pixels", area(rect1)
            );

            fn area(dimensions: (u32, u32)) -> u32 {
                dimensions.0 * dimensions.1
            }
        }

        // With Struct
        
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        {
            let rect1 = Rectangle {
                width: 30, 
                height: 50,
            };

            println!(
                "The area of the rectangle is {} square pixels.", area(&rect1)
            );

            dbg!(&rect1);
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        println!("\n\n Chapter 6: Enums and Pattern Matching  --------------------------------------\n");
/*
        {
            enum IpAddrKind{
                V4, 
                V6
            }

            let four = IpAddrKind::V4;
            let six = IpAddrKind::V6;

            enum Option<T> {
                None, 
                Some(T),
            }

            let absent_number: Option<i32> = None;
        }
*/

        println!("\n\n match Control Flow Construct --------------------------------------\n");

        {
            enum Coin {
                Penny, 
                Nickel, 
                Dime, 
                Quarter,
            }

            fn value_in_cents(coin:Coin) -> u8 {
                match coin {
                    Coin::Penny => 1,
                    Coin::Nickel => 5, 
                    Coin::Dime => 10,
                    Coin::Quarter => 25,
                }
            }

            let change = Coin::Dime;
            let value = value_in_cents(change);
            println!("{}", value);
        }

        {
            #[derive(Debug)]
            enum UsState{
                Alabama, 
                Alaska,
                NewYork
            }
            enum Coin {
                Penny, 
                Nickel, 
                Dime, 
                Quarter (UsState),
            }

            fn values_incents2(coin: Coin) -> u8 {
                match coin {
                    Coin::Penny => 1,
                    Coin::Nickel => 5, 
                    Coin::Dime => 10, 
                    Coin::Quarter(state) => {
                        println!("State quarter from {:?}!", state);
                        25
                    }
                }
            }
            let change = Coin::Quarter(UsState::NewYork);

            println!("{}", values_incents2(change));
        }

        println!("\n\n matching with option<t> --------------------------------------\n");
        {
            fn plus_one(x: Option<i32>) -> Option<i32> {
                match x {
                    None => None, 
                    Some(i) => Some(i + 1),  
                }
            }
        }

        println!("\n\n Chapter 7: Managing Growing Projects with Packages, Crates, and Modules --------------------------------------\n");


        

}
