// src/bin/generics_tutorial.rs
//
//
fn take_2_anything<T:std::ops::Mul<Output = T>>(a:T,b:T)->T{
    a*b
}
fn main() {
    println!("Learning about generics!");

    let res = take_2_anything(3.0, 5.1);

    println!("{:.2?}", res);
}

