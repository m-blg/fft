use std::f64::consts::PI;
use num::complex::Complex;
use std::vec;
use plotters::prelude::*;

trait DFT {
    fn dft(&self) -> Vec::<Complex<f64>>;
    fn idft(&self) -> Vec::<Complex<f64>>;
}

impl DFT for Vec<Complex<f64>> {
    fn dft(&self) -> Vec::<Complex<f64>> {
        let n = self.len();
        let n_f64 = n as f64;
        let w = (-2.*PI*Complex::i()/n as f64).exp();
        let mut c = vec![Complex::from(0.); n];
        for l in 0..n {
            // c[l] = (0..n).map(|m| self[m] *
            //        (-2.*PI*Complex::i()*(l as f64)*(m as f64)/n_f64).exp()).sum::<Complex<f64>>() / n_f64;
            c[l] = (0..n).map(|m| self[m] *
                   w.powu((l*m) as u32)).sum::<Complex<f64>>() / n_f64;
        }
        return c;
    }

    fn idft(&self) -> Vec::<Complex<f64>> {
        let n = self.len();
        let w = (2.*PI*Complex::i()/n as f64).exp();
        let mut f = vec![Complex::from(0.); n];
        for m in 0..n {
            f[m] = (0..n).map(|l| self[l] *
                   w.powu((l*m) as u32)).sum::<Complex<f64>>();
        }
        return f;
    }
}

unsafe fn _fft<'a>(f: *const Complex<f64>, stride: usize, n: usize, c: *mut Complex<f64>, fi: Complex<f64>) -> &'a mut [Complex<f64>]
{
    if n == 1 {
        *c = *f;
        return std::slice::from_raw_parts_mut(c, n);
    }

    let w = (fi/n as f64).exp();

    let y_e = _fft(f, 2*stride, n/2, c, fi);
    let y_o = _fft(f.offset(stride as isize), 2*stride, n/2, c.offset((n/2) as isize), fi);
    // let y_e = std::slice::from_raw_parts(c, n/2);
    // let y_o = std::slice::from_raw_parts(c + n/2, n/2);
    let y = std::slice::from_raw_parts_mut(c, n);
    
    for m in 0..n/2 {
        let y_em = y_e[m];
        let y_om = y_o[m];
        let w_m = w.powu(m as u32);
        y[m] = y_em + w_m*y_om;
        y[m + n/2] = y_em - w_m*y_om;
    }

    return y;
}

trait FFT {
    fn fft(&self) -> Vec::<Complex<f64>>;
    fn ifft(&self) -> Vec::<Complex<f64>>;
}

impl FFT for Vec<Complex<f64>> {
    fn fft(&self) -> Vec::<Complex<f64>> {
        let fi = -2.*PI*Complex::i();

        let mut c = vec![Complex::from(0.); self.len()];
        unsafe {
            _fft(self.as_ptr(), 1, self.len(), c.as_mut_ptr(), fi);
        }

        let n = self.len();
        c.iter_mut().for_each(|z| *z /= n as f64);
        return c;
    }

    fn ifft(&self) -> Vec::<Complex<f64>> {
        let fi = 2.*PI*Complex::i();

        let mut c = vec![Complex::from(0.); self.len()];
        unsafe {
            _fft(self.as_ptr(), 1, self.len(), c.as_mut_ptr(), fi);
        }
        return c;
    }
}

fn test_dft() {
    let xs: Vec<f64> = (0..8).map(|x| x as f64).collect();
    let ys: Vec<_> = xs.iter().map(|&x| Complex::from(x.sin())).collect();
    let cs = ys.dft();
    let zs = cs.idft();
    println!("{:?}\n", ys);
    println!("{:?}\n", cs);
    println!("{:?}\n", zs);
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

fn main() {
    println!("compiles");
    test_dft();
    test_fft();
}
