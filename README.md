# RUST
Buy the course at : https://www.udemy.com/course/master-the-rust-programming-language/?kw=rust+pro&src=sac&couponCode=ST21MT30625G1
# 1. hello_world_001
# 2. print_002
# 3. EMI_calculator
#### important notes
- {:.}2 - 2 decimal places 
- :#X - convert to Hexadecimal
- :#b or :b 
- :p to print a raw pointer 
- r and r# can be used in place \ to escape special characters that may confuse the compiler , such as \ , "", etc
## useful rust tools
#### rustfmt - for formating your code for best readability
       //example usage
       cargo fmt
#### CLippy
        //you can add it to to Settings JSON file
        "rust-analyzer.check.command": "clippy"
#### cargo fix 
        cargo fix --help

# 4. Variables and Data Types
### Data Types (Primitive)
- Integers: i8 , i32, i64, isize , u8 , u16, u32, u64, usize
- Floating-point numbers: f32, f64
- Boolean : bool
- Character: char
- Arrays : [T;N], where T is the type elements and N is the number of elements
- Slices : &[T]
- Tuples: A tuple is a fixed-length collection of elements, where each elememt can have a different type. They are defined by a set of closing parenthesis enclosing comma-seperated list of typea (T1, T2, T3, ...)

#### defining variables
        let num = 32; // defaults to i32 
        let num: i32 = 100;
        // type casting 
        let num1: i32 = 400;
        let num2: i32 = 30;
        let result: i8 = (num1 + num2) as i8;  // without type casting it will throw an error
        println!("Sum = {}", sum);
####  mutable and immutable data types
- defininhg a variable with let , automatically makes it immutable, once assigned it can not be reassigned to a new value
- you can make it mutable by defining it with the keyword mut

        let num = 10;
        let mut sum = 0;
        sum = num + 50;
#### "as" keyword
- The "as" keyword is used for explicit casting, meaning it is used to convert a value from one primitive type into a value of another primitive type
- "as" is also used to rename imports in use and extern crate statements 

- suffixes to specify the type of a number 

        let num1 = 100_u8
        let num2 = 0xffffu32
        let num3 = 0.5_f32

#### Storing ascii code 
        //converting from ascii to u8
        let ascii_code_of_plus = b'+';
        println!("{}", ascii_code_of_plus); // thos will print out the value of + = 43 

        //converting from u8 to ascii
        let ascii_number = 43_u8;
        println!("{}", ascii_code_of_plus as char); // output will be '+'

#### Unicode Scalar values 
        //uses '\u{}'
        let infinity_symbol = '\u{221E}'; // 221E is a Hexadecimal value without 0x prefix
        println!("symbol = {}, usv = {}", infinity_symbol, infinity_symbol as u32)

        // converting from u32 to char
        // TO convert a u32 value to a char, you can use the char::from_u32 function, which return Option<char>

        fn main(){
                let inf_usv = 0x221e_u32;
                if let Some(inf_symbol) = char::from_u32(inf_usv){
                        println!("Symbol = {}", inf_symbol);
                }
                else{
                        println!("Not a valid Unicode scalar value");
                }
        }
#### Arrays

        // they type and size of an array can inferred by the compiler , but generally this is how it is defined
        let my_array: [f64; 3] = [2.5, 4.0, 3.8];
        let my_array1 = [2.5, 4.0, 4.2]; // this also works , by the compiler 
        
- since size of an array can not be modified at run tim, RUST does have a dynamic alternative to arrays called Vectors chich can change in size
- Vectors are implemented as a wrapper around dynamically allocated array and provide methods to push and pop elements , as well as other useful functionality. So in RUST you can usea Vector if you want a dynamic array

- printing an array 
        
        let array = [1, 2 ,3];
        // {:?} or {:#?} format specifier uses
        // Debug trait of the array which is used to print arrays in a concise and readable format , because 
        println!("{:?}", my_array);
        println!("{:#?}", my_array);

- Repeat expession

        let array: [i32 ; 10] = [0;10]; // creates an array and initializes with 10 zeros of type i32
        let buffer = [0_u8; 1024]; // creates a 1kb buffer
        println!("{:?}", buffer);

- Array indexing 

        let array1 = [5,4,3,2,1];
        let element = array1[3];
        println!("{}", element)

        //you can create a mutable array as follows
        let mut array2 = [4_u8; 5];
        array2[3] = 10;
        println!("{:?}", array2); // [4,4,4,10 , 4]

- Iterating over an array

        let array = [10, -67, 88, -5, -2, 99, 132, 42]; //[i32, 8]

        let mut sum = 0;
        for i in array{
                sum+=i;
        }
        println!("{}", sum);




# 5. Military time converter
1. calculate the values of 3 fields in HH:MM:SS. HH denotes Hours, MM denotes Minutes, and SS denotes Seconds.

2. To calculate Hours(HH) use the below formula

Hours = total_seconds/3600;

Note: total_seconds is the value entered by the user.

3. Find out the remaining seconds using the mod(%) operator

remaining_seconds = total_seconds % 3600

4. To calculate Minutes(MM) use the below formula

Minutes = remaining_seconds / 60

5. Find out the remaining seconds using the mod(%) operator which also happens to be the Seconds(SS)

Seconds= remaining_seconds % 60

6) Print in the format : HH:MM:SS

#### taking input 

        use std::io::{self, Write};
        println!("Please enter the time in seconds: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
         io::stdin()
        .read_line(&mut buf)
        .expect("Enter valid time in seconds!");

        let time_seconds: i32 = buf.trim().parse().expect("enter a valid integer");

# 6. Functions

        fn convert(initial_value: f64)->f64{
                retun initial_value/3600;  //return statement can also be skipped , and removing the semicolon
        }
- RUST does not allow functions to have the same name , but you can do it using modules 

        mod module_1{
                pub fn convert(initial_value: f64)->f64{ //pub makes the function public because when inside the module it is private 
                        initial_value/3600
                }
        }
        mod module_2{
                pub fn convert(initial_value: i32)->i32{
                        initial_value/3600
                }
        }

        fn main(){
                module_1::convert(4.00); //put the full path when calling it.
        }
# 7. Why RUST is safe 
- No null pointer dereference
- No use after free
- No dangling pointers 
- No double free
- No buffer overflows
- Memory leaks are mostly( No language completely avoids them in all scenarios)
- No Data races (thread accessing memory location at the same time )
- No uninitialized memory access
- No type confusion
        
        static FAIL_SAFE_MODE: AtomicBool = AtomicBool::new(false);
        
        fn main(){
                panic::set_hook(Box::new(|info| {
                        FAIL_SAFE_MODE.store(true, Ordering::SeqCst);
                        println!("Panic occured: {}", info);
                        println!("Entering fail-safe mode...");
                        let buffer = [1,2,3,4,5];
                }));

                let result = panic::catch_unwind(||{

                        for i in 0..10{
                                //panics for i>=5
                                println!("Accessing index {}: {}", i , buffer[i]);
                        }
                });

                if FAIL_SAFE_MODE.load(Ordering::SeqCst){
                        println!("System is now in fail-safe mode.");
                }

                match result{
                        Ok(_)=> println!("No panic occured."),
                        Err(_)=> println!("Panic caught! Execution Continues")
                }
        }
# 8. Testing
1. Unit testing: Tests individual functions, methods, or modules in isolation to ensure that they perfom as expected.
2. Integration Testing: Tests the integration of multiple components or the entire module as a wholeto ensure they work together correctly
3. Documentation Testing (Doc-Tests): Ensures that code examples in documentation are accurate and executable.

- Rust does not natively support mocks. You can create mocks manually or use third party libraries.

## Software development practices 
- Test-Driven Development (TDD) - write the test cases first - write code to pass test case (iterate)
- Test-Last Development - tests are written after the code has been developed

        #[cfg(test)]
        mod tests{
                // helper function used by test cases 
                fn helper_functio(){

                }
                #[test] //this attribute tells cargo to treat this function as a test case (can only be executed using the command cargo test)
                fn test_your_test_case_1(){

                }
                #[test]
                fn test_your_test_case_2(){

                }
        }
# 9. References
- can be thought of as a pointer in C, in thesense that it refers to the memory location of a value. They are different from pointers from C in that:
1. References are immutable
2. Cannot be null and will never dangle
3. The borrow checker ensures that the reference will only be valid for the lifetime specified and will not outlive that lifetime

        fn main(){
                let value: i32 =  42;
                let ref_of_value = &value; //immutable reference
                println!("Value is {}", *ref_of_value); // Manual dereferencing
                println!("value is {}", ref_of_value);  // Automatic dereferencing
                println!("memory address of stored value is {:p}",ref_of_value ); 
                println!("memory address of stored address is {:p}",&ref_of_value ); 
        }
### Slice 
- slice is used to reference a portion of an array but they can also be used to reference other types of contiguous data structures, such as a string or a vector.
- The true power of a slice lies in its ability to let you work with a portion of data efficiently, without taking ownership , copying or allocation

        fn main(){
                let array: [i32; 4] = [1,2,3,4];

                let s1 = &array[1..=3]; // s1 is a slice whose type is &[i32]
                let s2 = &array[..]; // the entire array
                let s3 = &array[0..2]; // 0 to 2 not inclusive of 2
                println!("{:?}", s1); //output is [2,3,4]

                //start..end  -> start<=x<end
                //start..=end -> start<=x<=end
                //start.. -> start to the rest
        }

- Modifying from a slice 

        fin main(){
                let mut array = [-56, -1, 10 , 20 , 70 , 400];
                let s1 = &mut array[1..=4]; // for us to modify the from the slice the slice itself has to be muatble and has to follow the rules
                s1[2] = 100;
                println!("{:?}", array); // the output is  [-56, -1, 10, 100, 70 , 400]

        }
![alt text](image.png)

#### looping through an array 
        fn main(){
                let array = [-56, -1, 10 , 20 , 70 , 400];
                let slice = [0..=3];
                let mut sum = 0;
                //type of slice is &[i32]
                //i is a loop variable which is a reference not the actual value, and it is of type &i32, although it works , the compiler figures that out
                for i in slice{       // this also works for &i in slice{}, and in that case the type i will i32 instead of &i32
                        sum = sum+*i; // this also works sum = sum + i
                }
                println!("sum: {}", sum);
        }

### Borrow , Borrower and Referent

##### Mutable Borrow
        fn main(){
                let mut num1 = 42;
                let ref_of_num1 = &mut num1; // adding the mut before the num1 make the reference to be mutable
                *ref_of_num1 = 100;
                println!("{}", num1) // the code works , the output is 100
        }

                fn main(){
                let mut num1 = 50;
                let r1 = &num1; // type of 'r1' is &i32
                let r2 = &mut num1; //type of 'r2' is &mut i32


        }
- RUST does not allow us to create multiple mutable borrows because the is a good chance of data race condition 
                fn main(){
                let mut num1 = 50; //mutable referent
                let r1 = &num1; // immutable borrow
                let r2 = &num1; // immutable borrow
                let r3 = &num1; // immutable borrow

        }

#### Array and Slice methods
- Reverse

        let mut array = [2,3,4,5,6];
        array.reverse();

- Sort

        let mut array = [3,5,1,3,6,7];
        array.sort(); // will sort in ascending order --- checkout docs for more sort functions

- Find the biggest

        //sort then return the last ;last index
        let mut array = [3,5,1,3,6,7];
        array.sort(); // will sort in ascending order --- checkout docs for more sort functions
        let length = array.len();
        println!("Biggest number: {}", array[length - 1])


- Concat

        let array1 = [1,2,3];
        let array2 = [4,5,6];
        let array3 = [array1, array2].concat();
        println!("{:?}+ {:?} = {:?}", array1 , array2 , array3);

- Split_at methods

         let mut array = [3,5,1,3,6,7];
         let (l,r) = array.split_at(2);
         println!("{:?}",l); // prints the left hand side of the split array 
         println!("{:?}",r);

# 10. Decision Making

- if... else
- if..else if ..else
- if..let
- match

#### If  Expression 
        let age = 15;
        let message = if age< 18{
                println!("This is the if statement"); //you have to terminate this with ;
                "you can not vote" // no need for ; because it is an expression
        } else {
                println!("This is the if statement"); //you have to terminate this with ;
                "you can vote"
        }
#### Match statement

        fn main(){
                let x: u8 = 1; // the match statement has to be exhaustive for instance we said x is of type u8 , which means you have to handle values upto 127

                match x {
                        1 => println!("one"),
                        2 => println!("two"),
                        _ =>println!("Something else"), // _ means other cases
                }
        }
        // the output for this code is "one"

        //another example
        fn main(){
                let array1 = [1,-2,3,4];

                let invalid_array = match array1{
                       [n, _ , _ , _ ]|  [_, n , _ , _ ]|
                       [_, _ , n , _ ]|  [_, _ , _ , n ] if n < 0 =>{
                        true
                       }
                       _=> false ,
                };
                if invalid_array{
                        println!("Array is invalid");
                } else {
                        println!("Array is valid")
                }
        }

#### Matches statements 

        let invalid_array = matches!(array1 , [n, _ , _ , _ ]|  [_, n , _ , _ ]|
                       [_, _ , n , _ ]|  [_, _ , _ , n ] if n < 0 ); // this still works the same way as the above statement 

#### if..let, else if let 
- pattern matching like the match statement 

        fn main(){
                let point = (4,2);

                if let (0,0) = point{
                        println!("y is within the range 1..4");
                } else if let (_,y @ 1..=4) = point{
                        println!("y = {} is within the range 1..4", y);
                } else {
                        println!( "y is out of range");
                }
        }

#### Bitwise Operations 

| Hex | Binary | Hex | Binary |
| --- | ------ | --- | ------ |
| 0   | 0000   | 8   | 1000   |
| 1   | 0001   | 9   | 1001   |
| 2   | 0010   | A   | 1010   |
| 3   | 0011   | B   | 1011   |
| 4   | 0100   | C   | 1100   |
| 5   | 0101   | D   | 1101   |
| 6   | 0110   | E   | 1110   |
| 7   | 0111   | F   | 1111   |

        //Extract the 4th to 12th bit positions of the number
        fn main(){
                let num = 0x00ABCDEF;
                let mask  = 0x1FF << 4 ; // to mask we use 1 1111 1111, which is converted to hexadecimals to 0x1FF , but we left shift it by 4 positions

                let res = ((num & mask)>>4) & 0x1FF; // & 0x1FF is the optional final clean up to ensure we have 9 bits , its a common defensive habit
                println!("{:#X}", res )
        }
        /*
        num  = 0x00ABCDEF           0000 0000 1010 1011 1100 1101 1110 1111
        mask = 0x00001FF0           0000 0000 0001 1111 1111 0000 0000 0000
        -------------------------------------------------------------------
        num & mask = 0x00000DE0     0000 0000 0000 0000 1101 1110 0000 0000
        */

# 11. Convert 32-bit RGBA8888 color format to 16-bit RGB565 color Format Exercise
Write a program which accepts 32-bit RGBA8888 color format in hex  from the user and converts that into 16-bit RGB565 color format

 Hints
 ============================

1) Extract the red, green, and blue components from the 32-bit RGBA input.

2) Scale down these color components to fit the RGB565 format, which allocates 5 bits for red and blue, and 6 bits for green.

For example if 0xABCDEFEE is in RGBA8888 format, in binary it would look like below

    10101011(R)   11001101(G)   11101111(B)   11101110(A)

    to convert this into RGB565,

i) Neglect A

ii) in R consider only most significant 5 bits

iii) in G consider only most significant 6 bits

iv) in B consider only most significant 5 bits



3) Removing Hex prefix from user input:

Use the 'trim_start_matches' method on the string to remove the "0x" or "0X" prefix

4) To convert String to Integer U32 value , explore

u32::from_str_radix() with radix = 16



 - Expected Output
=============================

Enter RGBA8888 data in hex format: 0xABCDEFEE

0xABCDEFEE RGB565 equivalent is 0xAE7D

- FInd the code in rgba888_to_rgb565


# 12 Strings
##### Different types of strings in Rust 
1. String literal (&str) - UTF8 encoded characters , often created from a string literal in the source code. it has a fixed size.
        
        let message  = "Good Morning";// this is a string literal. The information "Good Morning " is hardcoded in the binary of the program

2. String: an owned , heap allocated string type, which provides methods for mutating its contents and has a dynamic size

        fn main(){
                let mut greeting = String::from("Good morning");
                let mut greeting = String::new(); // creating an empty string

                let mut num_string = 3.148.to_string(); // this also works on all datatypes that have access to the display attribute
        }
#### Memory representation
![alt text](image-1.png)

- Heap memory allocated to hold the string string will be deallocated automatically
### Copying for Strings 
1. Shallow copying 
2. Deep copying 

##### 1. Shallow copy
 - when a String value is assigneed to another variable, the varible is assigned a copy of the pointer, length , and capacity of the original String value, but the underlyig heap memory is not copied. This is also known as copy by reference

        fn main(){
                let s1 = String::from("hello");
                let s2 = s1; //s1 is no invalid, for integers this is not the case , String uses the heap memory that's why 
                println!("{}, world", s2); //s2 points to the same memory as s1
        }

![alt text](image-2.png)


##### 2. Deep copy
- When a String value is cloned, a new heap allocation is created with the same contents as the original String. This is also know as copy by value.

* In rust Every piece of Data has a single owner at any given time. One exception is Reference Counted(RC) type 

         let my_string = String::from("Hello World");
         mystring2 = my_string.clone();

#### SLice of a String 
- similar to arrays 
- you can not modify the original string from a slice by indexing, it is of type &[T]

#### Converting a String into a slice or string literal

        fn print_string(arg: &str){
                println!("{}", arg);
        }

        fn main(){
                let s: String = String::from("Hello");

                let slice = &s; //type of 'slice is &String  --> &str'

                print_string(slice);
                let slice2 = s.as_str() //type of slice2 is &str

                let slice3 = s.as_str() //type of slice3 is &mut str
        }
#### Converting a string literal or slice  into a String

        fn main(){
                let s = "Good morning ";
                let string1 = s.to_string(); // type of string1 is String 
                let string = String ::from(s); // type of string is String 

        }

#### String to byte array
        fn main(){
                let message: String  = String::from("hello+∞+§);
                let byte_slice: &[u8] = message.as_bytes();
                for byte: &u8 in byte_slice{
                        print!("{:#X}\t", byte); //the output is out the above UTF8 is stored in the memory
                }
        }

#### Iterating on a string

        let computer_in_hindi = "ºª•∑¶§™£¢∞¶§•ªª¶§∞§¢™≥≤`¡";

        for ch in computer_in_hindi.chars(){
                print!("{}", ch);
                print!(" ");
        }

# Password Validity project 
        Write a program to check validity of a password input by users Following are the criteria for checking the password:
        1. atleast 1 letter between a-z
        2. atleast 1 number btween 0-9
        3. atleast 1 letter between A-Z
        4. atleast 1 character from [$#*]
        5. minimum length of password : 6
        6. Maximum length of password: 12

# 13. Ownership , Move and Copy Semantics
         // Move

         fn main(){

                let array1 = [5, 6, 7,8]; //[i32; 4] , these will be in the stack memory
                let array2: [String;3] = [
                        String::from("foo"),
                        String::from("bar"),
                        String::from("baz"),
                ];   // these will be in the heap memeory, but their refernces will be in the stack 

                //doing this will work because it is of type i32 , which has a copy trait

                let item = array1[2]; // this is because is it of copy trait , 
                let str_item1 = array2[1]; however this does not work for the String in rust because it make the reference to be invalid 

                // something that works is indexing as a slice 
                let str_item : &str = &array2[1];
                println!("{}", str_item);

                //looping through the String 
                for str in &array2{
                        println!("{}",str);
                }
                println!("{:?}", array2);

         }


#### &mut T and &T
- &mut T is Move while &T is Copy


#### Call by value and Call by reference 

        fn main(){
                let arr = [10, 20, 30, 40 , 50];

                // Call by value
                let max_value = find_greatest_value_by_value(arr);
                println!("Maximum value by value: {}", max_value);

                // Call by reference
                let max_value = find_greatest_value_by_reference(&arr);
                println!("Maximum value by reference : {}", max_value)
        }
        fn find_greatest_value_by_value(arr: [i32; 5])-> i32 {     // Copy
                //ToDo
        }
        fn find_greatest_value_by_reference(arr: &[i32])-> i32{   // Move
                //ToDo
        }

# 14. Loops

#### Loop : 
- used to create an infinite loop that continues indefinitely until a break statement is encountered within the loop

        fn main(){
                let mut i = 0;

                loop{
                        if i == 3{
                                break;
                        }
                        println!("i = {}", i);
                        i += 1;
                }
                println!("loop ends");
        }
        // Break with return value 
        fn main(){
                let mut i = 0;

                let result = loop{
                        if i == 3{
                           break i*2;
                        }
                        println!("i = {}", i);
                        i += 1;
                }
                println!("result = {}", result);
        }
        // Loops can also have labels 
        fn main(){
                'outer: loop{
                        println!("Outer loop");

                        'inner: loop{
                                println!("Inner loop");

                                loop{
                                        println!("inner loop2");
                                        break 'outer
                                }
                        }
                }
        }


#### while:
- used to loop over a block of code as a specified condition remains true, condition based iterating

#### while let 
- This is a variant of the while loop that allows you to match againsta pattern and extract variable from it , continuing to loop as long as the pattern matches

#### for in: 
- Used to iterate over a collection of items such as an array , vector , or range. It is similar to the for loop in other programming languages


#### iterating over an immutable reference

        fn main(){
                let words = 
                [
                        "hello".to_string(),
                        "world".to_string(),
                        "how".to_string(),
                        "are".to_string(),
                        "you".to_string(),
                ];
                //iterate by value
                for str in words{
                        println!("{}", str);
                }
                // this takes the ownership of the array and you can't print it after the for loop


                //iterate by reference , this does not take ownership rather it borrows
                for str in &words{
                        println!("{}", str);
                }
        }
#### iterating over mutable reference

        fn main(){
                let mut words = 
                [
                        "hello".to_string(),
                        "world".to_string(),
                        "how".to_string(),
                        "are".to_string(),
                        "you".to_string(),
                ];
                //iterate by mutable reference
                // type of 'str' is &mut String
                for str in &mut words{
                        if str == "hello"{
                                str .push_str("good morning");
                        }
                }

                println!("{:?}", words);


}

#### Iterating over a string slice

when using the for loop think of a type that implements an iterator

        let msg = "Hello world";
        let target_char = "l";

        let mut count = 0;
        
        // for c in msg: would not work because msg is not of iterator type 
        for c in msg.chars(){
                if c == target_char{
                        count+=1;
                }
        }

# 15. Palindrome Project 
- To determine if the given char array is a palindrome, loop over the array up to half its length. During each iteration of the loop, compare the character at the current index from the start of the array with the character at the corresponding index from the end of the array. If all matching characters are equal, then the array is a palindrome.

- the solution is in the palindrome project directory

# 16. Tuples 

- A tuple is a data structure that allows you to group together multiple values of different types into a single, ordered collection.

        let my_tuple = (1 , "hello", true);
        let my_tuple: (i32, &str , bool) = (1, "hello", true);
        prntln!("{:?}", my_tuple); // does not implement the display trait therefore uses the debug trait

- Tuples in Rust do not have a named parameters. instead, the elements of a tuple are accessed by their position using indexing starting at 0.

        let number = my_tuple.0;  //indexing 
        let msg = my_tuple.1; 
        let boolean = my_tuple.2;

        // tuple destructing syntax
        let (number , message, is_valid) = my_tuple;

##### Mutable tuple 

 fn main(){
        let mut number = (1, 2, 3);
        increment_numbers(& mut number);
        println!("{:?}", number);
 }

 fn increment_numbers(num_list: &mut (i32, i32, i32)){
        num_list.0 += 1;
        num_list.1 += 1;
        num_list.2 +=1;
        // Rust provides automatic dereferencing for method calls and field access when you have a reference to an object such as a tuple,, or an object of a struct or enum

 }
 
 #### Tuples can be nested 

        let grid = ((1,2,3), (4,5,6), (7,8,9));
        println!("Middle element: {}", grid.1.1);

#### Pattern matching in tuples : Tuples comparison 

- Tuples can also be compared lexicographically using operators like ==, != , <, >, <=, and >= as long as all the types in the tuple implemenmt the PartialOrd and PartialEq trait 

- The PartialOrd trait is used to define the ordering relationship between two values , while the PartialEq trait is used to define the equality relationship between two values.

- Example : Process the middle element only when the first element is greater than 0 , the last element is less than 10

        fn main(){
                let rcvd_data = (5, "hello",  8);

                match rcvd_data {
                        (a,s,c) if a >0 && c < 10 => {
                                println!("Valid data: s = {}", s);
                        }

                        _ => println!("Invalid data"),  // match has to be exhaustive
                }
        }


        fn execute_command_refactored(mode: &str, status: &str) {

        match (mode, status) {

                ("admin","active") => {println!("Admin privileges granted. Executing active command.")},
                ("normal","pending") => println!("Normal operation. Execute pending or active commands."),
                ("maintenance","complete") => println!("Maintenance complete. System can resume normal operation."),
                _ => println!("No action needed or invalid mode/status."),
                
        }
        }
        
        
        fn main() {
        execute_command_refactored("admin", "active");
        execute_command_refactored("normal", "pending");
        execute_command_refactored("maintenance", "complete");
        execute_command_refactored("admin", "pending");  
        }

#### pattern matching with tuple using the rest operator (..) to ignore some elements 

        fn main(){
                let tup = (1, "hello", 2.5. true. 'a');

                match tup {
                        (_, _, c , ..) if c > 2.0 => println!("The third element is greater than 2.0"),
                        _ => println!("The third element is less than or equal to 2.0"),

                }
        }

- The .. syntax is called the "rest" operator in Rust. It can be used in destructing pattern to match any remaining elements in a tuple , array , or struct


#### pattern matching with tuple  using variable binding and rest operator

        fn main(){
                let tup = (10, "hello", 2.5 , true , 'a');

                match tup{
                        (a@10, b @ "hello", ..)=> println!("The first and second element: {} and {}", a , b),
                        _ =>println!("The tuple does not match the pattern"),
                }
        }

#### Move while matching 
Remember this 
- i32 -- copy 
- String -- move 

- ref is the possible fix for this 

#### ref keyword

        fn main(){
                let the_date = (
                        "Monday".to_string(),
                        "25".to_string(),
                        "June".to_string(),
                        "2023".to_string(),
                );

                match the_date{  // alternatively you can use  &the_date , instead of ref to borrow the whole tuple
                        (ref day, ..) if day .as_str() == "Sunday" =>{
                                println!("Its Sunday");

                        }
                        _ => println!("Some other date"),
                }
                println!("{:?}", the_date); // this will print out the the_date without an error , because of the ref word which allows the match to borrow rather than move which happen if the ref is not used
        }
        

# 17. Fruits Sort Project 
You are required to write a program to sort the (fruit-name, price, quanity) tuples by

ascending order where fruit-name is string, price and quantity are numbers.

1: Sort based on fruit-name;

2: Then sort based on price;

3: Then sort by quantiy.

The priority is that fruit-name > price > quantity.

If the following tuples are given as input to the program:

Mango-us,50,80

Mango-uk,50,80

Orange,19,80

Blackberry,20,90

Blueberry,17,91

Blueberry,17,93

Blueberry,21,85



Expected output:

("Blackberry", 20, 90)

("Blueberry", 17, 91)

("Blueberry", 17, 93)

("Blueberry", 21, 85)

("Mango-uk", 50, 80)

("Mango-us", 50, 80)

("Orange", 19, 80)



Hint :

1) Remember you can compare the tuple lexicographically.

2) Use bubble sort. logic is given in the code.

You need to compare the ith tuple with the (i+1)th tuple and swap them if necessary

3) Use swap() method for swapping array elements (https://doc.rust-lang.org/std/primitive.slice.html#method.swap)



# 18. Structs
- Structs are user-defined data types that enable you to organize and encapsulate related pieces of data with different data types into a single unit, using named member fields for convenient access and manipulation.
- By defining a struct, the programmer can create a custom data type that is tailored to the needs of their.

        struct Person {
                name: String,
                age: u32,
                address: String, 
        }

#### Different types of structs
- There are three main types of structs in Rust :
1. Tuple struct : Struct that has unnamed fields 
2. Named struct: Struct that has named fields
3. Unit Struct: Struct that has no fields 

1. Named Struct 

        struct Person{
                name: String,
                age: u32,
                address: String,
        }

        fn main(){
                let person = Person{
                        name : String::from("Alice"),
                        age: 25,
                        address: String::from("123 Main St"),
                };
                // use the dot (.) operator to print the values of each member element 
                println!("Name: {}", person.name);
                println!("Age: {}", person.age);
                println!("Address: {}", person.address);
        }

- Mutable struct 
        #[derive(debug)]  //inorder to print the struct, it does not implement the debug or display trait therefore this is the way
        struct Person{
                name: String,
                age: u32,
                address: String,
        }

        fn main(){
                // create a new mutable Person struct
                let mut person = Person{
                        name : String::from("Alice"),
                        age: 25,
                        address: String::from("123 Main St"),
                };

                person.name = String::from("Bob"); // this works because of the mut keyword

                //print the updated struct
                println!("{:?}", person);

                //move 'name' field of struct is uninitialized 
                // let _name = person.name;
        }

#### Structs copy or Move ?
- Structs are by default Move irregardless of their member contents , but you can make them to be copy if their member elements are copy eg i32 using the derive trait


        #[derive(Copy , Clone, debug)] 
        struct Person{
                x1: i32,
                x2: i32,
                x3: i32,
        }

#### Default trait 
- as we know we have to initialize a struct , you can actually give the struct default values using the default trait

        #[derive(Debug, Default)]
        struct Person{
                name: String,
                age: u8,
                is_male: bool,
                height: f32,
                initial: char,
                address: String,
        }

        fn main(){
                // create a new mutable Person struct
                let user= Person::default();
                println!("{:?}", user);

        }

#### Updating a struct using another struct 

- they have to be of the same type

        #[derive(Debug)]
        struct Person{
                name: String,
                age: u8,
                is_male: bool,
                height: f32,
        }

        fn main(){
                // create a new mutable Person struct
                let user1= Person{
                        name: String::from("Ronie"),
                        age: 23,
                        is_male: true,
                        height: 5.4

                };

                let user2= Person{
                        name: String::from("Leslie"),
                        ..user1

                };

                let user3= Person{
                        name: String::from("Sam"),
                        ..user1

                };
                println!("user3: {:?}", user3);

        }


#### Tuple Struct

- Tuple structs are similar to tuples in that they don't have named fields . Instead, the fields are accessed by their index within the tuple. However unlike the normal struct the tuple structs posses a name , which can be advantageus in providing additional context and clarity to the code

        struct Point (i32, f64, u8;)

        fn main(){
                let point = Point(10, 3.5, 1);
                println!("x: {}, y: {}, z: {}", point.0, point.1, point.2);
        }
         
#### Methods and associated  functions of a struct

##### Methods 
- Methods are associated with a specific instance of the struct or enum
- Methods are defined within the impl block for the struct or , and their first parameter is always a reference to the instance of the struct or enum
- struct methods are accessed using instance of the struct followed by the dot(.) operator. For example, if we have an instance of the Person struct named person with a method named introduce, we can access it as person.introduce().

##### Associated functions
- Associated functions are not tied to any particular instance and are called on the type itself.
- Associated functions are defined within the impl block as well,  but their first parameter is not a reference to the type.
- Associated functions are accessed using the struct's name followed by the double colon (::) operator
- For example, if we have a struct named Person with an associated function new, we can access it as Person::new().


        struct MyStruct {

        }
        // f1() and f2() are methods / associated functions of 'MyStruct'
        // Put them inside impl  <Struct-Name> block

        impl MyStruct {
                fn f1(...){
                        .....
                }
                fn f2(...){
                        ......
                }
        }

        fn main(){
                // create an instance of 'MyStruct'
                let struct_inst = MyStruct{
                        ......
                };
                // METHOD CALL
                struct_inst.f1();
        }

- Another Example
* self keyword in a struct method signifies the instance of the struct on which the method is being called. It is analogous on which the method is being called. It is to 'this ' in many other OOP languages

- Associated 
        //Associated function 
                struct Point {
                x: f32,
                y:f32,
        }

        fn distance_from_origin(point: &Point)-> f32 {
                (point.x.powi(2) + point.y.powi(2)).sqrt()
        }

        fn main(){
                let p = Point{x:3.0, y: 4.0};
                println!("distance: {}", p.distance_from_origin(&p));
        }

- Methods 
        //Methods borrowing self immutably and mutably
        #[derive(Debug)] // to enable printing using the debug trait since it is not not implemented for the structs 
        struct Point {
                x: f32,
                y:f32,
        }

        impl Point {
                //Method borrowing self immutably
                fn distance_from_origin(self: &Point)-> f32 {    // this also worksfn distance_from_origin(&self)-> f32
                        self.x.powi(2) + self.y.powi(2).sqrt()
                }
                //Method borrowing self mutably
                fn translate(&mut self , dx: f32, dy: f32){
                        self.x += dx;
                        self.y += dy;

                }
                // Method that takes ownership of the self
                fn into_tuple()->(f32, f32){
                        (self.x, self.y)
                }
                // Using self as the first parameter of a method means the method takes ownership of the instance and consumes it, transforming it into something else. This is useful when you want to prevent the caller from using the original instance after the transformation.
        }

        fn main(){
                let mut p = Point{x:3.0, y: 4.0};
                println!("distance: {}", p.distance_from_origin());
                println!("modified Points: {:?}", p);
        }

- Associated functions
        #[derive(Debug)] // to enable printing using the debug trait since it is not not implemented for the structs 
        struct Point {
                x: f32,
                y:f32,
        }
        impl Point {
                //Method borrowing self immutably
                fn distance_from_origin(self: &Point)-> f32 {    // this also worksfn distance_from_origin(&self)-> f32
                        self.x.powi(2) + self.y.powi(2).sqrt()
                }
                //Method borrowing self mutably
                fn translate(&mut self , dx: f32, dy: f32){
                        self.x += dx;
                        self.y += dy;

                }
                // Method that takes ownership of the self
                fn into_tuple()->(f32, f32){
                        (self.x, self.y)

                }
                // Using self as the first parameter of a method means the method takes ownership of the instance and consumes it, transforming it into something else. This is useful when you want to prevent the caller from using the original instance after the transformation.

                // Associated function
                fnn from_tuple(coords: &(f32, f32))-> Point{
                        Point{c: coords.0, y:coords.1}
                }
        }

        fn main(){
                let mut p = Point{x:3.0, y: 4.0};
                let tuple = (10,20);
                let q = Point::from_tuple(&tuple);
                //println!("distance: {}", p.distance_from_origin());
                //println!("modified Points: {:?}", p);
                println!("Points from tuple: ({},{})", q.x,q.y);
        }

- Associated functions as constructor of a struct
        #[derive(debug, Default)]
        struct Rectangle {
                width: u32,
                height: u32,
        }
        impl Rectangle {
                fn new(w: u32, h: 32) -> Rectangle{   // it is a convention to use the word new for the constructor of the 
                        Rectangle {width: w, height: h}

                }

                fn new_with_default()-> Rectangle {
                        Rectangle::default()
                }

        }

        fn main(){
                let rect = Rectangle{
                        width: 10,
                        height: 20
                };
        }

##### Pattern Matching with structs 
        #[derive(debug, Default)]
        struct Rectangle {
                width: u32,
                height: u32,
        }

        fn main(){
                let rect = Rectangle {
                        width: 10,
                        height: 20
                };

                match rect {
                        Rectangle{ width : w , height : h} if w == h => {
                                println!("The rectangle is square.");

                        }, 
                        Rectangle{ width : _ , height: _ } => {
                                println!("The rectangle is not square.");
                        }
                }

        }

** Another Example **

        #[allow(dead_code)]

        struct Point {
                x: i32,
                y: i32,
        }

        struct Rectangle {
                top_left: Point,
                top_right: Point,
        }

        fn main(){
                let rect = Rectangle {
                        top_left: Point{ x: 0, y:10},
                        bottom_right: Point{x:20, y:0},
                };

                match rect{
                        Rectangle{ top_left: Point {x: 0, ..}, ..} =>{
                                println!("The top-left corner of the rectangle is on the x- axis")
                        },
                        Rectangle{..} => println!("The rectangle is somewhere else."),

                        }
        }
        

** Ref keyword and @ **

        #[derive(Debug)]
        struct Person{
                name: String,
                age: i32,
        }

        fn main(){
                let person  = Person{
                        name: "Ram".to_string(),
                        age: 35,
                };

                match person {
                        Person { age : p_age @30 , name: _} => println!("A person with age {}.", p_age),
                        Person  {ref name , age: 35} => {
                                println!("Ram with age 35 found.");
                        }
                        _ => println!("Not sure who the person is. "),
                }

                println!("{:?}", person)
        }

# 19. Enums

- An enum is typically used in any programming language to represent a value that cann be one of several possible variants
- when you define an enum in Rust, you are essentially creating a new type that can have one of several possible variants. Each variant can be associated with its own set of data, which allows you to represent complex data structures usng single enum.
- Enums have methods associated with them , just like structs 
        struct Point {
                x: i32,
                y: i32,
        }
        enum CarStatus {
                MovingUp (u32, i32, i32), // we can have something like this MovingUp{speed: u32, x: i32 , y: i32}
                MovingDown,
                NotMoving (Point),
                NotWorking,
        }

        fn main(){

                let current_car_status = CarStatus::NotMoving(Point{x: 0 , y: 0});

                current_car_status = CarStatus::MovingUp(100, 67, 78);

                //Pattern matching
                //if car is movingUp, print its Speed 
                // if car is NotMoving then print it x coordinate value 
                match current_car_status{
                        CarStatus::MovingUp(a, ..)=>{
                                println!(" Car is moving up with speed : {}", a);
                        }
                        CarStatus::NotMoving(x, ..)=>{
                                println!(" Car is not moving and x is  : {}", x);
                        }
                        _=> println!("Car is not moving"),
                }
        }

##### Methods and associated functions of an Enum

        struct Rectangle {
                x: f32,
                y: f32,
                h: f32,
                w: f32,
        }

        enum Shape {
                Circle {x:32 , y:32, radius : f32},
                Rectangle(Rectangle),
                Square(f32, f32 , f32),
        }

        impl Shape {
                fn new_circle(x: f32, y: f32, radius: f32)-> Shape {
                        Shape::Circle{x, y, radius }
                }

                fn area (self: &Shape) -> f32{
                        match self{
                                Shape::Circle {radius: r, ..} =>{
                                        std::f32::consts::PI * r * r
                                }
                                Shape::Rectangle(rec) =>{
                                        rec.h * rec.w
                                }
                                Shape::Square(_,_,s) =>{
                                        s*s
                                }
                        }

                }
        }

        fn main(){

                let new_shape = Shape::new_circle(0_f32, 2.0, 2.5);
                let area = new_shape.area();
                // print(area);

        }

##### Pattern matching usingg enums

        enum lightState {
             On { brightness: u8}  ,
             Off,
        }

        fn main () {
                let bulb = LightState:: On { brightness : 200};

                if let LightState :: On { brightness: 200} = bulb {
                        println!("Brightness is : {}", brightness);
                }
        }

##### The Option<T> type USAGE
###### Find the biggest string 

        fn find_biggest_item(strings: &[&str]) -> Option<&str>{
                let longest: Option<&str> = None;
                for item in strings{
                        if longest.is_none() || (item.len() > longest.unwrap().len()){
                                longest = Some(item)
                        } 
                }
                longest
        }

        fn main(){
                let strings = ["Mango", "Banana", "Apple"];

                let biggest_item = find_biggest_item(&strings);
                println!("Biggest item: {}", biggest_item.unwrap());
        }

        // Using the match 

        fn find_biggest_item(strings: &[&str]) -> Option<&str>{
                let longest: Option<&str> = None;
                for item in strings{
                        if longest.is_none() || (item.len() > longest.unwrap().len()){
                                longest = Some(item)
                        } 
                }
                longest
        }

        fn main(){
                let strings = ["Mango", "Banana", "Apple"];

                let biggest_item = find_biggest_item(&strings);

                match biggest_item{
                        Some(value)=> {
                               println!("Biggest item: {}", value); 
                        },
                        None => println!("Array is empty"),
                }
                
        }

# 20. Vectors

- A growable list type that stores elements contiguously in memory
- Vectors are growable and shrinkable at run time, which means you can modify their sizes at runtime by adding or removing elements. (Remember arrays in RUST ([T; N]) have a fixed size that's determined at compile time?)
- Supports random access , push, pop and other list like operations 

        fn main() {
                // This creates an empty vector , its type will be determined as soon as you start pushing data into it
                let mut v = Vec::new();   // if you want to explicitly mention the type : let mut v : Vec<i32> = Vec::new()

                // Push elements into the vector
                v.push(1);
                v.push(2);
                v.push(3);
                v.push(4);
                v.push(5);    // All this can also be implemented using a rust macro vec! let v = vec![1, 2, 3, 4, 5];

                for i in v{
                        println!("{}", i);
                }
        }

- Converting array to Vector

        let arr = [1, 2, 3 , 4, 5];

        let vec1 = arr.to_vec();
        let vec2 = Vec::from(arr);
        let vec3 = Vec::from([1, 2, 3 , 4, 5])
        let vec4 = Vec::from([10; 5]);

        //use debug trait , coz you know the drill for these data types 
        println!("{:?}", vec4)

#### Vectors under the hood
![alt text](image-3.png)

![alt text](image-4.png)

- **Vectors are move by default**

        let v = vec![1, 2, 3];
        println!("{}", v[0]);

- indexing in Rust for vectors 

        let v = vec!["Sun".to_string(), "Mon".to_string(), "Tue".to_string()];

        let s = &v[0]; // since strings doesn't allow copying then borrowing like this will be the best method

- **Safer Vector Indexing**

The safest way to access elements in a Vec is using the get and get_mut methods. They return an Option<&T> or Option<&mut T> respectively instead of panicking when faced with out of bounds indices 

        fn main(){
                let mut vec = vec![1,2,3];

                //Using get
                // val_ref is of type Option<&i32>
                let val_ref = vec.get(1);

                if let Some(val) = val_ref {
                        println!("Value : {}", val);
                }
                // Using get_mut
                // val_mut_ref is of type Option <&mut i32>

                let val_mut_ref = vec.get_mut(2);
                if let Some(val) = val_mut_ref {
                        // Dereferencing and modifying the value in place 

                        *val += 10;  // 13 will be the new value at index 2 
                }
                println!("{:?}", vec);
        }