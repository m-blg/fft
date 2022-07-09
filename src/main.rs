#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]


mod fft_bench;
mod dft;
mod dft_demo;
mod noise_filter;

fn main() {
    println!("compiles");
    // test_dft();
    // test_fft();
    // bench_fft(11, FFT::fft);
    // fft_bench::compare_naive_with_fft();
    // dft_demo::test_anim();
    // fft_bench::test_dft_plot().unwrap();
    // dft_demo::demo2("demo", "data/dft_demo2.gif").unwrap();
    noise_filter::noise_filtering_test();
}
