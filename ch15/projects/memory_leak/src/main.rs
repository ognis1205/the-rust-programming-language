use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    use MutList::{Cons as MutCons, Nil as MutNil};

    let v = Rc::new(RefCell::new(5));
    let a = Rc::new(MutCons(Rc::clone(&v), Rc::new(MutNil)));
    let b = MutCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = MutCons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *v.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum MutList<T> {
    Cons(Rc<RefCell<T>>, Rc<MutList<T>>),
    Nil,
}
