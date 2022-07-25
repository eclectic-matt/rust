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


	//FIBONACCI
	println!("Fib({}) = {}",0,fibonacci(0));
	println!("Fib({}) = {}",1,fibonacci(1));
	println!("Fib({}) = {}",2,fibonacci(2));
	println!("Fib({}) = {}",3,fibonacci(3));


	//TWELVE DAYS
	println!("");
	println!("**THE TWELVE DAYS OF CHRISTMAS**");
	for index in 1..=12 {
		twelve_days_of_christmas(index);	
	}
}

fn farenheit_to_celsius(farenheit: i32) -> i32 {
	//(32°F − 32) × 5/9 = 0°C
	let farenheit = farenheit - 32;
	let farenheit = 5 * farenheit;
	let farenheit = farenheit / 9;
	return farenheit;

	//NOT SURE WHY THIS ONE-LINER DOESN'T WORK YET
	//let celsius = (5 / 9) * (farenheit - 32);
	//return celsius;
}

fn fibonacci(num: i32) -> i32 {

	//JUST RETURN NUM FOR 0,1
	if num == 0 {
		return num;
	} else if num == 1 {
		return num;
	}

	//let firstprev = 1;
	//let secondprev = 1;

	let mut total: i32 = 0;

	for _index in 2..num {
		total += num;
	}

	return total;

	//DEFINE ARRAY OF FIB NUMS, DEFAULT TO 0
	//let fibs: [i32; num];
	//SET INDEX 1 = 1;
	//fibs[1] = 1;
	//let total = 0;

	//for index in 1..num {
	//	total += fibs[index];
	//}

	//1, 2, 3, 4, 5, 6
	//0, 1, 1, 2, 3, 5
}

//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
//taking advantage of the repetition in the song.
fn twelve_days_of_christmas(day: u32) {

	println!("");
	print!("On the ");
	match day {
		1 => print!("first"),
		2 => print!("second"),
		3 => print!("third"),
		4 => print!("fourth"),
		5 => print!("fifth"),
		6 => print!("sixth"),
		7 => print!("seventh"),
		8 => print!("eighth"),
		9 => print!("ninth"),
		10 => print!("tenth"),
		11 => print!("eleventh"),
		12 => print!("twelfth"),
		_ => print!("remaining...")
	}
	print!(" day of Christmas, my true love sent to me:");
	println!("");
	
	for index in (1..=day).rev() {
		
		//IF WE'RE ON THE LAST DAY, ADD "and"
		if index == 1 {
			print!("and ")
		}
		//PRINT THIS DAY'S GIFT
		day_of_christmas(index);
	}
}

//PRINT THE GIFT FOR THE CURRENT DAY
fn day_of_christmas(day: u32) {

	//JUST MATCH TO THE APPROPRIATE STRING
	match day {
		1 => println!("a partridge in a pear tree"),
		2 => println!("Two turtledoves"),
		3 => println!("Three French hens"),
		4 => println!("Four calling birds"),
		5 => println!("FIVE GOLDEN RINGS"),
		6 => println!("Six geese a-laying"),
		7 => println!("Seven swams a-swimming"),
		8 => println!("Eight maids a-milking"),
		9 => println!("Nine ladies dancing"),
		10 => println!("Ten lords a-leaping"),
		11 => println!("Eleven pipers piping"),
		12 => println!("Twelve drummers drumming"),
		_ => println!("Well that's not very Christmassy, is it?")
	}
}