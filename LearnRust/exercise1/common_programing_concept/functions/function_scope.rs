
const GLOBAL_CONST:u8 = 100;  // global variable will always be constant and also need to declare their type and variable name will be in uppercase always
fn main(){
    let outside_variable = 5;

    {
        let inside_variable = 10;
        println!("Inside variable: {}", inside_variable);
        println!("Outside variable: {}", outside_variable);
        

    }
   //rintln!("Inside variable: {}", inside_variable); this will throw an error "nnot find value `inside_variable` in this scope"
    println!("outside variable:{}", outside_variable);
    println!("{}", GLOBAL_CONST);
    print_value();

}
fn print_value(){
 // println!("{}",outside_variable)  this will trow an error "not found in this scope"
 println!("{}",GLOBAL_CONST);
}