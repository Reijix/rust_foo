#[derive(Debug)]
enum Day {
    Saturday,
    Sunday,
}

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
impl<T> List<T> {
    fn head(&self) -> Maybe<T>
    where
        T: Copy,
    {
        match self {
            List::Nil => Maybe::Nothing,
            List::Cons(x, _) => Maybe::Just(*x),
        }
    }
}

#[derive(Debug)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

fn main() {
    let x = Day::Saturday;
    let empt: List<i32> = List::Nil;
    let elem2: List<i32> = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    println!("Today is: {:#?}", x);
    println!("empty: {:#?}\nelem2: {:#?}", empt, elem2);
    let hd: i32 = match List::head(&elem2) {
        Maybe::Nothing => 0,
        Maybe::Just(n) => n,
    };
    println!("listhead is: {:#?}", hd);
}
