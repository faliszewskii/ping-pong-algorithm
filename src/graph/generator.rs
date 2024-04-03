use rand::Rng;
use crate::graph::graph::Graph;
use crate::matrix::matrix::Matrix;


pub fn inverse_cantor(z : i32) -> (i32,i32){
    let n = ((-1.0+((1+8*z) as f64).sqrt())/2.0).floor() as i32;
    let y:i32 = z - (n*(n+1))/2;
    (n - y, y)
}
pub fn generate_ping_pong(size: i32, p: f64) -> Graph {
    let mut rng = rand::thread_rng();
    let mut m = Matrix::new(size as usize, size as usize);
    let n = (size-1)*size/2;
    (0..n)
        .map(inverse_cantor)
        .map(|(x,y)| (size - 1 - x, y))
        .map(|(x,y)| { if rng.gen_bool(p) { (y,x) } else { (x,y) } })
        .for_each(|(x,y)| { m[x as usize][y as usize] = 1 });
    Graph::new(m)
}