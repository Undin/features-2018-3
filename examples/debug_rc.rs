use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let rc = Rc::new("abc");
    let rc2 = rc.clone();
    let weak = Rc::downgrade(&rc2);

    let arc = Arc::new("abc");
    let arc2 = arc.clone();
    let weak2 = Arc::downgrade(&arc2);
}