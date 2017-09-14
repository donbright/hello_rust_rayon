extern crate rayon;
use std::time::Instant;
use rayon::prelude::*;

fn process_using_multithreads(input: &mut [i32]) {
    for i in 1..1000 {
      input.par_iter_mut()
           .for_each(|p| *p += *p - *p/i);
    }
}

fn process_using_singlethread(input: &mut [i32]) {
    for i in 1..1000 {
      for p in 0..input.len() {
        input[p] += input[p] - input[p]/i;
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
    let mut myvec = Vec::new();
    println!("building simple vector");
    for i in 0..1024*1024 {
      myvec.push(i);
    }


    let starttime = Instant::now();
    println!("vector:");
    printvec(&myvec);
    process_using_multithreads( &mut myvec );
    println!("vector processed:");
    printvec(&myvec);
    println!("{} elements, parallell elapsed time {:?}\n",myvec.len(),starttime.elapsed());

    let starttime = Instant::now();
    println!("vector:");
    printvec(&myvec);
    process_using_singlethread( &mut myvec );
    println!("vector processed:");
    printvec(&myvec);
    println!("{} elements,    single elapsed time {:?}",myvec.len(),starttime.elapsed());
}
