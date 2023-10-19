fn main() {
    println!("Hello, world!");
    println!("I am Vishal" );
    simple_function();
    second_function(100);
    third_function(214, "hello");
    exp();
    //data type in rust

    let no=2;
    print!("{}", no);

    let is_male = true;
    print!("{}", is_male);

    let chart = "abc";
    print!("{}", chart);
    tuple();
    array();
}

fn simple_function(){
    println!("Simple Function");
}
//function with single parameters
fn second_function(x : i32){
 println!("Second Function with Parameters : {}",x)
}
//function with multiple parameters
fn third_function(x : i32, y: &str){
    println!("third Function with Parameters : {}, and second Function with Parameters: {}",x,y)
}
//experession

fn exp(){
    let y= {
        let x = 5;  
        x + 1
    };
    println!("value of y is :{}",y);
}
fn tuple() {
    let mut tup = (1, "Vishal", true, 5.5);
    tup.0 = 85;             // Modify an element of the tuple

    println!("{:?}", tup);  // Print the entire tuple
    println!("{}", tup.2);   // Access and print a specific element

    println!("{:?}", tup);  // Print the modified tuple
}
fn array(){
    let arr=[1,2,3,4,5];
    print!("{:?}", arr);
    println!("{}",arr[1])
}







