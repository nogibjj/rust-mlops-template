use tch::{Tensor, Kind};

fn main() {
    let tensor = Tensor::of_slice(&[1, 2, 3, 4]).reshape(&[2, 2]).to_kind(Kind::Float);
    println!("Tensor: {:?}", tensor);
}
