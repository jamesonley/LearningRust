fn main() {


    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }
    {
        fn largest(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list{
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest numbe is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }
    {
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result); 
    }
    { /* 
        fn largest<T>(list: &[T]) -> &T {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest 
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest (&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest (&char_list);
        println!("The largest char is {}", result);
    */ }
    {
        struct Point<T> {
            x: T, 
            y: T,
        }

        let integer = Point { x: 5, y: 10};
        let float = Point {x: 1.0, y: 4.0};
    }
    {
        struct Point <T, U> {
            x: T, 
            y: U, 
        }

        let both_integer = Point { x: 5, y: 10 }; 
        let both_float = Point {x: 1.0, y: 4.0}; 
        let integer_and_float = Point {x: 5, y: 4.0};
    }
    {
        enum Option<T> {
            Some(T), 
            None,
        }
    }
}
