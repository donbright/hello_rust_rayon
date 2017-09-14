# hello_rust_rayon

Hyper Simple example showing Parallell processing in the Rust language with the Rayon package
No complicated ideas, graphics, physics, external libs, etc, are required.
It only uses basic arithmetic, and compares doing so in parallell with doing
so in singlethreaded mode. It is educational for people who are wanting
to try this out in Rust without a bunch of messing around with details.

To setup:

get rust and cargo, per instructions here:

https://www.rust-lang.org/en-US/install.html

This will typically only take a few minutes and is relatively straightforward.

To run:

    git clone https://github.com/donbright/hello_rust_rayon
    cd hello_rust_rayon
    cargo run --release

Output:

    building simple vector
    vector:
    1 2 . . . 1048573 1048574 
    vector processed:
    42593083 1418407359 . . . 513297762 969231052 
    1048576 elements, parallell elapsed time Duration { secs: 2, nanos: 383588762 }

    vector:
    42593083 1418407359 . . . 513297762 969231052 
    vector processed:
    -2116691985 -1682685121 . . . 2047531687 -1259345260 
    1048576 elements,    single elapsed time Duration { secs: 4, nanos: 794079709 }
  
Why:

This is made for people who don't know anything about Rust, don't care about
the details, just want to write some simple code that will use multiple CPUs.

Top:

Running top, on linux/bsd, should show the program using more than 100% of CPU.

