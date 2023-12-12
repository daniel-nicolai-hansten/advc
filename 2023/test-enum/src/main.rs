fn main() {
    let testvar = Structs::Struct1(TestStruct1 {a: 2, b: 2});
    match testvar {
        Structs::Struct0(var) => {
            println!("{var:?}")
        }
        Structs::Struct1(var) => println!("match on 1, {var:?}"),
        Structs::Struct2(_) => println!("match on 2"),
        
    }
}

enum Structs {
    Struct0(TestStruct0),
    Struct1(TestStruct1),
    Struct2(TestStruct2),
}
#[derive(Debug)]
struct TestStruct0 {
    a: i32,
    b: usize,
}
#[derive(Debug)]
struct TestStruct1 {
    a: i32,
    b: usize,
}
struct TestStruct2 {
    a: i32,
    b: usize,
}