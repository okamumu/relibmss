//

use dd::bdd;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::ft;
use crate::bdd as pybdd;

pub enum FTNode {
    Basic {
        id: usize,
        name: String,
    },
    Repeat {
        id: usize,
        name: String,
    },
    And {
        id: usize,
        args: Vec<FtNode>,
    },
    Or {
        id: usize,
        args: Vec<FtNode>,
    },
    KofN {
        id: usize,
        k: usize,
        args: Vec<FtNode>,
    },
}

pub struct Var {
    pub node: Vec<String>,
}

struct FTMgr {
    pub id: RefCell<usize>,
    pub events: RefCell<HashMap<String,Var>>,
    pub bddnode: RefCell<HashMap<usize,pybdd::BddNode>>,
}

impl FTMgr {
    pub fn new() -> FTMgr {
        FTMgr {
            id: RefCell::new(0),
            events: RefCell::new(HashMap::new()),
            bddnode: RefCell::new(HashMap::new()),
        }
    }

    pub fn id(&self) -> usize {
        *self.id.borrow()
    }

    fn basic(mgr: &Rc<RefCell<Self>>, name: &str) -> FtNode {
        let mgr_borrow = mgr.borrow();
        let node = FTNode::Basic { id: mgr_borrow.id(), name: name.to_string() };
        *mgr_borrow.id.borrow_mut() += 1;
        FtNode::new(mgr.clone(), Rc::new(node))
    }

    fn repeat(mgr: &Rc<RefCell<Self>>, name: &str) -> FtNode {
        let mgr_borrow = mgr.borrow();
        let node = FTNode::Repeat { id: mgr_borrow.id(), name: name.to_string() };
        *mgr_borrow.id.borrow_mut() += 1;
        FtNode::new(mgr.clone(), Rc::new(node))
    }

    fn and(mgr: &Rc<RefCell<Self>>, args: Vec<FtNode>) -> FtNode {
        let mgr_borrow = mgr.borrow();
        let node = FTNode::And { id: mgr_borrow.id(), args };
        *mgr_borrow.id.borrow_mut() += 1;
        FtNode::new(mgr.clone(), Rc::new(node))
    }

    fn or(mgr: &Rc<RefCell<Self>>, args: Vec<FtNode>) -> FtNode {
        let mgr_borrow = mgr.borrow();
        let node = FTNode::Or { id: mgr_borrow.id(), args };
        *mgr_borrow.id.borrow_mut() += 1;
        FtNode::new(mgr.clone(), Rc::new(node))
    }

    fn kofn(mgr: &Rc<RefCell<Self>>, k: usize, args: Vec<FtNode>) -> FtNode {
        let mgr_borrow = mgr.borrow();
        let node = FTNode::KofN { id: mgr_borrow.id(), k, args };
        *mgr_borrow.id.borrow_mut() += 1;
        FtNode::new(mgr.clone(), Rc::new(node))
    }

    fn create(mgr: &Rc<RefCell<Self>>, bddmgr: &mut pybdd::BddMgr, top: FtNode) -> pybdd::BddNode {
        let ftmgr = mgr.borrow();
        let node = top.node();
        match node.as_ref() {
            FTNode::Basic { id, name } => {
                match ftmgr.events.borrow_mut().get_mut(name) {
                    Some(v) => {
                        let u = v.node.len();
                        let name_ = format!("{}_{}", name, u);
                        let x = bddmgr.var(&name_).unwrap();
                        v.node.push(name_);
                        x.clone()
                    },
                    None => {
                        let name_ = format!("{}_0", name);
                        let x = bddmgr.var(&name_).unwrap();
                        let v = Var { node: vec![name_] };
                        ftmgr.events.borrow_mut().insert(name.clone(), v);
                        x.clone()
                    },
                }
            }
            FTNode::Repeat { id, name } => {
                match ftmgr.events.borrow_mut().get(name) {
                    Some(v) => bddmgr.var(name).unwrap(),
                    None => {
                        let x = bddmgr.var(&name).unwrap();
                        let v = Var { node: vec![name.clone()] };
                        ftmgr.events.borrow_mut().insert(name.clone(), v);
                        x.clone()
                    },
                }
            }
            FTNode::And { id, args } => {
                match ftmgr.bddnode.borrow_mut().get(id) {
                    Some(x) => x.clone(),
                    None => {
                        let mut b = Vec::new();
                        for arg in args.iter() {
                            let tmp = Self::create(mgr, bddmgr, arg.clone());
                            b.push(tmp);
                        }
                        let x = bddmgr.and(b);
                        ftmgr.bddnode.borrow_mut().insert(*id, x.clone());
                        x.clone()
                    },
                }
            }
            FTNode::Or { id, args } => {
                match ftmgr.bddnode.borrow_mut().get(id) {
                    Some(x) => x.clone(),
                    None => {
                        let mut b = Vec::new();
                        for arg in args.iter() {
                            let tmp = Self::create(mgr, bddmgr, arg.clone());
                            b.push(tmp);
                        }
                        let x = bddmgr.or(b);
                        ftmgr.bddnode.borrow_mut().insert(*id, x.clone());
                        x.clone()
                    },
                }
            }
            FTNode::KofN { id, k, args } => {
                match ftmgr.bddnode.borrow_mut().get(id) {
                    Some(x) => x.clone(),
                    None => {
                        let mut b = Vec::new();
                        for arg in args.iter() {
                            let tmp = Self::create(mgr, bddmgr, arg.clone());
                            b.push(tmp);
                        }
                        let x = bddmgr.kofn(*k, b);
                        ftmgr.bddnode.borrow_mut().insert(*id, x.clone());
                        x.clone()
                    },
                }
            }
        }
    }
    
}

#[pyclass(unsendable)]
pub struct FtMgr {
    ftmgr: Rc<RefCell<FTMgr>>,
}

#[pymethods]
impl FtMgr {
    #[new]
    pub fn new() -> FtMgr {
        FtMgr {
            ftmgr: Rc::new(RefCell::new(FTMgr::new())),
        }
    }

    pub fn basic(&self, name: &str) -> FtNode {
        FTMgr::basic(&self.ftmgr, name)
    }

    pub fn repeat(&self, name: &str) -> FtNode {
        FTMgr::repeat(&self.ftmgr, name)
    }

    pub fn and(&self, args: Vec<FtNode>) -> FtNode {
        FTMgr::and(&self.ftmgr, args)
    }

    pub fn or(&self, args: Vec<FtNode>) -> FtNode {
        FTMgr::or(&self.ftmgr, args)
    }

    pub fn kofn(&self, k: usize, args: Vec<FtNode>) -> FtNode {
        FTMgr::kofn(&self.ftmgr, k, args)
    }
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct FtNode {
    ftmgr: Weak<RefCell<FTMgr>>,
    node: Rc<FTNode>,
}

impl FtNode {
    fn new(ftmgr: Rc<RefCell<FTMgr>>, node: Rc<FTNode>) -> Self {
        FtNode {
            ftmgr: Rc::downgrade(&ftmgr),
            node: node,
        }
    }

    pub fn ftmgr(&self) -> Rc<RefCell<FTMgr>> {
        self.ftmgr.upgrade().unwrap()
    }

    pub fn node(&self) -> Rc<FTNode> {
        self.node.clone()
    }
}

#[pymethods]
impl FtNode {
    fn __repr__(&self) -> String {
        match self.node.as_ref() {
            FTNode::Basic { name, .. } => name.clone(),
            FTNode::Repeat { name, .. } => name.clone(),
            FTNode::And { args, .. } => args.iter().map(|x| x.__repr__()).collect::<Vec<String>>().join(" & "),
            FTNode::Or { args, .. } => args.iter().map(|x| x.__repr__()).collect::<Vec<String>>().join(" | "),
            FTNode::KofN { k, args, .. } => format!("{} of {}", k, args.iter().map(|x| x.__repr__()).collect::<Vec<String>>().join(" | ")),
        }
    }

    fn __and__(&self, other: &FtNode) -> FtNode {
        let ftmgr = self.ftmgr();
        let args = vec![self.clone(), other.clone()];
        FTMgr::and(&ftmgr, args)
    }

    fn __or__(&self, other: &FtNode) -> FtNode {
        let ftmgr = self.ftmgr();
        let args = vec![self.clone(), other.clone()];
        FTMgr::or(&ftmgr, args)
    }

    pub fn compile(&self, bddmgr: &mut pybdd::BddMgr) -> pybdd::BddNode {
        let ftmgr = self.ftmgr();
        FTMgr::create(&ftmgr, bddmgr, self.clone())
    }
}

#[pyfunction]
pub fn ftkofn(k: usize, args: Vec<FtNode>) -> FtNode {
    let ftmgr = args[0].ftmgr();
    FTMgr::kofn(&ftmgr, k, args)
}
