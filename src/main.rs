use rand::Rng;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let k = 3;
    let n = 6;
    let polynomial: Vec<u16> = construct_polynomial(&k);
    println!("{:?}", polynomial);

    let mut points: Vec<Point> = Vec::new();
    for x in 0..n {
        let result = construct_points(&polynomial, x as f32);
        let to_push = Point {
            x: x as f32, 
            y: result
             };
        points.push(to_push);
    }
    println!("{:?}", points);
    // reconstruct with only k of n
    let  finals = reconstruct_proofs(&points);
    println!("{:?}", finals)

}

fn construct_polynomial(k: &u64) -> Vec<u16>{
    let mut rng = rand::thread_rng();
    let mut polynomial = Vec::new();
    for _x in 0..*k {
        polynomial.push(rng.gen());
    }
    polynomial 
}

fn construct_points(polynomial: &Vec<u16>, x: f32) -> f32 {
    let polynomial_size = polynomial.len();
    let mut total = 0.0;
    for i in 0..polynomial_size {
        total += polynomial[i] as f32 * x.powi(i as i32);
    }

    total

    
}

fn reconstruct_proofs(points: &Vec<Point>) -> f32 {
   let n = points.len() as usize;

   let mut result = 0.0;
   for i in 0..n {
    let mut term = points[i].y;

    for j in 0..n {
        if i != j {
            assert!(points[i].x - points[j].x != 0.0);
            let denominator = points[i].x - points[j].x;
            let numerator = - points[j].x;
            term = term * (numerator/denominator);
        }
    }
    result += term
}
    // assert secret is recreated
    assert_eq!(points[0].y, result);

    result
}
