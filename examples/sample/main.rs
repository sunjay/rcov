#![feature(plugin)]
#![plugin(rcov)]

#![coverage]

mod foo;

use foo::foo;

fn main() {
    println!("Hello, World!!!!");

    foo();
}
