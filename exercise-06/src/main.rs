fn main() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let s: &mut [i32] = &mut a[1..3];

    s[1] = 10;

    println!("{:?}", s);
}
