fn main() {
    
	//#-#-#-#-#-#-#-#-#-#-#-#-#
	// VARIABLE MUTABILITY
	//#-#-#-#-#-#-#-#-#-#-#-#-#

	//IMMUTABLE
	let x = 5;
	println!("The value of x is: {x}");		// 5

	//THROWS COMPILE ERROR (CANNOT MUTATE x)
	//x = 6;
	//println!("The value of x is: {x}");		// ERROR

	//DECLARE A NEW SCOPE
	{
		//USING let, WE CAN "SHADOW" THIS VARIABLE AT THIS SCOPE
		let x = 7;
		println!("The value of x is: {x}");		// 7
	}

	//THE SHADOW VARIABLE IS NOW OUT-OF-SCOPE
	println!("The value of x is: {x}");		// 5


	//#-#-#-#-#-#-#-#-#-#-#-#-#
	// NUMBER TYPES
	//#-#-#-#-#-#-#-#-#-#-#-#-#

	//SIGNED (+/-) INTEGERS
	//let eight_bit_signed: i8 = -128;								// TO +127
	//let sixteen_bit_signed: i16 = -32768;							// TO +32767
	//let thirty_two_bit_signed: i32 = -2147483648;					//TO +2,147,483,647
	//let sixty_four_bit_signed: i64 = -9223372036854775808;			//TO +9,223,372,036,854,775,807

	//UNSIGNED (+VE ONLY) INTEGERS
	//let eight_bit_unsigned: u8 = 128;
	//let sixteen_bit_unsigned: u16 = 32767;
	//let thirty_two_bit_unsigned: u32 = 2147483648;
	//let sixty_four_bit_unsigned: u64 = 9223372036854775808;

	//FLOATING POINT NUMBERS
	//let x = 2.0; // f64 (DEFAULT ON MODERN SYSTEMS, MORE PRECISION)
	//let y: f32 = 3.0; // f32


	//#-#-#-#-#-#-#-#-#-#-#-#-#
	// MATHEMATICAL OPERATIONS
	//#-#-#-#-#-#-#-#-#-#-#-#-#

	//ADD
	//let sum = 5 + 10;
	//SUBTRACT
	//let difference = 10 - 5;
	//MULTIPLY
	//let product = 44 * 12;
	//DIVISION
	//let quotient = 23 / 11;
	//let floored = 1 / 2; // FLOORS TO 0
	//let remainder = 45 % 23;


	//#-#-#-#-#-#-#-#-#-#-#-#-#
	// MIN-MAX VALUES
	//#-#-#-#-#-#-#-#-#-#-#-#-#

	println!("----------------------------");
	println!("Max values for numeric types");
	println!("----------------------------");

	//SIGNED INTEGERS
	println!("");
	println!("--SIGNED INTEGERS--");

	//i8
	let mut num = "i8";
	let i8_max = i8::MAX;
	let i8_min = i8::MIN;
	println!("The max value for {num} = {i8_min} to {i8_max}");

	//i16
	num = "i16";
	let i16_max = i16::MAX;
	let i16_min = i16::MIN;
	println!("The max value for {num} = {i16_min} to {i16_max}");

	//i32
	num = "i32";
	let i32_max = i32::MAX;
	let i32_min = i32::MIN;
	println!("The max value for {num} = {i32_min} to {i32_max}");

	//i64
	num = "i64";
	let i64_max = i64::MAX;
	let i64_min = i64::MIN;
	println!("The max value for {num} = {i64_min} to {i64_max}");

	//i128
	num = "i128";
	let i128_max = i128::MAX;
	let i128_min = i128::MIN;
	println!("The max value for {num} = {i128_min} to {i128_max}");

	//isize
	num = "isize";
	let isize_max = isize::MAX;
	let isize_min = isize::MIN;
	println!("The max value for {num} = {isize_min} to {isize_max}");

	//UNSIGNED
	println!("");
	println!("--UNSIGNED INTEGERS--");

	//u8
	num = "u8";
	let u8_max = u8::MAX;
	let u8_min = u8::MIN;
	println!("The max value for {num} = {u8_min} to {u8_max}");

	//u16
	num = "u16";
	let u16_max = u16::MAX;
	let u16_min = u16::MIN;
	println!("The max value for {num} = {u16_min} to {u16_max}");

	//u32
	num = "u32";
	let u32_max = u32::MAX;
	let u32_min = u32::MIN;
	println!("The max value for {num} = {u32_min} to {u32_max}");

	//u64
	num = "u64";
	let u64_max = u64::MAX;
	let u64_min = u64::MIN;
	println!("The max value for {num} = {u64_min} to {u64_max}");

	//u128
	num = "u128";
	let u128_max = u128::MAX;
	let u128_min = u128::MIN;
	println!("The max value for {num} = {u128_min} to {u128_max}");

	//usize
	num = "usize";
	let usize_max = usize::MAX;
	let usize_min = usize::MIN;
	println!("The max value for {num} = {usize_min} to {usize_max}");


	/*
	let names = vec!["Bob", "Frank", "Ferris"];
	let names = vec![i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, u256, usize];

	let types: array (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, u256, usize);
	for(type in types){
		let mut type_min = type::MIN;
		let mut type_max = type::MAX;
		println!("The max value for {type} = {type_min} to {type_max}");
	
	}*/


	//#-#-#-#-#-#-#-#-#-#-#-#-#
	// BOOLEANS (BOOL)
	//#-#-#-#-#-#-#-#-#-#-#-#-#
	
	//DEFINE BOOL
	let t = true;

	//WITH EXPLICIT TYPE ANNOTATION
	let f: bool = false;

	
	//#-#-#-#-#-#-#-#-#-#-#-#-#
	// CHARACTERS (CHAR)
	//#-#-#-#-#-#-#-#-#-#-#-#-

	let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

	//#-#-#-#-#-#-#-#-#-#-#-#-#
	// COMPOUND TYPES
	//#-#-#-#-#-#-#-#-#-#-#-#-#

	//TUPLES - FIXED SIZE, ELEMENTS CAN BE DIFFERENT DATA TYPES

	//IMPLICIT TYPES TUPLE
	let inferred_tuple = ("hello", 'C', 22.4);

	//DEFINE TUPLE (FIXED-SIZE ARRAY) WITH EXPLICIT TYPE ANNOTATIONS
	let tup: (i32, f64, u8) = (500, 6.4, 1);

	//DESTRUCTURE TUPLE INTO VARIABLES
	let (x, y, z) = tup;

	//ACCESS TUPLE ELEMENT BY INDEX (0-INDEXED)
	let five_hundred = tup.0;
	let six_point_four = tup.1;
	let one = x.2;

	//ARRAYS - FIXED SIZE, ELEMENTS MUST BE SAME DATA TYPE

	//DEFINE ARRAY (ALSO FIXED-LENGTH BUT MUST BE THE SAME TYPE)
	let arr = [1, 2, 3, 4, 5];

	//WITH EXPLICIT TYPE AND SIZE DECLARATION [i32 = type; 5 = arr_length]
	let arr_two: [i32; 5] = [1, 2, 3, 4, 5];

	//OR DEFINE BY FILLING WITH AN INITIAL VALUE
	let arr_three = [3; 5];	// [3, 3, 3, 3, 3]

	//ACCESS ELEMENTS (ALSO 0-INDEXED)
	let first_element = arr[0];
	let second_element = arr[1];
}
