fn main() {
    // 1. Add suffix by moving and mutating
    // let first = String::from("Raj");

    // Will move `first`, hence `first` can not be used in `println!`
    // let full = add_suffix_mut(first);

    // let first_clone = first.clone();
    // let full = add_suffix_mut(first_clone);
    // println!("{full}, originally {first}"); // "Raj Patil, originally Raj"
    // =======================================================================================

    // 2. Add suffix by mutating but passing the reference
    // let mut first = String::from("Raj");
    // add_suffix_ref(&mut first);
    // println!("{first}"); // "Raj Patil"
    // =======================================================================================

    // 3. Dereferecing
    // let mut x = Box::new(1);
    // let a = *x;
    // *x += 1;
    // println!("x={}, a={}", x, a); // x=2, a=1
    // let r1 = &x; // r1 is a reference to x which is a reference to heap value of x
    // let b = **r1;

    // let r2 = &*x; // r2 is a reference to heap value of x
    // let c = *r2;
    // =======================================================================================

    // 4. Data should never be aliased and mutated at the same time.
    let mut v = vec![1, 2, 3];
    let num = &v[2];
    // v.push(4); // cannot mutate as immutable ref `num` is in use
    println!("Third element is {}", *num);
    // =======================================================================================
}

fn add_suffix_mut(mut name: String) -> String {
    name.push_str(" Patil");
    name
}

fn add_suffix_ref(name: &mut String) {
    name.push_str(" Patil");
}

fn take_ownership(v: [i32; 99999]) {
    // do something with v
    println!("{:?}", v);
}

fn print_half(i: i32) {
    println!("{}", i / 2);
}
