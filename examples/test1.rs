use grapheq;
use image::Luma;

fn main() {
    let style: grapheq::Style<Luma<u8>> = grapheq::Style::new();
    let stretch = 100.0;
    let buffer = grapheq::plot(
        &|x, y| y - (stretch / x).sin(),
        &|x, _y| (
            stretch * (stretch/x).cos() / (x*x),
            1.,
        ),
        &style
    );
    println!("Writing a.png");
    buffer.save("a.png").unwrap()
}
