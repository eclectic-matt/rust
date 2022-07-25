fn main() {

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //INFINITE LOOP
    //#-#-#-#-#-#-#-#-#-#-#-#-#
	//loop {
	//	println!("Hello, world!");
	//}

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //BREAKING LOOPS
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	//USE A COUNTER FOR BREAKING THE LOOP
	let mut counter = 0;

	//SET RESULT TO THE OUTPUT OF THE LOOP
	let result = loop {

		//INCREMENT THE COUNTER
		counter += 1;

		//IF IT IS 10
		if counter == 10 {

			//BREAK OUT OF THE LOOP AND RETURN counter * 2 (= 20)
			break counter * 2;
		}
	};

	println!("The result is {result}");


    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //LOOP LABELS
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	//LOOP LABELS (MULTIPLE LOOPS, BREAK TO LABEL)
	let mut count = 0;

	//DEFINE A LOOP LABEL CALLED 'counting_up
	'counting_up: loop {

		println!("count = {count}");
		let mut remaining = 10;

		//START A NESTED LOOP
		loop {

			println!("remaining = {remaining}");
			if remaining == 9 {
				
				//A BREAK STATEMENT WILL EXIT THE NESTED LOOP ONLY
				break;
			}
			if count == 2 {

				//A BREAK STATEMENT ON A LOOP LABEL WILL EXIT THAT NAMED LOOP
				break 'counting_up;
			}
			remaining -= 1;
		}

		count += 1;
	}

	println!("End count = {count}");


	//#-#-#-#-#-#-#-#-#-#-#-#-#
    //WHILE LOOPS
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	let mut countdown = 3;

	while countdown != 0 {
		println!("{countdown}!");

		countdown -= 1;
	}

	println!("LIFT OFF!");

	//LOOP A COLLECTION WITH WHILE
	println!("Iterate array elements using while loop:");
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;
	while index < 5 {

		println!("The value is {}", a[index]);
		index += 1;
	}


	//#-#-#-#-#-#-#-#-#-#-#-#-#
    //FOR LOOPS
    //#-#-#-#-#-#-#-#-#-#-#-#-#
	println!("Iterate array elements using for loop:");

	//SAFER, DOES NOT RELY ON index < 5 BEING KEPT IN SYNC WITH ARRAY SIZE
	for element in a {

		println!("The value is: {element}");
	}

	//USING A RANGE AND THE "REVERSE" METHOD
	println!("A neater countdown:");
	//THIS USES A RANGE (1..4) TO GET 1,2,3 AND THEN .rev() REVERSES THIS TO 3,2,1
	//NOTE: FOR RANGE, THE UPPER VALUE IS <
	for number in (1..4).rev() {
		println!("{number}!");
	}
	println!("LIFT OFF!");


	//#-#-#-#-#-#-#-#-#-#-#-#-#
    //CHALLENGES
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	//CONVERT A NUMBER BETWEEN FARENHEIT AND CELSIUS
	let farenheit = 64;
	let celsius = farenheit_to_celsius(farenheit);
	println!("Converted {}F to celsius = {}C", farenheit, celsius);

	let farenheit = 14;
	let celsius = farenheit_to_celsius(farenheit);
	println!("Converted {}F to celsius = {}C", farenheit, celsius);

}

fn farenheit_to_celsius(farenheit: i32) -> i32 {
	//(32°F − 32) × 5/9 = 0°C
	let farenheit = farenheit - 32;
	let farenheit = 5 * farenheit;
	let farenheit = farenheit / 9;
	return farenheit;
	//let celsius = (5 / 9) * (farenheit - 32);
	//return celsius;
}
