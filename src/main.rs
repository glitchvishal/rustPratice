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
    loop_fun();
    while_loop_fn();
    while_loop();
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


//loop function that

fn loop_fun(){
    let mut x = 0;
    loop{
        x += 1;
        println!(" x = {}",x);

        if x == 5{
            println!("we get 5");
            break;
        }
    }
}

//loop with while

// fn while_loop_fn(){
//     let  number =0;
//     while number != 0 {
//         println!("{}",number);
//         number == 1;
//     }
//     println!("hey");
// }


fn while_loop_fn() {
    let mut number = 0;
    while number != 0 {
        println!("{}", number);
        number = 1; // Use the assignment operator to change the value
    }
    println!("hey");
}

fn while_loop(){
    let a = [10,20,55,68,98,32,41,66,75,48];
    let mut i = 0;

    while i <= 5{
        println!("This is the value of the index {:?} : {}",i ,a[i]);
        i += 1;
    }

}