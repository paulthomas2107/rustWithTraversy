pub fn run() {
    // let immutable as is
    let name = "Paul";
    // let with 'mut' is mutable
    let mut age = 37;
    age = age + 1;
    println!("Name is {} age {}", name, age);

    // Const
    const ID: i32 = 001;
    println!("ID {}", ID);

    // Assign multiple vars
    let (name, age, grade) = ("Paul", 44, "A+");
    println!("name:{}, age:{}, grade:{}", name, age, grade);
}
