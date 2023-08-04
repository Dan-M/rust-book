#[allow(unused_variables)]
fn main() {
    // How to write closures and assign it to a variable:
    let add_one_v1 = |x: u32| -> u32 { x + 1 };
    let add_one_v2 = |x| x + 1;

    // v3 and v4 need evaluation so that types can be assigned
    let v2_res = add_one_v2(4);

    println!("{v2_res}");
    // This doesn't work because the the first evaluation fixed it to i32
    // let v3_2_res = add_one_v2(1.5);

    //
    let vec_option = optional_vector();
    let vec_default: Vec<i32> = Vec::new();
    let vec = vec_option.unwrap_or_else(|| {
        let mut default = Vec::new();
        if true {
            default.push(1);
        }
        default
    });
    // ...easier - but without the closure
    // let vec = vec_option.unwrap_or(vec_default);

    dbg!(vec);
}

fn optional_vector() -> Option<Vec<i32>> {
    let vec_some = Some(vec![1]);
    let vec_none: Option<Vec<i32>> = None;

    if true {
        vec_some
    } else {
        vec_none
    }
}
