pub fn human(taps_v: &Vec<f32>, real_taps: &Vec<f32>, tap_errors: &Vec<f32>,
	resistors: &Vec<f32>, current: f32, whole_resistance: f32,
	total_resistance: f32, resistance_err: f32)
{
	println!("\n");

	println!(" | {} V ", taps_v[0]);

	for (n, real_tap) in real_taps.iter().enumerate()
	{
		print!(" |\n[ ]  {:.4e} Ohms \n |", resistors[n]);
		println!("---- Tap {}: {:.4e} V (Ideal: {} V, Error: {:.2}%)",
			n+1, real_tap, taps_v[n+1], tap_errors[n] * 100.0);
	}
	print!(" |\n[ ]  {:.4e} Ohms \n |", resistors[resistors.len()-1]);
	println!("\n | 0 V");


	println!("\n");
	println!("Current Consumption:\t{:.6} Amperes", current);
	println!("Total Resistance:\t{:.4e} Ohms (Ideal {:.4e} Ohms, Error: {:.2}%)",
		whole_resistance, total_resistance, resistance_err * 100.0);
}




pub fn csv(taps_v: &Vec<f32>, real_taps: &Vec<f32>, tap_errors: &Vec<f32>,
	resistors: &Vec<f32>, current: f32, whole_resistance: f32,
	total_resistance: f32, resistance_err: f32)
{
	println!("SourceVoltage,{}", taps_v[0]);
	println!("Current,{}", current);
	println!(",");
	println!("Item, Value, Ideal, Error%");
	println!("TotalResistance,{},{},{}", whole_resistance, total_resistance,
		resistance_err);

	for (n, real_tap) in real_taps.iter().enumerate() {
		println!("Tap{},{},{},{}", n+1, real_tap, taps_v[n+1],
			tap_errors[n] * 100.0);
		println!("R{},{},,", n+1, resistors[n]);
	}
}



pub fn spice(taps_v: &Vec<f32>, real_taps: &Vec<f32>, resistors: &Vec<f32>,
	whole_resistance: f32)
{
	// Title card
	println!("* Resistor Divider String");
	println!("* Input Voltage: {} V", taps_v[0]);
	println!("* Total Resistance: {} Ohms", whole_resistance);
	println!("");
	
	// subcircuit declaration
	print!(".SUBCKT RDIV ");
	print!("VPOS VNEG ");
	for (n, _) in real_taps.iter().enumerate() {
		print!("TAP{} ", n+1);
	}
	println!("");

	if resistors.len() == 1 {
		println!("R1 VPOS VNEG {}", resistors[0]);
	}
	else if resistors.len() == 2 {
		println!("R1 VPOS TAP1 {}", resistors[0]);
		println!("R2 TAP1 VNEG {}", resistors[1]);
	}
	else {
		println!("R1 VPOS TAP1 {}", resistors[0]);
		for (n, r) in resistors[1..resistors.len()-1].iter().enumerate() {
			println!("R{} TAP{} TAP{} {}", n+2, n+1, n+2, r);
		}

		let end = resistors.len();
		println!("R{} TAP{} VNEG {}", end, end-1, resistors[end-1]);
	}


	println!(".ENDS");
}
