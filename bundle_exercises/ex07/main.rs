fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

fn slice_sum(slice: &[i32]) -> i32 {
    let mut result: i32 = 0;
    for nbr in slice {
        result = result + nbr;
    }
    return result;
}

fn print_modulo(n1: i32, n2: i32) {
    let modulo = n1 % n2;
    println!("{}", modulo);
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
}
