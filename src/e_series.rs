use std::f32;



const RAW_E24: [f32; 24] = [1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4,
	2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6, 6.2, 6.8, 7.5, 8.2, 9.1];

const RAW_E192: [f32; 192] = [1.0, 1.01, 1.02, 1.04, 1.05, 1.06, 1.07, 1.09, 1.1,
	1.11, 1.13, 1.14, 1.15, 1.17, 1.18, 1.2, 1.21, 1.23, 1.24, 1.26, 1.27, 1.29,
	1.3, 1.32, 1.33, 1.35, 1.37, 1.38, 1.4, 1.42, 1.43, 1.45, 1.47, 1.49, 1.5,
	1.52, 1.54, 1.56, 1.58, 1.6, 1.62, 1.64, 1.65, 1.67, 1.69, 1.72, 1.74, 1.76,
	1.78, 1.8, 1.82, 1.84, 1.87, 1.89, 1.91, 1.93, 1.96, 1.98, 2.0, 2.03, 2.05,
	2.08, 2.1, 2.13, 2.15, 2.18, 2.21, 2.23, 2.26, 2.29, 2.32, 2.34, 2.37, 2.4,
	2.43, 2.46, 2.49, 2.52, 2.55, 2.58, 2.61, 2.64, 2.67, 2.71, 2.74, 2.77, 2.8,
	2.84, 2.87, 2.91, 2.94, 2.98, 3.01, 3.05, 3.09, 3.12, 3.16, 3.2, 3.24, 3.28,
	3.32, 3.36, 3.4, 3.44, 3.48, 3.52, 3.57, 3.61, 3.65, 3.7, 3.74, 3.79, 3.83,
	3.88, 3.92, 3.97, 4.02, 4.07, 4.12, 4.17, 4.22, 4.27, 4.32, 4.37, 4.42, 4.48,
	4.53, 4.59, 4.64, 4.7, 4.75, 4.81, 4.87, 4.93, 4.99, 5.05, 5.11, 5.17, 5.23,
	5.3, 5.36, 5.42, 5.49, 5.56, 5.62, 5.69, 5.76, 5.83, 5.9, 5.97, 6.04, 6.12,
	6.19, 6.26, 6.34, 6.42, 6.49, 6.57, 6.65, 6.73, 6.81, 6.9, 6.98, 7.06, 7.15,
	7.23, 7.32, 7.41, 7.5, 7.59, 7.68, 7.77, 7.87, 7.96, 8.06, 8.16, 8.25, 8.35,
	8.45, 8.56, 8.66, 8.76, 8.87, 8.98, 9.09, 9.2, 9.31, 9.42, 9.53, 9.65, 9.76,
	9.88];



// returns every nth value, where n is the divider
fn cut_series(source: &[f32], divider: i32) -> Vec<f32>
{
	let mut results = vec![];
	let divider = divider as usize;

	for (n, value) in source.iter().enumerate() {
		if n % divider == 0 {
			results.push(*value);
		}
	}

	results
} // cut_series()



pub fn e6() -> Vec<f32>
{
	cut_series(&RAW_E24, 4)
} // e6()



pub fn e12() -> Vec<f32>
{
	cut_series(&RAW_E24, 2)
} // e12()



pub fn e24() -> Vec<f32>
{
	RAW_E24.to_vec()
} // e24()



pub fn e48() -> Vec<f32>
{
	cut_series(&RAW_E192, 4)
} // e192()



pub fn e96() -> Vec<f32>
{
	cut_series(&RAW_E192, 2)
} // e192()



pub fn e192() -> Vec<f32>
{
	RAW_E192.to_vec()
} // e192()



// returns the two closest elements in the series
fn binary_search(key: &f32, series: &Vec<f32>) -> (f32, f32)
{
	if series.len() <= 2 {
		(series[0], series[1])
	}
	else { // continue recursion
		let cursor = series.len() / 2;
		//println!("len {}, cursor{}: {:?}", series.len(), series[cursor], series);

		if *key < series[cursor] {
			binary_search(&key, &series[..cursor+1].to_vec())
		}
		else { // series[cursor] <= key
			binary_search(&key, &series[cursor..].to_vec())
		}
	}
} // binary search



// returns the magnitude, where magnitude = 10^order
fn find_magnitude(number: f32) -> f32
{
	if number.is_infinite() {
		return number; // saturate result
	}
	else if number == 0.0 {
		return 0.0;
	}
	else if number.is_nan() {
		panic!{"Can't calculate magnitude of Nan!"}
	}

	let mut magnitude = 1.0;
	let mut tmp = number.abs(); // treat negative numbers as positive

	if number > 1.0 {
		while tmp >= 10.0 {
			tmp /= 10.0;
			magnitude *= 10.0;
		}
	}
	else if number < 1.0 {
		while tmp < 1.0 {
			tmp *= 10.0;
			magnitude /= 10.0;
		}
	}

	magnitude
}



pub fn find_closest(target: f32, raw_series: &Vec<f32>) -> Result<f32, ()>
{
	let mut series = vec![];

	if target <= 0.0 {
		return Ok(0.0);
	}

	let magnitude = find_magnitude(target);

	if magnitude == f32::INFINITY {
		return Err(());
	}
	else if magnitude == f32::NEG_INFINITY || magnitude <= 0.0 {
		return Ok(0.0);
	}

	// get proper magnitude
	for n in 0..raw_series.len()-1 {
		series.push(raw_series[n] * magnitude);
	}

	// get the possible values
	let (lower, upper) = binary_search(&target, &series);
	let lower_extreme = series[series.len()-1] / 10.0;
	let upper_extreme = series[0] * 10.0;

	// get ready to find the closest of all the possibilities
	let mut closest = lower_extreme;
	let mut best_distance = (lower_extreme - target).abs();
	let possibilities = [lower, upper, upper_extreme];

	// find closest
	for value in &possibilities
	{
		let distance = (value-target).abs();

		if distance < best_distance {
			closest = *value;
			best_distance = distance;
		}
	}

	Ok(closest)
}
