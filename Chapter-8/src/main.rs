fn main() {
    
    {
        let mut v: Vec<i32> = Vec::new();
        
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1,2,3,4,5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }

    {
        let v = vec![1,2,3,4,5];

       // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
        if(does_not_exist == None){
            println!("None");
        }
    }
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    }
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{i}");
        }
    }
    
}