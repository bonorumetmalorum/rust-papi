extern crate papi;

fn main() {
    let counters = &[papi::Counter::PAPI_L1_DCM, papi::Counter::PAPI_L2_DCM];
    let mut counters = unsafe {
        papi::CounterSet::new(counters)
    };
    let start = counters.read();
    let x = fib(14);
    let stop = counters.accum();
    println!("Computed fib(14) = {} with {} L1 misses, {} L2 misses",
             x, stop[0] - start[0], stop[1] - start[1]);
    counters.plot();
}

fn fib(n: isize) -> isize {
    if n < 2 { 1 }
    else { fib(n - 1) + fib(n - 2) }
}
