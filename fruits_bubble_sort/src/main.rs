/// Bubble-sort with two nested `for` loops.
/// Outer loop runs `len - 1` passes.
/// Inner loop walks the shrinking unsorted zone and swaps neighbours.
fn bubble_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    if len < 2 {
        return;
    }

    for pass in 0..len - 1 {
        let mut swapped = false;

        // After each pass, the largest element is fixed at the end,
        // so we can stop one slot earlier next time: `len - pass - 1`.
        for i in 0..len - pass - 1 {
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
                swapped = true;
            }
        }

        // Optional optimisation: quit early if no swaps happened.
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut fruits = [
        ("Mango-us", 50, 80),
        ("Mango-uk", 50, 80),
        ("Orange",   19, 80),
        ("Blackberry", 20, 90),
        ("Blueberry", 17, 91),
        ("Blueberry", 17, 93),
        ("Blueberry", 21, 85),
    ];

    bubble_sort(&mut fruits);

    for t in fruits {
        println!("{:?}", t);
    }
}
