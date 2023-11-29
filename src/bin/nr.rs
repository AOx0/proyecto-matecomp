use proyecto_matecomp::SR;

fn main() {
    let sr = SR::new(
        &|x| x.powf(x) - 100.,
        &|x| x.powf(x) +  x.powf(x) * x.ln(),
        10.,
    );

    for (i, s) in sr.into_iter().enumerate() {
        println!("{}: {:.60}", i, s);
    }
}
