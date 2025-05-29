# RUST
# 1. hello_world_001
# 2. print_002
# 3. EMI_calculator
#### important notes
- {:.}2 - 2 decimal places 
- :#X - convert to Hexadecimal
- :#b or :b 
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












         
