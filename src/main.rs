extern crate rayon;
use std::time::Instant;
use rayon::prelude::*;

fn process_using_multithreads(input: &mut [i32]) {
    for i in 1..1000 {
      input.par_iter_mut()
           .for_each(|p| *p = (*p).wrapping_add( *p - *p/i ) );
    }
}

fn process_using_singlethread(input: &mut [i32]) {
    for i in 1..1000 {
      for p in 0..input.len() {
        input[p] = input[p].wrapping_add(input[p] - input[p]/i);
      }
   }
}

fn printvec( input: &[i32] ) {
    for x in 1..3 {
        print!("{} ", input[x]);
    }
    print!(". . . ");
    for x in input.len()-3..input.len()-1 {
        print!("{} ", input[x]);
    }
    print!("\n");
}

fn main() {
    let mut myvec1 = Vec::new();
    let mut myvec2 = Vec::new();
    println!("building simple test vectors...\n");
    for i in 0..1024*100 {
      myvec1.push(i);
      myvec2.push(i);
    }


    let starttime = Instant::now();
    println!("--parallell threads test--");
    println!("rayon::current_num_threads: {}",rayon::current_num_threads());
    println!("input vector:");
    printvec(&myvec1);

    process_using_multithreads( &mut myvec1 );
    println!("vector after processing:");
    printvec(&myvec1);
    println!("{} elements, elapsed time {:?}",myvec1.len(),starttime.elapsed());
    println!("");

    let starttime = Instant::now();
    println!("--single thread test--");
    println!("input vector:");
    printvec(&myvec2);
    process_using_singlethread( &mut myvec2 );
    println!("vector after processing:");
    printvec(&myvec2);
    println!("{} elements, elapsed time {:?}",myvec2.len(),starttime.elapsed());
}
