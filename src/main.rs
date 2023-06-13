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

// fn main() {
//    //initialize structure
//    let emp1 = Employee{
//       company:String::from("TutorialsPoint"),
//       name:String::from("Mohtashim"),
//       age:50
//    };
//    let emp2 = Employee {
//       company:String::from("TutorialsPoint"),
//       name:String::from("Kannan"),
//       age:32
//    };
//    let elder = who_is_elder(emp1,emp2);
//    println!("elder is:");

//    //prints details of the elder employee
//    display(elder);
// }
// //accepts instances of employee structure and compares their age
// fn who_is_elder (emp1:Employee,emp2:Employee)->Employee {
//    if emp1.age>emp2.age {
//       return emp1;
//    } else {
//       return emp2;
//    }
// }
// //display name, comapny and age of the employee
// fn display( emp:Employee) {
//    println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
// }
// //declare a structure
// struct Employee {
//    name:String,
//    company:String,
//    age:u32
// }



//** Enum 
// **The `derive` attribute automatically creates the implementation
// ** required to make this `enum` printable with `fmt::Debug`.
// #[derive(Debug)]
// enum GenderCategory {
//    Male,Female
// }
// fn main() {
//    let male = GenderCategory::Male;
//    let female = GenderCategory::Female;

//    println!("{:?}",male);
//    println!("{:?}",female);
// }

//** Struc Enum combined 
// The `derive` attribute automatically creates the 
// implementation
// required to make this `enum` printable with 
// `fmt::Debug`.

// #[derive(Debug)]
// enum GenderCategory {
//    Male,Female
// }
// #[derive(Debug)]
// struct Person {
//    name:String,
//    gender:GenderCategory
// }

// fn main() {
//    let p1 = Person {
//       name:String::from("Vijay"),
//       gender:GenderCategory::Male
//    };
//    let p2 = Person {
//       name:String::from("Anyy"),
//       gender:GenderCategory::Female
//    };
//    println!("{:?}",p1);
//    println!("{:?}",p2);
// }

//** enums

// fn main() {
//    enum Day{
//       Monday,
//       Tuesday, 
//       Wednesday,
//       Thursday,
//       Friday,
//       Saturday,
//       Sunday
//    }

//    impl Day {
//       fn is_weekend(&self) -> bool {
//          match self {
//             Day::Saturday | Day:: Sunday => true,
//             _=> false
//          }
//       }
//    }

//    let today:Day = Day::Saturday;
//    match today {
//       Day::Monday => println!("Everyone hates Monday"),
//       Day::Tuesday => println!("Hectic day"),
//       Day::Wednesday => println!("Learning Day"),
//       Day::Thursday => println!("Pay Day"),
//       Day::Friday => println!("almost weekend"),
//       Day::Saturday => println!("weekend"),
//       Day::Sunday => println!("weekend"),
//    }
//    println!("IS today the weekend {}", today.is_weekend())
// }

// fn main() {
//    let result = is_even(3);
//    println!("{:?}",result);
//    println!("{:?}",is_even(30));
// }
// fn is_even(no:i32)->Option<bool> {
//    if no%2 == 0 {
//       Some(true)
//    } else {
//       None
//    }
// }

// // ** Vector 
// fn main() {
//    let vec1:Vec<i32> = Vec::new();

//    let mut vec2: Vec<i32> = vec![1,2,3,4];
//    vec2.push(5);

//    println!("1st : {}", vec2[0]);

//    let second: &i32 = &vec2[1];
//    match vec2.get(index: 1){
//       Some(second: &i32) => println!("2nd : {}",second),
//       None => println!("Mo 2nd value"),
//    }

//    for i: &mut i32 in &mut vec2 {
//       *i *= 2;
//    }

//    for i: &32 in &vec2 {
//       println!("{}", i);
//    }

//    println!("Vec Length {}", vec2.len());
//    println!("pop : {:?}", vec2.pop());

// }

//** Match Statement and Enum

// enum CarType {
//    Hatch,
//    Sedan,
//    SUV
// }
// fn print_size(car:CarType) {
//    match car {
//       CarType::Hatch => {
//          println!("Small sized car");
//       },
//       CarType::Sedan => {
//          println!("medium sized car");
//       },
//       CarType::SUV =>{
//          println!("Large sized Sports Utility car");
//       }
//    }
// }

// fn main(){
//    print_size(CarType::SUV);
//    print_size(CarType::Hatch);
//    print_size(CarType::Sedan);
// }

//** Match with Option

// fn main() {
//    match is_even(5) {
//       Some(data) => {
//          if data==true {
//             println!("Even no");
//          }
//       },
//       None => {
//          println!("not even");
//       }
//    }
// }
// fn is_even(no:i32)->Option<bool> {
//    if no%2 == 0 {
//       Some(true)
//    } else {
//       None
//    }
// }

//** Match & Enum with Data Type

// #[derive(Debug)]
// enum GenderCategory {
//    Name(String),UsrId(i32)
// }

// fn main() {
//    let p1 = GenderCategory::Name(String::from("vijay"));
//    let p2 = GenderCategory::UsrId(100);
//    println!("{:?}",p1);
//    println!("{:?}",p2);
//    match p1 {
//       GenderCategory::Name(val)=> {
//          println!("{}",val);
//       }
//       GenderCategory::UsrId(val)=> {
//          println!("{}",val);
//       }
//    }
// }

// ** Illustration: Defining a Module
// pub mod movies {
//    pub fn play(name:String) {
//       println!("Playing movie {}",name);
//    }
// }
// fn main(){
//    movies::play("Vijay and Kumar".to_string());
// }

//** Using Use Keyword
// pub mod movies {
//    pub fn play(name:String) {
//       println!("Playing movie {}",name);
//    }
// }
// use movies::play; // using use Keyword
// fn main(){
//    play("Vijay and Kumar ".to_string());
// }

//** Nested Modules

// pub mod movies {
//    pub mod english {
//       pub mod comedy {
//          pub fn play(name:String) {
//             println!("Playing comedy movie {}",name);
//          }
//       }
//    }
// }
// use movies::english::comedy::play; 
// // importing a public module

// fn main() {
//    // short path syntax
//    play("vijay and Kumar".to_string());
//    play("Hera Pheri".to_string());

//    //full path syntax
//    movies::english::comedy::play("Golmaal!".to_string());
// }

//** Illustration Creating A vector

// fn main() {
//    let mut v = Vec::new();
//    v.push(20);
//    v.push(30);
//    v.push(40);

//    println!("size of vector is :{}",v.len());
//    println!("{:?}",v);
// }

//**Creating a Vector - vec! Macro

// fn main() {
//    let v = vec![1,2,3];
//    println!("{:?}",v);
// }

//** Illustration: push()

// fn main() {
//    let mut v = Vec::new();
//    v.push(20);
//    v.push(30);
//    v.push(40);
   
//    println!("{:?}",v);
// }

//** Illustration: Remove()

// fn main() {
//    let mut v = vec![10,20,30];
//    v.remove(0);
//    println!("{:?}",v);
// }

// ** Illustration - contains()

// fn main() {
//    let v = vec![10,20,30];
//    if v.contains(&20) {
//       println!{"found 20"}
//    }
//    println!{"{:?}", v};
// }

// ** Illustration: len()

// fn main() {
//    let v = vec![1,2,3];
//    println!("size of vector is :{}",v.len());
// }

// fn main() {
//    let mut v = Vec::new();
//    v.push(20);
//    v.push(30);

//    println!("{:?}",v[0]);
// }

// fn main() {
//    let mut v = Vec::new();
//    v.push(20);
//    v.push(30);
//    v.push(40);
//    v.push(500);

//    for i in &v {
//       println!("{}",i);
//    }
//    println!("{:?}",v);
// }

// ** HashMap structure

// use std::collections::HashMap;
// fn main(){
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    println!("{:?}",stateCodes);
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    println!("size of map is {}",stateCodes.len());
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    println!("size of map is {}",stateCodes.len());
//    println!("{:?}",stateCodes);

//    match stateCodes.get(&"KL") {
//       Some(value)=> {
//          println!("Value for key KL is {}",value);
//       }
//       None => {
//          println!("nothing found");
//       }
//    }
// }

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");

//    for (key, val) in stateCodes.iter() {
//       println!("key: {} val: {}", key, val);
//    }
// }

// Illustration: contains_key()

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    stateCodes.insert("GJ","Gujarat");

//    if stateCodes.contains_key(&"GJ") {
//       println!("found key");
//    }
// }

// Illustration: remove()

// use std::collections::HashMap;
// fn main() {
//    let mut stateCodes = HashMap::new();
//    stateCodes.insert("KL","Kerala");
//    stateCodes.insert("MH","Maharashtra");
//    stateCodes.insert("GJ","Gujarat");

//    println!("length of the hashmap {}",stateCodes.len());
//    stateCodes.remove(&"GJ");
//    println!("length of the hashmap after remove() {}",stateCodes.len());
// }

//** Hash Set

// Illustration - insert()

// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();

//    names.insert("Mohtashim");
//    names.insert("Kannan");
//    names.insert("TutorialsPoint");
//    names.insert("Mohtashim");//duplicates not added

//    println!("{:?}",names);
// }

//length

// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Mohtashim");
//    names.insert("Kannan");
//    names.insert("TutorialsPoint");
//    println!("size of the set is {}",names.len());
// }


//** iterate in arbitrary order
// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Mohtashim");
//    names.insert("Kannan");
//    names.insert("TutorialsPoint");
//    names.insert("Mohtashim");

//    for name in names.iter() {
//       println!("{}",name);
//    }
// }

// **Illustration: get()

// use std::collections::HashSet;
// fn main() {
//    let mut names = HashSet::new();
//    names.insert("vijay");
//    names.insert("Kiran");
//    names.insert("Samrath");
//    names.insert("Samrath");

//    match names.get(&"Samrath"){
//       Some(value)=>{
//          println!("found {}",value);
//       }
//       None =>{
//          println!("not found");
//       }
//    }
//    println!("{:?}",names);
// }

// Illustration - contains()

// use std::collections::HashSet;

// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Vijay");
//    names.insert("Kiran");
//    names.insert("Samrath");

//    if names.contains(&"Samrath") {
//       println!("found name");
//    }  
// }

//** Remove 
// use std::collections::HashSet;

// fn main() {
//    let mut names = HashSet::new();
//    names.insert("Vijay");
//    names.insert("Kiran");
//    names.insert("Samrath");
//    println!("length of the Hashset: {}",names.len());
//    names.remove(&"Kiran");
//    println!("length of the Hashset after remove() : {}",names.len());
// }

//** Generic Collection 

// fn main(){
//     let mut vector_integer: Vec<i32> = vec![20,30];
//     vector_integer.push(40);
//     println!("{:?}", vector_integer)
// }

// ** Illustration: Generic Structure

// struct Data<T> {
//    value:T,
// }
// fn main() {
//    //generic type of i32
//    let t:Data<i32> = Data{value:350};
//    println!("value is :{} ",t.value);
//    //generic type of String
//    let t2:Data<String> = Data{value:"Tom".to_string()};
//    println!("value is :{} ",t2.value);
// }

// use std::fmt::Display;

// fn main(){
//    print_pro(10 as u8);
//    print_pro(20 as u16);
//    print_pro("Hello TutorialsPoint");
// }

// fn print_pro<T:Display>(t:T){
//    println!("Inside print_pro generic function:");
//    println!("{}",t);
// }

// use std::fmt::Display;

// fn main(){
//    print_pro(10 as u8);
//    print_pro(20 as u16);
//    print_pro("Hello TutorialsPoint");
// }

// fn print_pro<T:Display>(t:T){
//    println!("Inside print_pro generic function:");
//    println!("{}",t);
// }

// Illustration − Reading from the Console − stdin()

// fn main(){
//    let mut line = String::new();
//    println!("Enter your name :");
//    let b1 = std::io::stdin().read_line(&mut line).unwrap();
//    println!("Hello , {}", line);
//    println!("no of bytes read , {}", b1);
// }

// use std::io::Write;
// fn main() {
//    let b1 = std::io::stdout().write("Vinayak ".as_bytes()).unwrap();
//    let b2 = std::io::stdout().write(String::from("Samrath").as_bytes()).unwrap();
//    std::io::stdout().write(format!("\nbytes written {}",(b1+b2)).as_bytes()).unwrap();
// }

// fn main(){
//     let cmd_line = std::env::args();
//     println!("No of elements in arguments is :{}",cmd_line.len()); 
//     //print total number of values passed
//     for arg in cmd_line {
//        println!("[{}]",arg); //print all values passed 
//     //    as commandline arguments
//     }
// }

// fn main(){
//     let cmd_line = std::env::args();
//     println!("No of elements in arguments is 
//     :{}",cmd_line.len()); 
//     // total number of elements passed
 
//     let mut sum = 0;
//     let mut has_read_first_arg = false;
 
//     //iterate through all the arguments and calculate their sum
 
//     for arg in cmd_line {
//        if has_read_first_arg { //skip the first argument since it is the exe file name
//           sum += arg.parse::<i32>().unwrap();
//        }
//        has_read_first_arg = true; 
//        // set the flag to true to calculate sum for the subsequent arguments.
//     }
//     println!("sum is {}",sum);
// }

//** Iterators

//  fn main() {
//     let a = [10,20,30];

//     let mut iter = a.iter();
//     println!("{:?}", iter);

//     println!("{:?}",iter.next());
//     println!("{:?}",iter.next());
//     println!("{:?}",iter.next());
//     println!("{:?}",iter.next());
//  }

// using for in syntax for iteration

// fn main() {
//     let a = [10,20,30];

//     // let iter = a.iter();
//     for data in a.iter(){
//         println!("{}\t",data);
//     }
// }

//** Illustration:iter()

//  fn main() {
//     let names = vec!["vijay","Kiran", "Samrath"];
//     for name in names.iter() {
//         match name {
//             &"Kiran" => println!("I have matched with the name"),
//             _=> println!("Hello {}", name),
//         }
//     }
//     println!("{:?}", names);
//     // Reusing the collection after iteration
//  }

//**Illustration - into_iter()

//  fn main() {
//     let names = ["Vijay", "Kiran", "Samrath"];
//     for name in names.into_iter(){
//         match name{
//             "Kiran" => println!("I am here Kiran"),
//             _ => println!("Hello{}", name),
//         }
//     }
//     // cannot reuse the collection after iteration
//     //println!("{:?}",names); 
//     //Error:Cannot access after ownership move
//  }

// **Illustration - for and iter_mut()

// fn main() {
//     let mut names = vec!["Kannan", "Mohtashim", "Kiran"];
//     for name in names.iter_mut() {
//        match name {
//           &mut "Mohtashim" => println!("There is a rustacean among us!"),
//           _ => println!("Hello {}", name),
//        }
//     }
//     println!("{:?}",names);
//     //// reusing the collection after iteration
//  }

//** Illustration - for and iter_mut()

//  fn main() {
//     let mut names = vec!["Vijay", "Kiran","Samrath"];
//     for name in names.iter_mut(){
//         match name {
//             &mut "Kiran" => println!("So here are you Kiran"),
//             _=> println!("Hello {}", name),
//         }
//     }
//     println!("{:?}", names);
//         //// reusing the collection after iteration

//  }

// **Clousure

// syntax :
// let closure_function = |parameter| {
//     //logic
//  }

// fn main() {
//     let is_even = |x| {
//         x%2 == 0
//     };

//     let no = 10;
//     println!("{} is even ? {} ", no, is_even(no));
// }

fn main(){
    let val = 10;

    let numbers = |x| {
        x + val
    };
    println!("{}",numbers(2));
}

























