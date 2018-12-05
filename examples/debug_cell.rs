use std::cell::Cell;
use std::cell::RefCell;
use std::borrow::Borrow;

fn main() {
    let cell = Cell::new("abc");
    let x = cell.borrow();
    x.set("def");

    let ref_cell = RefCell::new(vec![1, 2, 3]);

    {
        let mut ref_mut = ref_cell.borrow_mut();
        let y = 1;
        *ref_mut = vec![4, 5, 6];
        let yy = 2;
    }
    let r = ref_cell.borrow();
}