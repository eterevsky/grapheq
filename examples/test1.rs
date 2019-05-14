use grapheq;
use image::Luma;

fn main() {
    let style: grapheq::Style<Luma<u8>> = grapheq::Style::new();
    // let buffer = grapheq::plot(
    //     &|x, y| x * x + y * y - 1.0,
    //     &style
    // );
    let buffer = grapheq::plot(
        &|x, y| y - (1./x).sin(),
        &|x, _y| (
            (1./x).cos() / (x*x),
            1.,
        ),
        &style
    );
    buffer.save("a.png").unwrap()
}
