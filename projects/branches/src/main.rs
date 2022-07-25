fn main() {

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //IF BLOCKS
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	let number = 7;

	//SIMPLE IF BLOCK
	if number < 5 {
		//THIS IS AN "ARM"
		println!("Number less than 5!");
	} else {
		//AND ANOTHER "ARM" (LIKE THE -match STATEMENT)
		println!("Number greater than 5!");
	}

	//THIS WILL THROW AN ERROR, NO TYPE CONVERSION OR "TRUTHY" VALUES!
	//if number {
	//	println!("CONDITION MUST BE BOOLEAN");
	//}

	//THIS IS A SUITABLE CONDITION THOUGH (RETURNS A BOOLEAN)
	if number != 0 {
		println!("Number is not zero!");
	}

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //MULTIPLE ARMS
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	if number % 4 == 0 {

		println!("Number is divisble by 4!");
	} else if number % 3 == 0 {

		println!("Number is divisible by 3!");
	} else if number % 2 == 0 {

		println!("Number is divisible by 2!");
	} else {

		println!("Number is not divisble by 2, 3 or 4!");
	}

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //"IF" WITH "LET" STATEMENT
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	//IF CAN BE USED IN A "LET" STATEMENT (BECAUSE "IF" IS AN EXPRESION)
	let condition = true;
	let variable_number = if condition { 5 } else { 6 };
	println!("The value of this variable number is: {variable_number}!");

	//HOWEVER, JUST LIKE FUNCTION RETURN VALUES, THEY CANNOT BE MIXED LIKE THIS
	//variable_number = if condition { 3 } else { "four" };	//ERROR, INCOMPATIBLE TYPES
}
