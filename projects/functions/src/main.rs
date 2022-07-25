fn main() {

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //FUNCTIONS
    //#-#-#-#-#-#-#-#-#-#-#-#-#

	//MAIN EXECUTED FIRST, IN ORDER
    println!("Hello, world!");

	//CALL ANOTHER FUNCTION
    another_function();

    //PASSING A PARAMETER
    squared(3);

    //PASSING MULTIPLE PARAMETERS
    print_labelled_measurement(5,'L');

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //STATEMENTS AND EXPRESSIONS
    //#-#-#-#-#-#-#-#-#-#-#-#-#

    //THIS IS A STATEMENT (PERFORM AN ACTION BUT DO NOT RETURN A VALUE)
    //let a = 6;

    //THEREFORE, THIS WOULD THROW AN ERROR AS THE FIRST STATEMENT RETURNS NOTHING
    //let x = (let y = 6);

    //SIMILARLY, YOU CANNOT DO THIS IN RUST (UNLIKE C, RUBY)
    //let x = y = 6;

    //HOWEVER, YOU CAN CREATE A NEW SCOPE TO INCLUDE EXPRESSIONS WHICH CAN THEN BE ASSIGNED
    let y = {
        //THIS EXPRESSION IS EVALUATED
        let x = 3;
        //AND THIS ONE
        x + 1
        //AND THE BLOCK-SCOPING WILL RETURN THE EVALUATED RESULT
    };
    //SO AT THIS POINT, y = 4
    println!("The value of y is: {y}");

    //#-#-#-#-#-#-#-#-#-#-#-#-#
    //RETURN VALUES
    //#-#-#-#-#-#-#-#-#-#-#-#-#

    let five = five();
    println!("The value of five is: {five}");

    let six = six();
    println!("The value of six is: {six}");

    let seven = plus_one(six);
    println!("The value of seven is: {seven}");
}

//DOES NOT MATTER WHERE PLACED (ABOVE OR BEFORE CALL)
//AS LONG AS FUNCTION IS IN THE SAME SCOPE
fn another_function() {

    println!("Another function.");
}

//TYPES MUST BE DECLARED FOR FUNCTION PARAMETERS
fn squared(x: i32){

    println!("The value of x is: {x}");
    let squared: i32 = x * x;
    println!("The value of x squared is: {squared}");
}

//AND DIFFERENT TYPES CAN BE SPECIFIED AS FOLLOWS
fn print_labelled_measurement(value: i32, unit_label: char){

    println!("The measurement is: {value}{unit_label}");
}

//YOU MUST DECLARE THE TYPE OF THE RETURN VALUE
fn five() -> i32 {
    //HOWEVER, THE RETURN VALUE IS IMPLICITLY THE LAST EXPRESSION WITHIN A FUNCTION
    5
    //NOTE THE LACK OF SEMICOLON HERE, IF THIS WAS PRESENT, THIS WOULD ERROR
}

fn six() -> i32 {
    //OR YOU CAN FORCE THE RETURN VALUE USING THE "RETURN" KEYWORD (SEMICOLON OPTIONAL)
    return 6;
}

//THIS FUNCTION TAKES A PARAMETER (i32) AND RETURNS AN i32
fn plus_one(x: i32) -> i32 {
    //AGAIN, NO SEMICOLON HERE TO KEEP AS A FINAL EXPRESSION, RATHER THAN A STATEMENT
    x + 1
}

