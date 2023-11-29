use proyecto_matecomp::MS;

fn main() {
    let sr = MS::new(&|x| x.powf(x) -100., 10.0, 11.);

    for (i, s) in sr.into_iter().enumerate() {
        println!("{}: {:.60}", i, s);
    }
}
