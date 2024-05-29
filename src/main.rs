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
}