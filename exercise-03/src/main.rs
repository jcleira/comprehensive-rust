fn main() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    a[1] = 10;
    println!("a = {:?}", a);

    let b: (bool, i32, f64) = (true, 10, 1.234);
    println!("b = {:?}", b);
}
