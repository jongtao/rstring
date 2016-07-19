mod e_series;
mod rstring;
mod input;
mod printing;


fn main()
{
	let (out_file, ideal, series, mut voltages, resistance)	=
		match input::read() {
			Ok(a) => a,
			Err(()) => return
		};

	// get the data ready
	voltages.sort_by(|b, a| a.partial_cmp(b).unwrap());

	// solve!
	let resistors =
		match rstring::rstring(ideal, &series, resistance, &voltages) {
			Ok(a) => a,
			Err(_) =>
			{
				println!("Try something more sensible!");
				return;
			}
		};

	// analyze results
	let (real_resistance, current, real_voltages, voltage_err) =
		rstring::calc_errors(&resistors, voltages[0], &voltages[1..].to_vec());

	let resistance_err = (real_resistance - resistance) / resistance;

	// output
	match out_file {
		input::OutputType::HUMAN =>
			printing::human(&voltages, &real_voltages, &voltage_err, &resistors,
				current, real_resistance, resistance, resistance_err),
		input::OutputType::CSV =>
			printing::csv(&voltages, &real_voltages, &voltage_err, &resistors,
				current, real_resistance, resistance, resistance_err),
		input::OutputType::SPICE =>
			printing::spice(&voltages, &real_voltages, &resistors, real_resistance),
	}
}
