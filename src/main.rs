extern crate papi;
extern crate gnuplot;

use gnuplot::Figure;
use papi::Plotter;

fn main() {
    let counters = &[papi::Counter::PAPI_L1_DCM, papi::Counter::PAPI_L2_DCM];
    let mut counters = unsafe {
        papi::CounterSet::new(counters)
    };
    let mut plotter = Plotter::new(counters, ||{fib(16);}, Figure::new());
    plotter.plot();
}

fn fib(n: isize) -> isize {
    if n < 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}
