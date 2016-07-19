# rstring

A resistor divider string calculator with preferred values


## Description

**rstring** is a command line program that finds the closest [preferred
value](https://en.wikipedia.org/wiki/Preferred_number) (E-series) resistors to
produce "tap" voltages from a resistor divider, given the total resistance and
the source voltage applied across the divider.

If no taps are given, the program simply finds the resistor that is closest to
the given total resistance.


## Features

* Arbitrary number of tap voltages
* Resistor options:
	* E6, E12, E24, E48, E96, E192
	* ideal resistance
* Cool ASCII graphic of a resistor string
* Output formats: 
	* CSV output
	* SPICE sub-circuit output
* Error and current analysis


## Compiling

This program was written in [Rust](https://www.rust-lang.org) with the
[Cargo](http://doc.crates.io/guide.html) environment.

### With Cargo
At the top level directory, run
```
cargo build --release
```
and the executable will be built as `target/release/rstring`

### With rustc
Alternatively, you can build it with just the compiler.

In the `src` directory, run
```
rustc main.rs -o rstring
```
and the executable will be built as `rstring`



## Examples
1. Usage
```
$ rstring
Usage: rstring [Opt. E Series] [Opt. Format] [Source Voltage] [Total Resistance] [Tap Voltages...]

	E Series options (defaults to E12 series if not given): 
		-e6, -e12, -e24, -e48, -e96, -e192, -ideal

	Format options (defaults to human readable if not given):
		-csv, -spice

	Description:
		Finds the closest preferred value resistors to produce "tap" voltages from a
		resistor divider, given the total resistance and the source voltage applied
		across the divider.

		If no taps are given, the program simply finds the resistor that is closest to
		the given total resistance.
```

2. Typical Use
```
$ rstring -e12 10 54321 1 1.5 5.4 2.6 3.9 8.1


 | 10 V 
 |
[ ]  1.0000e4 Ohms 
 |---- Tap 1: 8.1785e0 V (Ideal: 8.1 V, Error: 0.97%)
 |
[ ]  1.5000e4 Ohms 
 |---- Tap 2: 5.4463e0 V (Ideal: 5.4 V, Error: 0.86%)
 |
[ ]  6.8000e3 Ohms 
 |---- Tap 3: 4.2077e0 V (Ideal: 3.9 V, Error: 7.89%)
 |
[ ]  6.8000e3 Ohms 
 |---- Tap 4: 2.9690e0 V (Ideal: 2.6 V, Error: 14.19%)
 |
[ ]  6.8000e3 Ohms 
 |---- Tap 5: 1.7304e0 V (Ideal: 1.5 V, Error: 15.36%)
 |
[ ]  2.7000e3 Ohms 
 |---- Tap 6: 1.2386e0 V (Ideal: 1 V, Error: 23.86%)
 |
[ ]  6.8000e3 Ohms 
 |
 | 0 V

Current Consumption:	0.000182 Amperes
Total Resistance:	5.4900e4 Ohms (Ideal 5.4321e4 Ohms, Error: 1.07%)
```

3. SPICE Output
```
$ rstring -e12 -spice 10 54321 1 1.5 5.4 2.6 3.9 8.1
* Resistor Divider String
* Input Voltage: 10 V
* Total Resistance: 54900 Ohms

.SUBCKT RDIV VPOS VNEG TAP1 TAP2 TAP3 TAP4 TAP5 TAP6 
R1 VPOS TAP1 10000
R2 TAP1 TAP2 15000
R3 TAP2 TAP3 6800
R4 TAP3 TAP4 6800
R5 TAP4 TAP5 6800
R6 TAP5 TAP6 2700
R7 TAP6 VNEG 6800
.ENDS
```


4. CSV Output
```
$ rstring -e12 -csv 10 54321 1 1.5 5.4 2.6 3.9 8.1 | column -t -s,
SourceVoltage    10
Current          0.00018214936
                 
Item              Value          Ideal   Error%
TotalResistance  54900          54321   0.010658861
Tap1             8.178507       8.1     0.9692157
R1               10000                  
Tap2             5.446266       5.4     0.8567792
R2               15000                  
Tap3             4.2076507      3.9     7.888476
R3               6800                   
Tap4             2.9690351      2.6     14.193665
R4               6800                   
Tap5             1.7304195      1.5     15.361301
R5               6800                   
Tap6             1.2386162      1       23.861622
R6               2700                   

```


## Changelog
* 18-July-2016 Init
