//@ known-bug: #130521

#![feature(dyn_compatible_for_dispatch)]
struct Vtable(dyn Cap);

trait Cap<'a> {}

union Transmute {
    t: u64,
    u: &'static Vtable,
}

const G: &Copy = unsafe { Transmute { t: 1 }.u };
