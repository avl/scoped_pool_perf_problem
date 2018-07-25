#![feature(test)]

extern crate scoped_pool;
extern crate test;
use scoped_pool::Pool;
use test::{Bencher};

#[bench]
pub fn test_bench_alt(b: &mut Bencher) {
	let parallellism = 2;
	let mut pool = Pool::new(parallellism);
	let par = parallellism as usize;
	
	{

		let mut data = Vec::new();
		let data_size = 500_000;
		for i in 0..data_size {
			data.push(0);
		}

		let mut totvecs=Vec::<Vec<i32>>::new();
		for i in 0..par {
			let mut t = Vec::<i32>::with_capacity(data_size/par);
			totvecs.push(t);
		}
	    b.iter(move || {    	
	    	
	    	let dataref = &data;
			for i in 0..par {
				totvecs[i].clear();
			}
			{

				let mut totvecref=&mut totvecs;
				pool.scoped(move |scope| {
					//let fidx=0;
					let mut idx=0usize;
					{
						for sidx in totvecref.iter_mut() {						
					        scope.execute(move || {					        
					        	for item in &dataref[(idx*(data_size/par))..((idx+1)*(data_size/par))] {
					        		sidx.push(*item);
					        	}
					        								        
							});
							idx+=1;
					    }

					}
				});
			}
			let mut totvecref2=&mut totvecs;
			pool.scoped(move|scope|
				for sub in totvecref2.iter_mut() {
					//let sub:&Vec<i32> = sub;
			        scope.execute(move || {
						for sublot in sub {

			        		assert!(*sublot!=32);
				        	
			        	}
					});
		        }
			);
		});
	}
	
}


fn main() {
    println!("Hello, world!");
}
