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

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum MutList<T> {
    Cons(Rc<RefCell<T>>, Rc<MutList<T>>),
    Nil,
}

#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match *self {
            List::Cons(_, ref item) => Some(item),
            List::Nil => None,
        }
    }
}
