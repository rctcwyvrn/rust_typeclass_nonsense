use crate::chaos::*;

fn main() {
    // <chaos::Target<True> as Test>::print();
    <chaos::Target<True> as Test>::print();
}

mod chaos {
    //#![allow(warnings)] // lmao


    use std::marker::PhantomData;
    pub struct Target<T> {
        phantom: PhantomData<T>
    }

    trait Testable {
        fn test() {}
    }

    pub trait Test {
        fn print();
    }

    // impl<T: Testable> Test for Target where T: Or<False,False,T> {
    //     fn print(){
    //         T::test() 
    //     }
    // }

    impl<T: Testable + Or<True,False>> Test for Target<T> {
        fn print(){
            T::test() 
        }
    }

    // enum Nil {}
    // enum Cons<x,xs> {
    //     x(x,xs)
    // }

    // trait First<a,b> {}
    // impl First<Nil, Nil> {}
    // impl<x, more> First<Cons<x, more>, x>{}

    pub enum True {}
    impl Testable for True {
        fn test() {
            println!("True");
        }
    }
    enum False {}
    impl Testable for False {
        fn test() {
            println!("False");
        }
    }

    trait Not<b1,b> {}
    impl Not<False, True> {}
    impl Not<True, False> {}

    // trait Or<b1,b2,b> {}
    // impl Or<False, True, True> {}
    // impl Or<True, True, True> {}
    // impl Or<True, False, True> {}
    // impl Or<False, False, False> {}

    trait Or<A,B> {
    }

    impl Or<True, True> for True {}
    impl Or<False, True> for True {}
    impl Or<True, False> for True {}

    impl Or<False, False> for False {}
}