use grapheq;
use image::Luma;

fn main() {
    let style: grapheq::Style<Luma<u8>> = grapheq::Style::new();
    let style = style.set_xmin(-2.).set_xmax(2.).set_ymin(-2.).set_ymax(2.).set_width(0.002).set_image_width(1024).set_image_height(1024);
    let buffer = grapheq::plot(
        &|x, y| x*x + y*y - 1. + 0.1 * (10.*x + 10.*y).sin(),
        &|x, y| (
            2. * x + 0.1 * (10.*x + 10.*y).cos() * 10.,
            2. * y + 0.1 * (10.*x + 10.*y).cos() * 10.,
        ),
        &style
    );
    buffer.save("b.png").unwrap()

    // let buffer = grapheq::plot(
    //     &|x, y| x*x + y*y - 1. + 0.1 * (10. * y.atan2(x)).sin(),
    //     &|x, y| (
    //         2. * x + 0.1 * (10. * y.atan2(x)).cos() * 10. / (1. + y*y / x*x) * (-1. / x * x),
    //         2. * y + 0.1 * (10. * y.atan2(x)).cos() * 10. / (1. + y*y / x*x),
    //     ),
    //     &style
    // );
    // buffer.save("c.png").unwrap()
}
