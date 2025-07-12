use std::io;

fn main() {
    let mut input = String::new();
    println!("Masukkan panjang deret Fibonacci:");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");

    let n: u32 = input.trim().parse().expect("Masukkan angka");

    let (mut a, mut b) = (0, 1);

    for _ in 0..n {
        print!("{} ", a);
        let next = a + b;
        a = b;
        b = next;
    }

    println!(); // newline
}
