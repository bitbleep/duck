use duck::{static_vec, Vec};

// statically allocate a vector of 1024
// unsigned 32-bit integers (ie. a total of 4096 bytes)
// with a default value of zero
static_vec!(my_numbers, u32, 1024, 0);

fn main() {
    let mut numbers = my_numbers::vec().expect("failed to get numbers vec");
    numbers.push(123);
    numbers.push(456);
    numbers.push(789);
    print(&numbers);
    while let Some(value) = numbers.pop() {
        println!("pop() => {:?}", value);
    }
    print(&numbers);
}

fn print(vec: &Vec<u32>) {
    println!("vec contains {} element(s)", vec.len());
    for (index, value) in vec.iter().enumerate() {
        println!("{}: {}", index, value);
    }
}
