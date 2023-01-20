fn main() {
    let test = newvec![1, 2, 3];
    println!("macro: {:?}", test);
}

#[macro_export]
macro_rules! newvec {
    ( $( $x:expr ),* ) => {
	{
	    let mut temp_vec = Vec::new();
	    $(
		temp_vec.push($x);
	    )*
		temp_vec
	}
    }
}
