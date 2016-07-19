use e_series;



pub fn rstring(ideal: bool, series: &Vec<f32>,
	total_resistance: f32, taps_v: &Vec<f32>) -> Result<Vec<f32>, ()>
{
	let mut resistance_left = total_resistance;
	// first element is the source voltage!!! Don't fuck this up.
	let mut last_voltage = taps_v[0];
	let mut results = vec![];

	// for all voltages excluding the source voltage
	for voltage in &taps_v[1..]
	{
		// obtain tap resistance
		let lower_resistance = (voltage / last_voltage) * resistance_left;
		let tap_resistance = resistance_left - lower_resistance;

		let closest =
			if !ideal {
				match e_series::find_closest(tap_resistance, &series) {
					Ok(close) => close,
					Err(_) => return Err(())
				}
			}
			else {
				tap_resistance
			};
		//println!("ideal {}, close {}", tap_resistance, closest);

		// obtain remaining resistance
		resistance_left -= closest;
		last_voltage = *voltage;

		// store result
		results.push(closest);
	}

	let closest =
		if !ideal {
			match e_series::find_closest(resistance_left, &series) {
				Ok(close) => close,
				Err(_) => return Err(())
			}
		}
		else {
			resistance_left
		};

	results.push(closest);
	//println!("ideal {}, close {}", resistance_left, closest);

	Ok(results)
} // rstring()



pub fn calc_errors(resistances: &Vec<f32>, source: f32, ideal_taps: &Vec<f32>)
	-> (f32, f32, Vec<f32>, Vec<f32>)
{
	// calculate whole resistance
	let mut whole_resistance = 0.0;

	for n in resistances.iter() {
		whole_resistance += *n;	
	}

	// calculate actual voltages
	let current = source/ whole_resistance;

	let mut real_taps = vec![];
	let mut prev_voltage = source;

	// for every tap, obtain its voltage by subtracting the voltage drop across
	// the tap resistor from the previous tap voltage
	for resistance in resistances[..resistances.len()-1].iter() {
		let voltage = prev_voltage - (current * resistance);
		real_taps.push(voltage);

		prev_voltage = voltage;
	}

	// calculate errors of taps
	let mut tap_errors = vec![];

	// for every real tap,
	for (n, real_tap) in real_taps.iter().enumerate() {
		let ideal_tap = ideal_taps[n];
		let error = (real_tap - ideal_tap) / ideal_tap;
		tap_errors.push(error);
	}
	
	(whole_resistance, current, real_taps, tap_errors)
} // calc_errors
