#[derive(Debug)]
pub struct Foo;

pub fn foo() {
    println!("Public foo!!");
    private();

    let f = Foo;
    println!("This is my struct: {:?}", f);
}

fn private() {
    println!("Private method!!!");

    for i in 0..10 {
        println!("Privately writing a number...{}", i);
    }
}

