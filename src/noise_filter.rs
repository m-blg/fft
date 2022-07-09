
use std::f64::consts::PI;
use num::complex::Complex;
use std::vec;
use std::ops::Range;

use plotters::prelude::*;
use plotters::element::PointCollection;

use std::time::{Duration, Instant};
use std::thread::sleep;
use rand_distr::{Normal, Distribution};

use crate::dft::*;

pub fn noise_filtering_test(){
    let N = 2_i32.pow(10);
    let (a,b) = (0., 1.);
    let dt = (b-a)/(N-1)as f64;
    let xs: Vec<f64> = (0..N).map(|i| a + dt*i as f64).collect();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let noise:Vec<_> = xs.iter().map(|_| normal.sample(&mut rand::thread_rng())).collect();

    fn wave(x:f64) -> f64 {
        0.5*((2.*PI* 50.*x).sin() - (2.*PI* 120.*x).sin())
    }
    let f: Vec<_> = xs.iter().enumerate().map(|(i, &x)| Complex::from(wave(x) + noise[i]*0.1)).collect();
    let f_hat = f.dft();
    let f_hat_proc: Vec<_> = f_hat.iter().map(|&c| if c.norm() > 0.1 {c} else {Complex::from(0.)}).collect();
    let f_proc = f_hat_proc.ifft();

    // (0..xs.len()).map(|i| (xs[i], ys[i])),
    
    let freq: Vec<_> = (0..f_hat.len()/2).map(|l| l as f64).collect();
    let asp: Vec<_> = (0..f_hat.len()/2).map(|i| f_hat[i].norm()).collect();
    let asp_proc: Vec<_> = (0..f_hat_proc.len()/2).map(|i| f_hat_proc[i].norm()).collect();


    let f_re: Vec<_> = f.into_iter().map(|c| c.re).collect();
    let f_proc_re: Vec<_> = f_proc.into_iter().map(|c| c.re).collect();
        
    plot_graph(&xs, &f_re, a..b, -3.0f64..3.0f64, "Noisy Signal (0.5*(sin(2*PI * 50*x) - sin(2*PI * 120*x) + Gaussian noise)", "data/filter/1.png").unwrap();
    plot_graph(&freq, &asp, 0.0..512.0f64, 0.0..1.0f64, "First half of the Frequency spectrum", "data/filter/2.png").unwrap();
    plot_graph(&freq, &asp_proc, 0.0..512.0f64, 0.0..1.0f64, "First half of the Frequency spectrum processed", "data/filter/3.png").unwrap();
    plot_graph(&xs, &f_proc_re, a..b, -3.0..3.0f64, "Signal processed", "data/filter/4.png").unwrap();
    // plot_graph((0..xs.len()).map(|i| (xs[i], nycs[i].re)), a..b, -3.0f64..3.0f64);
}

fn plot_graph(xs: &[f64], ys: &[f64], range_d: Range<f64>, range_r: Range<f64>, caption: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> 
{
    let root = BitMapBackend::new(file_name, (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(range_d, range_r)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..xs.len()).map(|i| (xs[i], ys[i])),
            (&RED).stroke_width(2),
        ))?;
        // .label("dft")
        // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // chart
    //     .configure_series_labels()
    //     .background_style(&WHITE.mix(0.8))
    //     .border_style(&BLACK)
    //     .draw()?;

    Ok(())
}
