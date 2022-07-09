
use std::f64::consts::PI;
use num::complex::Complex;
use std::vec;

use plotters::prelude::*;

use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::dft::*;

fn test_dft() {
    let xs: Vec<f64> = (0..8).map(|x| x as f64).collect();
    let ys: Vec<_> = xs.iter().map(|&x| Complex::from(x.sin())).collect();
    let cs = ys.dft();
    let zs = cs.idft();
    println!("{:?}\n", ys);
    println!("{:?}\n", cs);
    println!("{:?}\n", zs);
}
pub fn test_dft_plot() -> Result<(), Box<dyn std::error::Error>> {
    let N = 2_i32.pow(10);
    let (a,b) = (0., 1.);
    let dt = (b-a)/(N-1)as f64;
    let xs: Vec<f64> = (0..N).map(|i| a + dt*i as f64).collect();
    // let ys: Vec<_> = xs.iter().map(|&x| x.sin()).collect();
    let ycs: Vec<_> = xs.iter().map(|&x| Complex::from((50.*PI*x).sin() + (120.*PI*x).sin())).collect();
    let cs = ycs.dft();


    let root = BitMapBackend::new("data/test_dft_plot.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("test_dft", ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(a..b, -1.0f64..1.0f64)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..xs.len()).map(|i| (xs[i], cs[i].norm())),
            // (0..xs.len()).map(|i| (xs[i], cs[i].im.atan2(cs[i].re)/PI * cs[i].norm())),
            (&RED).stroke_width(2),
        ))?
        .label("dft")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn test_fft() {
    let xs: Vec<f64> = (0..8).map(|x| x as f64).collect();
    let ys: Vec<_> = xs.iter().map(|&x| Complex::from(x.sin())).collect();
    let cs = ys.fft();
    let zs = cs.ifft();
    println!("{:?}\n", ys);
    println!("{:?}\n", cs);
    println!("{:?}\n", zs);
}

fn bench_fft<F>(n: u32, fft: F) -> (Duration, Vec<Complex<f64>>)
where F: Fn(&Vec<Complex<f64>>)->Vec<Complex<f64>>
{
    let N = 2_i32.pow(n);
    let (a,b) = (0., 2.*PI);
    let dt = (b-a)/(N-1)as f64;
    let xs: Vec<f64> = (0..N).map(|i| a + dt*i as f64).collect();
    let ys: Vec<_> = xs.iter().map(|&x| Complex::from(x.sin())).collect();

    let st = Instant::now();
    let cs = fft(&ys);
    let dur = st.elapsed();
    println!("{}\n", dur.as_millis());
    // println!("{:?}\n", cs);
    return (dur, cs);
}

pub fn compare_naive_with_fft() {
    const n_q:usize = 16;
    const n_b:usize = 2;

    let mut n_range = [0u32; n_q];
    let mut fft_benches = [0u128; n_q];
    let mut dft_benches = [0u128; 9];
    let mut errors = [0.0f64; n_q];
    for i in 0..n_q {
        let n = i+n_b;
        n_range[i] = n as u32;
        let (dur1, cs1) = bench_fft(n as u32, FFT::fft);
        if i < 9 {
            let (dur2, cs2) = bench_fft(n as u32, DFT::dft);
            dft_benches[i] = dur2.as_micros();
            errors[i] = (0..cs1.len()).map(|k| (cs1[k].re-cs2[k].re).abs() + (cs1[k].im-cs2[k].im).abs())
                .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        }

        fft_benches[i] = dur1.as_micros();
        // dft_benches[i] = 0;
    }

    plot_fft_benches(&n_range, &dft_benches, &fft_benches, "naive ~ fft comparison", "./data/fft_bench.png").unwrap();
}

fn plot_fft_benches(d: &[u32], r1: &[u128], r2: &[u128], plot_header: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let s = file_name;
    let s1 = plot_header;
    // let root = BitMapBackend::new(&s, (640, 480)).into_drawing_area();
    let root = BitMapBackend::new(&s, (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(&s1, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(2u32..17u32, 0u128..10u128.pow(6))?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..r1.len()).map(|i| (d[i], r1[i])),
            (&RED).stroke_width(2),
        ))?
        .label("dft")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            (0..r2.len()).map(|i| (d[i], r2[i])),
            (&BLUE).stroke_width(2),
        ))?
        .label("fft")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}