use crate::try_smart_pointers::pointer::RcList::{RcCons, RcNil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum RcList {
    RcCons(i32, RefCell<Rc<RcList>>),
    RcNil,
}

impl RcList {
    //  fn tail(&self) -> Option<&RefCell<Rc<RcList>>> {
    //        match self {
    //      RcCons(_, item) => Some(item),
    //      RcNil => None,
    //        }
    //  }
}

pub fn make_reference_cycle() {}
