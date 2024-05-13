use average_pool_benchmark::Matrix;
fn main() {
    let m = Matrix::random(1000, 4);
    let m = m.average_pool(2, 2);
    println!("{:?}", m);
}
