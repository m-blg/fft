

use std::f64::consts::PI;
use num::complex::Complex;

use plotters::prelude::*;

use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io::{self, Write};

use crate::dft::*;

fn test_anim_frame(plot_header: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let s = file_name;
    let s1 = plot_header;
    // let root = BitMapBackend::new(&s, (640, 480)).into_drawing_area();
    let root = BitMapBackend::gif(&s, (800, 600), 100)?.into_drawing_area();
    for i in 0..10 {
        print!(".");
        io::stdout().flush().unwrap();
        // sleep(Duration::from_millis(500));

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(&s1, ("sans-serif", 50).into_font())
            .margin(15)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1.0f64..1.0f64, -1.0f64..1.0f64)?;

        chart.configure_mesh().draw()?;

        let t = 0.1*i as f64;
        let val = (2.*PI*Complex::i()*t).exp();
        let line = [(0.,0.), (val.re,val.im)];
        chart
            .draw_series(LineSeries::new(
                line.into_iter(),
                (&BLUE).stroke_width(2),
            ))?
            .label("fft")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
    }
    Ok(())
}

pub fn test_anim() {
    test_anim_frame("anim", "data/test_anim.gif").unwrap();
}


pub fn demo(plot_header: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    let N = 2u32.pow(6);
    let ts: Vec<_> = (0..N).map(|i| i as f64 / N as f64).collect();
    let f: Vec<_> = ts.into_iter().map(|t| Complex::cis(2.*PI*t) + 0.1*Complex::cis(2.*PI * 6.*t)).collect();

    let f_hat = f.fft();
    let mut points = vec![Complex::from(0.); f_hat.len()];


    let s = file_name;
    let s1 = plot_header;
    // let root = BitMapBackend::new(&s, (640, 480)).into_drawing_area();
    let root = BitMapBackend::gif(&s, (800, 800), 100)?.into_drawing_area();
    for i in 0..N {
        print!(".");
        io::stdout().flush().unwrap();
        // sleep(Duration::from_millis(500));

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(&s1, ("sans-serif", 50).into_font())
            .margin(15)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-2.0f64..2.0f64, -2.0f64..2.0f64)?;

        chart.configure_mesh().draw()?;

        let t = i as f64;
        let w = Complex::cis(2.*PI/N as f64);
        points[0] = f_hat[0];
        for l in 1..points.len() {
            points[l] = points[l-1] + f_hat[l]*w.powf(l as f64 * t);
        }

        chart
            .draw_series(LineSeries::new(
                (0..f.len()).map(|i| (f[i].re, f[i].im)).chain([(f[0].re, f[0].im)].into_iter()),
                (&BLUE).stroke_width(2),
            ))?;

        chart
            .draw_series(LineSeries::new(
                (0..points.len()).map(|i| (points[i].re, points[i].im)),
                (&RED).stroke_width(1),
            ))?;

        // chart
        //     .configure_series_labels()
        //     .background_style(&WHITE.mix(0.8))
        //     .border_style(&BLACK)
        //     .draw()?;

        root.present()?;
    }
    Ok(())
}

fn sample<F, T>(f: F, from: f64, to: f64, n: usize) -> Vec<T> 
where F: Fn(f64)->T 
{
    let dt = (to - from)/(n-1) as f64;
    (0..n).map(|i| f(from + i as f64 * dt)).collect()
}

pub fn demo2(plot_header: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    let N = 2u32.pow(6);
    let mut f = sample(|t| Complex::new(t, t+1.), -1., 0., (N/3) as usize);
    f.extend(sample(|t| Complex::new(t, -t+1.), 0., 1., (N/3) as usize));
    let len = f.len();
    f.extend(sample(|t| Complex::new(t, 0.), 1., -1., (N - len as u32) as usize));


    let f_hat = f.fft();
    let mut points = vec![Complex::from(0.); f_hat.len()];


    let s = file_name;
    let s1 = plot_header;
    // let root = BitMapBackend::new(&s, (640, 480)).into_drawing_area();
    let root = BitMapBackend::gif(&s, (800, 800), 100)?.into_drawing_area();
    for i in 0..N {
        print!(".");
        io::stdout().flush().unwrap();
        // sleep(Duration::from_millis(500));

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(&s1, ("sans-serif", 50).into_font())
            .margin(15)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1.1f64..1.1f64, -0.2f64..1.1f64)?;

        chart.configure_mesh().draw()?;

        let t = i as f64;
        let w = Complex::cis(2.*PI/N as f64);
        points[0] = f_hat[0];
        for l in 1..points.len() {
            points[l] = points[l-1] + f_hat[l]*w.powf(l as f64 * t);
        }

        chart
            .draw_series(LineSeries::new(
                (0..f.len()).map(|i| (f[i].re, f[i].im)),
                (&BLUE).stroke_width(2),
            ))?;

        chart
            .draw_series(LineSeries::new(
                (0..points.len()).map(|i| (points[i].re, points[i].im)),
                (&RED).stroke_width(1),
            ))?;

        // chart
        //     .configure_series_labels()
        //     .background_style(&WHITE.mix(0.8))
        //     .border_style(&BLACK)
        //     .draw()?;

        root.present()?;
    }
    Ok(())
}