fn main() {
    let char_array1 = ['r', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array1) {
        println!("Given array is a palindrome.");
    } else {
        println!("Given array is not a palindrome")
    }

    let char_array2 = ['b', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array2) {
        println!("Given array is a palindrome.");
    } else {
        println!("Given array is not a palindrome")
    }
}

fn is_palindrome<T: PartialEq>(array: &[T]) -> bool {
    let mut i = 0;
    let mut j = array.len() - 1;

    if array.len() == 0 {
        return true;   // alway remembver base condition
    }

    while i < j {
        if array[i] != array[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
