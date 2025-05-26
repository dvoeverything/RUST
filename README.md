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




         
