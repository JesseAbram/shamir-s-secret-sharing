use rand::Rng;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let polynomial: Vec<u16> = construct_polynomial();
    // let k = 3;
    // let n = 6;

    let mut points: Vec<Point> = Vec::new();
    for x in 0..6 {
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

fn construct_polynomial() -> Vec<u16>{
    let mut rng = rand::thread_rng();
    let a0: u16 = rng.gen();
    let a1: u16 = rng.gen();
    let a2: u16 = rng.gen();

    vec!(a0, a1, a2)

    
}

fn construct_points(polynomial: &Vec<u16>, x: f32) -> f32 {
    //TODO build this better without hardcoding using the threshold k and number n
    polynomial[0] as f32 + (polynomial[1] as f32 * x ) + (polynomial[2] as f32 * (x.powi(2)))
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
