// fn main() {
//     println!("Hello, world!");
// }

// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {secret_number}");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     let guess: u32 = guess.trim().parse().expect("Please type a number!");

//     println!("You guessed: {guess}");

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small!"),
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => println!("You win!"),
//   }
// }

// fn main() {
//    let result = 10;    // i32 by default
//    let age:u32 = 20;
//    let sum:i32 = 5-15;
//    let mark:isize = 10;
//    let count:usize = 30;
//    println!("result value is {}",result);
//    println!("sum is {} and age is {}",sum,age);
//    println!("mark is {} and count is {}",mark,count);
//    println!("count is {} and mark is {}",count,mark);
// }

// fn main() {
//    let fees = 25_000;
//    let salary:f64 = 35_000.00;
//    println!("fees is {} and salary is {}",fees,salary);
// }

// fn main() {
//    let fees = 25_000;
//    println!("fees is {} ",fees);
//    fees = 35_000;
//    println!("fees changed is {}",fees);
// }

// let mut variable_name = value;
// let mut variable_name:dataType = value;

// fn main() {
//    let mut fees:i32 = 25_000;
//    println!("fees is {} ",fees);
//    fees = 35_000;
//    println!("fees changed is {}",fees);
// }

// fn main() {
//    const USER_LIMIT:i32 = 100;    // Declare a integer constant
//    const PI:f32 = 3.14;           //Declare a float constant

//    println!("user limit is {}",USER_LIMIT);  //Display value of the constant
//    println!("pi value is {}",PI);            //Display value of the constant
// }

// fn main() {
//    let uname = "Mohtashim";
//    let uname = uname.len();
//    println!("name changed to integer : {}",uname);
// }

// fn main() {
//    let salary = 100.00;
//    let salary = 1.50 ; 
//    // reads first salary
//    println!("The value of salary is :{}",salary);
// }

// fn main() {
//    let company:&'static str = "rapid innovation";
//    let location:&'static str = "Noida";
//    println!("company is : {} location :{}",company,location);
// }

// **string

// fn main(){
//    let name1 = "Hello TutorialsPoint , 
//    Hello!".to_string();
//    println!("{}",name1);
// }

// **if else
// fn main(){
//    let num:i32 = 5;
//    if num > 0 {
//       println!("number is positive") ;
//    }
// }

// fn main() {
//    let num = 12;
//    if num % 2==0 {
//       println!("Even");
//    } else {
//       println!("Odd");
//    }
// }

// fn main() {
//    let num = 2 ;
//    if num > 0 {
//       println!("{} is positive",num);
//    } else if num < 0 {
//       println!("{} is negative",num);
//    } else {
//       println!("{} is neither positive nor negative",num) ;
//    }
// }

// ** Match statement
// fn main(){
//     let state_code = "MH";
//     let state = match state_code {
//        "MH" => {println!("Found match for MH"); "Maharashtra"},
//        "KL" => "Kerala",
//        "KA" => "Karnadaka",
//        "GA" => "Goa",
//        _ => "Unknown"
//     };
//     println!("State name is {}",state);
//  }

// fn main(){
//     for x in 1..11{ // 11 is not inclusive
//        if x==5 {
//           continue;
//        }
//        println!("x is {}",x);
//     }
//  }

//  fn main(){
//    let mut x = 0;
//    while x < 10{
//       x+=1;
//       println!("inside loop x value is {}",x);
//    }
//    println!("outside loop x value is {}",x);
// }

// fn main(){
//     //while true
 
//     let mut x = 0;
//     loop {
//        x+=1;
//        println!("x={}",x);
 
//        if x==15 {
//           break;
//        }
//     }
//  }

//  fn main() {

//     let mut count = 0;
 
//     for num in 0..21 {
//        if num % 2==0 {
//           continue;
//        }
//        count+=1;
//     }
//     println! (" The count of odd values between 0 and 20 is: {} ",count);
//     //outputs 10
//  }

//  fn main(){
//     //calling a function
//     fn_hello();
//  }
//  //Defining a function
//  fn fn_hello(){
//     println!("hello from function fn_hello ");
//  }

//  fn main(){
//     println!("pi value is {}",get_pi());
//  }
//  fn get_pi()->f64 {
//     22.0/7.0
//  }

//  fn main(){
//     let no:i32 = 5;
//     mutate_no_to_zero(no);
//     println!("The value of no is:{}",no);
//  }
 
//  fn mutate_no_to_zero(mut param_no: i32) {
//     param_no = param_no*0;
//     println!("param_no value is :{}",param_no);
//  }

// fn main(){
//     let name:String = String::from("TutorialsPoint");
//     display(name); 
//     //cannot access name after display
//  }
//  fn display(param_name:String){
//     println!("param_name value is :{}",param_name);
//  }

// fn main() {
//     let tuple:(i32,f64,u8) = (-325,4.9,22);
//     println!("{:?}",tuple);
//  }

//  fn main() {
//     let tuple:(i32,f64,u8) = (-325,4.9,22);
//     println!("integer is :{:?}",tuple.0);
//     println!("float is :{:?}",tuple.1);
//     println!("unsigned integer is :{:?}",tuple.2);
//  }

// fn main(){
//     let b:(i32,bool,f64) = (110,true,10.9);
//     print(b);
//  }
//  //pass the tuple as a parameter
 
//  fn print(x:(i32,bool,f64)){
//     println!("Inside print method");
//     println!("{:?}",x);
//  }

// fn main(){
//     let b:(i32,bool,f64) = (30,true,7.9);
//     print(b);
//  }
//  fn print(x:(i32,bool,f64)){
//     println!("Inside print method");
//     let (age,is_male,cgpa) = x; //assigns a tuple to 
//     // distinct variables
//     println!("Age is {} , isMale? {},cgpa is 
//     {}",age,is_male,cgpa);
// }

// fn main(){
//     let arr:[i32;4] = [10,20,30,40];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
// }

// fn main(){
//     let arr = [10,20,30,40];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
//  }

//  fn main() {
//     let arr:[i32;4] = [-1;4];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
//  }

//Array with for loop

//  fn main(){
//     let arr:[i32;4] = [10,20,30,40];
//     println!("array is {:?}",arr);
//     println!("array size is :{}",arr.len());
 
//     for index in 0..4 {
//        println!("index is: {} & value is : {}",index,arr[index]);
//     }
//  }

// Array with iter

// fn main(){
// let arr:[i32;4] = [10,20,30,40];
//    println!("array is {:?}",arr);
//    println!("array size is :{}",arr.len());

//    for val in arr.iter(){
//       println!("value is :{}",val);
//    }
// }

// Array with mutable Array

// fn main(){
//    let mut arr:[i32;4] = [10,20,30,40];
//    arr[1] = 0;
//    println!("{:?}",arr);
// }

// fn main() {
//    let arr = [10,20,30];
//    update(arr);
//    print!("Inside main {:?}",arr);
// }
// fn update(mut arr:[i32;3]){
//    for i in 0..3 {
//       arr[i] = 0;
//    }
//    println!("Inside update {:?}",arr);
// }

// fn main() {
//    let mut arr = [10,20,30];
//    update(&mut arr);
//    print!("Inside main {:?}",arr);
// }
// fn update(arr:&mut [i32;3]){
//    for i in 0..3 {
//       arr[i] = 0;
//    }
//    println!("Inside update {:?}",arr);
// }

// ** Function Borrow

// fn main(){
//    // a list of nos
//    let v = vec![10,20,30];
//    print_vector(v);
//    println!("{}",v[0]); // this line gives error
// }
// fn print_vector(x:Vec<i32>){
//    println!("Inside print_vector function {:?}",x);
// }

// fn main(){
//    // a list of nos
//    let v = vec![10,20,30];
//    print_vector(&v); // passing reference
//    println!("Printing the value from main() v[0]={}",v[0]);
// }
// fn print_vector(x:&Vec<i32>){
//    println!("Inside print_vector function {:?}",x);
// }

// fn add_one(e: &mut i32) {
//    *e+= 1;
// }
// fn main() {
//    let mut i = 3;
//    add_one(&mut i);
//    println!("{}", i);
// }

// let instance_name = Name_of_structure {
//    field1:value1,
//    field2:value2,
//    field3:value3
// }; 
// **NOTE the semicolon
// **Syntax: Accessing values in a structure
// **Use the dot notation to access value of a specific field.
// **instance_name.field1
// **Illustration
// struct Employee {
//    name:String,
//    company:String,
//    age:u32
// }
// fn main() {
//    let emp1 = Employee {
//       company:String::from("TutorialsPoint"),
//       name:String::from("Mohtashim"),
//       age:50
//    };
//    println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
// }

// fn who_is_elder (emp1:Employee,emp2:Employee)->Employee {
//    if emp1.age>emp2.age {
//       return emp1;
//    } else {
//       return emp2;
//    }
// }

fn main() {
   //initialize structure
   let emp1 = Employee{
      company:String::from("TutorialsPoint"),
      name:String::from("Mohtashim"),
      age:50
   };
   let emp2 = Employee {
      company:String::from("TutorialsPoint"),
      name:String::from("Kannan"),
      age:32
   };
   let elder = who_is_elder(emp1,emp2);
   println!("elder is:");

   //prints details of the elder employee
   display(elder);
}
//accepts instances of employee structure and compares their age
fn who_is_elder (emp1:Employee,emp2:Employee)->Employee {
   if emp1.age>emp2.age {
      return emp1;
   } else {
      return emp2;
   }
}
//display name, comapny and age of the employee
fn display( emp:Employee) {
   println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}
//declare a structure
struct Employee {
   name:String,
   company:String,
   age:u32
}

