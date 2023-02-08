#![allow(dead_code, unused_labels)]


fn main() {

    //all 50 elements get value 2
    let ys: [i32;50] = [2;50];

    print_arr(&ys);

    //create empty slice
    let empty_slice: [u32;0] = [];

    assert_eq!(&empty_slice, &[]);
    assert_eq!(&empty_slice, &[][..]);

    //safely accessing arrays

    // get return Option element
    for i in 0..ys.len() + 1{
        match ys.get(i){
            Some(ys_val) => println!("{} with value {}", i, ys_val),
            None => println!("Overflow over {}", i),
        }
    }

}

//whole array can be used as a slice
fn print_arr (arr_slice: &[i32]){

    println!("Array length is {}", arr_slice.len());
/*
    for item in arr{
        println!("{item}");
    }
*/

}