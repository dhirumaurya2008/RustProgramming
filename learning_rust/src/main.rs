fn main() {
        /* ######This section contains about ## Variables and Scalar Data Types */
        println!("\n\nStart Variables and Scalar Data Types \n\n");

        let x = 15; 
        println!("The value of variable is {}", x);
        println!("The maximum value of i8 is {}", std::i8::MAX);
        println!("The maximum value of i32 is {}", std::i32::MAX);
        println!("The maximum value of u32 is {}", std::u32::MAX);
        println!("The maximum value of f32 is {}", std::f32::MAX);

        //Char Data types
        let c1:char = 'a';
        let c2:char = '3';
        let c3:char = '+';
        let c4:char = '\u{288A}';
        let c5: char = '\"';

        println!("The value of c1 = {}, c2 is {}, c3 is {}, c4 is {}, c5 is {} ", c1, c2, c3, c4, c5);

        println!("\n\nEnd Variables and Scalar Data Types \n\n");
        /* ######This section contains about ## comment and print */
        print!("Hello World"); // This will print  but it will not move the cursor to next line.
        print!("Hello World 1"); // This will be printing in same line as previous one as previous print did not have ln
    
        // print!(10); // This won't compile as it is treated as integer constant and which require format specifier
    
        println!("The value is {}", 10); 
    
        println!("My first name:  {} and my Last name : {}", "Dhirendra", "Maurya");
    
        print!("This is a print command !!");
        print!("This is going to be printed on thesame line");
        print!("This is going to be printed on thesame line");
        print!("This is going to be printed on thesame line");
    
        println!("");
    
        print!("This is going to be 
        Printed on the same 
        lines");
    
        println!("\n\n This is going to be printed after one line");
        println!("\n\n This will have some empty space at the begining");
        println!("This is some text which willbe over-written \r This text will apear onthe screen");
    
        println!("\n doing {2} from {1} and i {0} it ", "like", 20, "programming");
    
        println!("{language} is a system programming language which is cool to {activity} in", activity="code", language="Rust");
        println!("The summation of 20 + 30 is {}", 20 + 30);
    
}
