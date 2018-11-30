# hello_rust_rayon

This is an extremely simple example showing multicore aka multithreaded 
processing in the Rust language with the Rayon package. No complicated 
ideas, graphics, physics, external libraries, etc, are required.

The example runs a simple test by performing arithmetic on a large set 
of integers, once in parallell and once again in serial mode. It prints 
the time elapsed for each test.

To setup:

get rust and cargo, per instructions here:

https://www.rust-lang.org/en-US/install.html

This will typically only take a few minutes and is relatively straightforward.

To run:

    git clone https://github.com/donbright/hello_rust_rayon
    cd hello_rust_rayon
    cargo run --release

Output:

    building simple test vectors...

    --parallell threads test--
    rayon::current_num_threads: 4
    input vector:
    1 2 . . . 102397 102398 
    vector after processing:
    42593083 1418407359 . . . -2066691175 -1580502428 
    102400 elements, elapsed time 3.231979772s

    --single thread test--
    input vector:
    1 2 . . . 102397 102398 
    vector after processing:
    42593083 1418407359 . . . -2066691175 -1580502428 
    102400 elements, elapsed time 13.428865759s

Why:

This is made for people who don't know anything about Rust, don't care about
the details, just want to write some simple code that will use multiple CPUs.

Top:

Running top, on linux/bsd, should show the program using more than 100% of CPU.

