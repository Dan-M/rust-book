#[warn(unused_variables)]
fn main() {
    // create a new empty vector
    let mut e:Vec<i8> = Vec::new();
    // create and initialize vector
    let v = vec![1,2,3];

    println!("Empty: {:?}, initialized: {:?}", e, v);

    e.push(4);
    e.push(5);
    e.push(6);
    println!("...now filled: {:?}, initialized: {:?}", e, v);

    // reading with index
    // reference to the 3rd i32 element in the vec
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // reading with get
    let third: Option<&i32> = v.get(7);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no 7th element."),
    }

    // ...will panic because there is no 7th elem
    // let seventh: &i32 = &v[6];
    // println!("The sixth element is {seventh}");

    vec_manipulation();
    iterate_vec();
    different_types_with_enum();

}

fn vec_manipulation() {
    let mut v = vec![1,2,3];
    let _second = &v[1];
    v.push(4);
    // Compiler doesn't allow using second after push
    // println!("{}", second);
}

fn iterate_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // changing all elements within a vec
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // *i to dereference the value
        *i += 50;
    }
    println!("{:?}", v);
}

fn different_types_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row is {:?}", row);
}
