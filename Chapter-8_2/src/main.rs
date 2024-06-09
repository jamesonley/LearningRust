fn main() {
    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    //let s= "initial contents".to_string();
    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
        println!("s1 is {s1}");
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        println!("{s3}"); 
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{s}");
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}"); // Doesn't take ownership of any parameters. 
        println!("{s}");
    }
    {
        let hello = String::from("Hola");

        for c in hello.chars(){
            println!("{c}");
        }
        for b in hello.bytes(){
            println!("{b}");
        }
    }
}
