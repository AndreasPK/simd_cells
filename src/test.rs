
#![cfg_attr(feature = "dump_mir", feature(rustc_attrs))]
#![cfg_attr(feature = "dump_mir", rustc_dump_program)]

struct A {
    b : u64,
    a : u64,
    c : Vec<u64>,
}

struct B {
    actually_a : A
}

fn foo(a: A) -> A {
    let t = a;
    t.a = 2;
    return t
}

fn bar(b: B) -> B {
    let x = B.actually_a;
    x.a = 3;
    x
}