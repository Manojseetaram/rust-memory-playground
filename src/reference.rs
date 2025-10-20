use std::pin::Pin;
struct S {
    text: String,
    slice: *const str,
}

impl S {
    fn new(data: String) -> Pin<Box<S>> {
        //Step 1 : temporary dummy self-reference
        let mut boxed = Box::pin(S {
            text: data,
            slice: "".as_ref() as *const str,
        });
        //Step 2 : set correct reference after pinning
        let data_ref: *const str = boxed.text.as_str();
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            let this = Pin::get_unchecked_mut(mut_ref);
            this.slice = data_ref;
        }
        boxed
    }
    fn show(self: Pin<&Self>) {
        println!("Data : {}", self.text);
        unsafe {
            println!("Reference : {}", &*self.slice);
        }
    }
}

pub fn reference_operation() {
    println!("Reference operation running...in rust code");
    let s = S::new(String::from("Hello"));
    s.as_ref().show();
}
