#![feature(test)]
extern crate scoped_pool;
extern crate test;
use scoped_pool::Pool;
use test::{Bencher};


/// This is a minimized program exhibiting a performance problem
/// Why is this program twice as fast, when the number of threads is set to 1 instead of 2?
#[bench]
pub fn test_bench_alt(b: &mut Bencher) {
	let parallellism = 1;
	let data_size = 500_000;		
	
	let mut pool = Pool::new(parallellism);
	
	{
		let mut data = Vec::new();
		for _ in 0..data_size {
			data.push(0);
		}

		let mut output_data=Vec::<Vec<i32>>::new();
		for _ in 0..parallellism {
			let mut t = Vec::<i32>::with_capacity(data_size/parallellism);
			output_data.push(t);
		}
	    b.iter(move || {    	
	    	
	    	for i in 0..parallellism {
				output_data[i].clear();
			}
			{

				let mut output_data_ref=&mut output_data;
                let data_ref = &data;						
				pool.scoped(move |scope| {
				
					for (idx,output_data_bucket) in output_data_ref.iter_mut().enumerate() {						
				        scope.execute(move || {					        
				        	for item in &data_ref[(idx*(data_size/parallellism))..((idx+1)*(data_size/parallellism))] { //Yes, this is a logic bug when parallellism does not evenely divide data_size. I could use "chunks" to avoid this, but I wanted to keep this simple for this analysis.
				        		output_data_bucket.push(*item);
				        	}
				        								        
						});
				    }

					
				});
			}
			let mut output_data_ref=&mut output_data;
			pool.scoped(move|scope|
				for sub in output_data_ref.iter_mut() {

			        scope.execute(move || {
						for sublot in sub {

			        		assert!(*sublot!=42);
				        	
			        	}
					});
		        }
			);
		});
	}
	
}


fn main() {
}

