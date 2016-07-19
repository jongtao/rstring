use std::env;

use e_series;



pub enum OutputType { HUMAN, CSV, SPICE, }



fn print_usage(args: &Vec<String>)
{
	let path: Vec<&str> = args[0].split("/").collect();
	let exe_name = &path[path.len()-1];

	println!("Usage: {} [Opt. E Series] [Opt. Format] [Source Voltage] [Total Resistance] \
			[Tap Voltages...]", exe_name);

	print!("\n");
	println!("\tE Series options (defaults to E12 series if not given): ");
	println!("\t\t-e6, -e12, -e24, -e48, -e96, -e192, -ideal");

	print!("\n");
	println!("\tFormat options (defaults to human readable if not given):");
	println!("\t\t-csv, -spice");

	print!("\n\t");
	println!("Description:");
	println!(
"\t\tFinds the closest preferred value resistors to produce \"tap\" voltages from a
\t\tresistor divider, given the total resistance and the source voltage applied
\t\tacross the divider.");
	print!("\n");
	println!(
"\t\tIf no taps are given, the program simply finds the resistor that is closest to
\t\tthe given total resistance.");
	print!("\n");

}



fn set_series(series_set: &mut bool, raw_series: Vec<f32>)
	-> Result<Vec<f32>, ()>
{
	if !*series_set {
		*series_set = true;
		Ok(raw_series)
	}
	else {
		println!("Format Error: More than one series flag!");
		Err(())
	}
}




fn set_output(output_set: &mut bool, output: OutputType)
	-> Result<OutputType, ()>
{
	if !*output_set {
		*output_set = true;
		Ok(output)
	}
	else {
		println!("Format Error: More than one output flag!");
		Err(())
	}
}



fn set_source(voltage_set: &mut bool, voltage_arg: &String) -> Result<f32, ()>
{
	if !*voltage_set {
		*voltage_set = true;
	}
	else {
		println!("INTERNAL ERROR (BAD): Source voltage set more than once!");
	}


	let voltage: f32 =
		match voltage_arg.parse() {
			Ok(number) => number,
			Err(_)=> {
				println!("Source voltage must be a number!");
				println!("\tReceived: {}", voltage_arg);
				return Err(());
			}
		}; 

	if !voltage.is_finite() {
		println!("Source voltage too extreme! Try something more sensible.");
		println!("\tReceived: {}", voltage_arg);
		println!("\tInterpreted as: {}", voltage);
		return Err(());
	}

	if voltage <= 0.0 {
			println!("Source voltage must be a larger than 0 V!");
			return Err(());
	}

	Ok(voltage)
}



fn set_resistance(resistance_set: &mut bool, resistance_arg: &String)
	-> Result<f32, ()>
{
	if !*resistance_set {
		*resistance_set = true;
	}
	else {
		println!("INTERNAL ERROR (BAD): Resistance set more than once!");
	}

	let resistance: f32 =
		match resistance_arg.parse() {
			Ok(number) => number,
			Err(_)=> {
				println!("Total resistance must be a number!");
				println!("\tReceived: {}", resistance_arg);
				return Err(());
			}
		}; 

	if !resistance.is_finite() {
		println!("Resistance too extreme! Try something more sensible.");
		println!("\tReceived: {}", resistance_arg);
		println!("\tInterpreted as: {}", resistance);
		return Err(());
	}

	if resistance <= 0.0 {
		println!("Total resistance must be a larger than 0 Ohms!");
		return Err(());
	}

	Ok(resistance)
}



fn set_voltage(source: f32, voltage_arg: &String) -> Result<f32, ()>
{
	let voltage: f32 =
		match voltage_arg.parse() {
			Ok(number) => number,
			Err(_)=> {
				println!("Source voltage must be a number!");
				println!("\tReceived: {}", voltage_arg);
				return Err(());
			}
		}; 

	if voltage <= 0.0 {
			println!("Tap voltage must be a larger than 0 V!");
			return Err(());
	}

	if !voltage.is_finite() {
		println!("Tap voltage too extreme! Try something more sensible.");
		println!("\tReceived: {}", voltage_arg);
		println!("\tInterpreted as: {}", voltage);
		return Err(());
	}

	if voltage >= source {
		println!("Tap voltage must be smaller than source voltage!");
		println!("\tReceived: {}", voltage);
		return Err(());
	}

	Ok(voltage)
}



pub fn read() -> Result<(OutputType, bool, Vec<f32>, Vec<f32>, f32), ()>
{
	// Get the arguements to examine
	let args: Vec<_> = env::args().collect();

	// Parsing state
	let mut series_set = false;
	let mut output_set = false;
	let mut source_set = false;
	let mut resistance_set = false;

	// input data
	let mut ideal = false; // whether to use ideal or E series
	let mut series = vec![];
	let mut out_file = OutputType::HUMAN;
	let mut source = 0.0;
	let mut voltages = vec![];
	let mut resistance = 0.0;

	if args.len() < 3 {
		print_usage(&args);
		return Err(());
	}
	else { // parse arguements
		for arg in &args[1..] {
			if source_set == false {
				match arg.as_ref() {
					"-e6" => series = try!(set_series(&mut series_set, e_series::e6())),
					"-e12" => series = try!(set_series(&mut series_set, e_series::e12())),
					"-e24" => series = try!(set_series(&mut series_set, e_series::e24())),
					"-e48" => series = try!(set_series(&mut series_set, e_series::e48())),
					"-e96" => series = try!(set_series(&mut series_set, e_series::e96())),
					"-e192" => series = try!(set_series(&mut series_set, e_series::e192())),
					"-ideal" =>
					{
						series = try!(set_series(&mut series_set, vec![]));
						ideal = true;
					}, // case "-ideal"
					"-csv" => out_file = try!(set_output(&mut output_set, OutputType::CSV)),
					"-spice" => out_file = try!(set_output(&mut output_set, OutputType::SPICE)),
					_ =>
					{
						// source_set = true if setting voltage succeeds
						source = try!(set_source(&mut source_set, arg));
						voltages.push(source);

						if series_set == false { // default series E12
							series = try!(set_series(&mut series_set, e_series::e12()));
						}

						if output_set == false { // default output is human readable
							out_file = try!(set_output(&mut output_set, OutputType::HUMAN));
						}
					}, // case _
				}; // match
			} // while source voltage is not set
			else if resistance_set == false{
				resistance = try!(set_resistance(&mut resistance_set, arg));
			}
			else { // source voltage and resistance are set
				voltages.push(try!(set_voltage(source, arg)));
			}
		} // for arg
	} // parse

	if !series_set || !output_set || !source_set || !resistance_set
	{
		print_usage(&args);
		return Err(());
	}

	Ok((out_file, ideal, series, voltages, resistance))
}

