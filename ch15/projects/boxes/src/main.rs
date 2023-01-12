use std::ops::Deref;
use std::rc::Rc;

fn main() {
    //    use List::{Cons, Nil};

    let b = Box::new(5);
    println!("b = {}", b);

    //    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //    println!("list = {:?}", list);

    let x = 5;
    let y = MyBox::new(x);
    println!("y = {:?}", y);
    println!("y = {:?}", *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("custom pointer creaqted");

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("list = {:?}", a);
    println!("list = {:?}", b);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

//#[derive(Debug)]
//enum List<T> {
//    Cons(T, Box<List<T>>),
//    Nil,
//}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping with data {}", self.data);
    }
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}
