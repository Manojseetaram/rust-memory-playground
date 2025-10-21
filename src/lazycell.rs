use std::cell::LazyCell;

pub fn lazy_operation() {
    let lazy_value = LazyCell::new(|| {
        println!("Initializing Value....");
        400
    });

    println!("Before access lazy_value");
    println!("Access First time : {:?}", *lazy_value);
    println!("Access Second time : {:?}", *lazy_value);
}
