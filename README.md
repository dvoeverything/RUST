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
        // Debug trait of the array which is used to print arrays in a concise and readable format
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















         
