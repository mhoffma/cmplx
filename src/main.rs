mod cmplx;
mod fft;

use fft::fft;
use ndarray::prelude::*;
use num::Complex;
use plotters::prelude::*;
use std::f64::consts::PI;

fn plot(o: &Array1<Complex<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    let n = o.len();
    let max = o
        .fold(0.0, |x, y| y.im.max(x))
        .max(o.fold(0.0, |x, y| y.re.max(x)));
    let min = o
        .fold(0.0, |x, y| y.im.min(x))
        .min(o.fold(0.0, |x, y| y.re.min(x)));

    let root = BitMapBackend::new("1.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("FFT output {}", n), ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..(2.0 * PI), min..max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..n).map(|x| ((x as f64) * 2.0 * PI / (n as f64), o[x].re)),
            &RED,
        ))
        .unwrap()
        .label("y = o.im")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(
            (0..n).map(|x| ((x as f64) * 2.0 * PI / (n as f64), o[x].im)),
            &BLUE,
        ))
        .unwrap()
        .label("y = o.re")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello");
    let mut a = Array::from_iter((0..(1 << 12)).map(|_| Complex::new(0.0, 0.0)));
    a[1].re = 1.0;
    a[4].im = 0.25;
    let o = fft(&a);
    plot(&o)
}
