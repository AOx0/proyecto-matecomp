use proyecto_matecomp::BM;

fn main() {
    let solver = BM::new(&|x| x.powf(x) - 100., (-4., 4.));
    //.with_iter(MAX_ITER)
    //.with_precision(0.000000);

    for (i, s) in solver.into_iter().enumerate() {
        println!("{}: {:.60}", i, s);
    }
}
