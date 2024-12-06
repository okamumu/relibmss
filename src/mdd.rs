use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use std::fmt::Display;

use dd::dot::Dot;
use dd::mdd::Mdd;
use dd::mtmdd2;
use dd::mtmdd2::build_from_rpn;
use dd::mtmdd2::gen_var;
use pyo3::pyclass;
use pyo3::pymethods;
use std::collections::HashMap;

#[pyclass(unsendable)]
pub struct MddMgr {
    mdd: Rc<RefCell<mtmdd2::MtMdd2<i64>>>,
    vars: Rc<RefCell<HashMap<String, MddNode>>>,
}

#[pyclass(unsendable)]
#[derive(Clone, Debug)]
pub struct MddNode {
    parent: Weak<RefCell<mtmdd2::MtMdd2<i64>>>,
    node: mtmdd2::MtMdd2Node<i64>,
}

impl MddNode {
    fn new(parent: Rc<RefCell<mtmdd2::MtMdd2<i64>>>, node: mtmdd2::MtMdd2Node<i64>) -> Self {
        MddNode {
            parent: Rc::downgrade(&parent),
            node,
        }
    }

    fn new_weak(parent: Weak<RefCell<mtmdd2::MtMdd2<i64>>>, node: mtmdd2::MtMdd2Node<i64>) -> Self {
        MddNode { parent, node }
    }
}

#[pymethods]
impl MddMgr {
    #[new]
    pub fn new() -> Self {
        MddMgr {
            mdd: Rc::new(RefCell::new(mtmdd2::MtMdd2::new())),
            vars: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn size(&self) -> (usize, usize, usize, usize) {
        self.mdd.borrow().size()
    }

    pub fn zero(&self) -> MddNode {
        MddNode::new(self.mdd.clone(), self.mdd.borrow().zero())
    }

    pub fn one(&self) -> MddNode {
        MddNode::new(self.mdd.clone(), self.mdd.borrow().one())
    }

    pub fn val(&self, value: i64) -> MddNode {
        let mut mdd = self.mdd.borrow_mut();
        let node = mdd.value(value);
        MddNode::new(self.mdd.clone(), node)
    }

    pub fn defvar(&self, label: &str, range: Vec<i64>) -> MddNode {
        let level = self.vars.borrow().len();
        let mut mdd = self.mdd.borrow_mut();
        let node = gen_var(&mut mdd, label, level, &range);
        let result = MddNode::new(self.mdd.clone(), node);
        self.vars
            .borrow_mut()
            .insert(label.to_string(), result.clone());
        result
    }

    pub fn var(&self, label: &str) -> Option<MddNode> {
        if let Some(node) = self.vars.borrow().get(label) {
            Some(node.clone())
        } else {
            None
        }
    }

    pub fn rpn(&self, rpn: &str) -> MddNode {
        let mut mdd = self.mdd.borrow_mut();
        // sparate white spaces and convert to tokens
        let tokens = rpn
            .split_whitespace()
            .map(|x| {
                match x {
                    "+" => mtmdd2::Token::Add,
                    "-" => mtmdd2::Token::Sub,
                    "*" => mtmdd2::Token::Mul,
                    "/" => mtmdd2::Token::Div,
                    "==" => mtmdd2::Token::Eq,
                    "!=" => mtmdd2::Token::Neq,
                    "<" => mtmdd2::Token::Lt,
                    "<=" => mtmdd2::Token::Lte,
                    ">" => mtmdd2::Token::Gt,
                    ">=" => mtmdd2::Token::Gte,
                    "&&" => mtmdd2::Token::And,
                    "||" => mtmdd2::Token::Or,
                    "!" => mtmdd2::Token::Not,
                    "?" => mtmdd2::Token::IfElse,
                    "true" => {
                        let node = mdd.one();
                        mtmdd2::Token::Value(node)
                    }
                    "false" => {
                        let node = mdd.zero();
                        mtmdd2::Token::Value(node)
                    }
                    _ => {
                        // parse whether it is a number or a variable
                        match x.parse::<i64>() {
                            Ok(val) => {
                                let node = mdd.value(val);
                                mtmdd2::Token::Value(node)
                            }
                            Err(_) => {
                                if let Some(node) = self.vars.borrow().get(x) {
                                    mtmdd2::Token::Value(node.node.clone())
                                } else {
                                    panic!("Unknown token: {}", x);
                                }
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
        let node = build_from_rpn(&mut mdd, &tokens).unwrap();
        MddNode::new(self.mdd.clone(), node)
    }
}

impl MddMgr {
    fn from_token(&self, tokens: &Vec<mtmdd2::Token<i64>>) -> MddNode {
        let mut mdd = self.mdd.borrow_mut();
        let node = build_from_rpn(&mut mdd, tokens).unwrap();
        MddNode::new(self.mdd.clone(), node)
    }
}

#[pymethods]
impl MddNode {
    pub fn dot(&self) -> String {
        self.node.dot_string()
    }
}

enum MddOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Not,
    IfElse,
}

impl Display for MddOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MddOp::Add => write!(f, "+"),
            MddOp::Sub => write!(f, "-"),
            MddOp::Mul => write!(f, "*"),
            MddOp::Div => write!(f, "/"),
            MddOp::Eq => write!(f, "=="),
            MddOp::Neq => write!(f, "!="),
            MddOp::Lt => write!(f, "<"),
            MddOp::Lte => write!(f, "<="),
            MddOp::Gt => write!(f, ">"),
            MddOp::Gte => write!(f, ">="),
            MddOp::And => write!(f, "&&"),
            MddOp::Or => write!(f, "||"),
            MddOp::Not => write!(f, "!"),
            MddOp::IfElse => write!(f, "?"),
        }
    }
}

enum _SymbolicNode {
    Operation {
        id: usize,
        op: MddOp,
        args: Vec<SymbolicNode>,
    },
    Var {
        id: usize,
        label: String,
        range: Vec<i64>,
    },
    Bool(bool),
    Val(i64),
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct SymbolicNode {
    parent: Weak<RefCell<_SymbolicMgr>>,
    node: Rc<_SymbolicNode>,
}

struct _SymbolicMgr {
    id_counter: RefCell<usize>,
    mddmgr: MddMgr,
    mddnodes: HashMap<usize, MddNode>,
    vars: HashMap<String, SymbolicNode>,
}

#[pyclass(unsendable)]
pub struct SymbolicMgr {
    mgr: Rc<RefCell<_SymbolicMgr>>,
}

impl _SymbolicMgr {
    fn generate_id(&self) -> usize {
        let id = *self.id_counter.borrow();
        *self.id_counter.borrow_mut() += 1;
        id
    }

    fn val(mgr: &Rc<RefCell<Self>>, value: i64) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let x = _SymbolicNode::Val(value);
        SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(x),
        }
    }

    fn bool(mgr: &Rc<RefCell<Self>>, value: bool) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let x = _SymbolicNode::Bool(value);
        SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(x),
        }
    }

    fn var(mgr: &Rc<RefCell<Self>>, label: &str, range: Vec<i64>) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let v = _SymbolicNode::Var {
            id,
            label: label.to_string(),
            range,
        };
        let new_var = SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(v),
        };
        mgr.borrow_mut()
            .vars
            .insert(label.to_string(), new_var.clone());
        new_var
    }

    fn and(mgr: &Rc<RefCell<Self>>, args: Vec<SymbolicNode>) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let x = _SymbolicNode::Operation {
            id,
            op: MddOp::And,
            args,
        };
        SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(x),
        }
    }

    fn or(mgr: &Rc<RefCell<Self>>, args: Vec<SymbolicNode>) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let x = _SymbolicNode::Operation {
            id,
            op: MddOp::Or,
            args,
        };
        SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(x),
        }
    }

    fn not(mgr: &Rc<RefCell<Self>>, node: SymbolicNode) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let x = _SymbolicNode::Operation {
            id,
            op: MddOp::Not,
            args: vec![node],
        };
        SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(x),
        }
    }

    fn ifelse(
        mgr: &Rc<RefCell<Self>>,
        cond: SymbolicNode,
        then: SymbolicNode,
        els: SymbolicNode,
    ) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let x = _SymbolicNode::Operation {
            id,
            op: MddOp::IfElse,
            args: vec![cond, then, els],
        };
        SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(x),
        }
    }

    fn binary(
        mgr: &Rc<RefCell<Self>>,
        op: MddOp,
        left: SymbolicNode,
        right: SymbolicNode,
    ) -> SymbolicNode {
        let id = mgr.borrow().generate_id();
        let x = _SymbolicNode::Operation {
            id,
            op,
            args: vec![left, right],
        };
        SymbolicNode {
            parent: Rc::downgrade(&mgr),
            node: Rc::new(x),
        }
    }

    // fn tomdd(mgr: &Rc<RefCell<Self>>, mddmgr: &Rc<RefCell<MddMgr>>, x: SymbolicNode) -> MddNode {
    //     let symgr = mgr.borrow();
    //     let node = x.node;

    // }

    fn tokens(mgr: &Rc<RefCell<Self>>, node: &SymbolicNode) -> Vec<mtmdd2::Token<i64>> {
        let x = node.node.clone();
        match x.as_ref() {
            _SymbolicNode::Operation { id, op, args } => {
                let mut tokens = Vec::new();
                for arg in args {
                    tokens.extend(Self::tokens(mgr, &arg));
                }
                match op {
                    MddOp::And => tokens.push(mtmdd2::Token::And),
                    MddOp::Or => tokens.push(mtmdd2::Token::Or),
                    MddOp::Not => tokens.push(mtmdd2::Token::Not),
                    MddOp::IfElse => tokens.push(mtmdd2::Token::IfElse),
                    MddOp::Add => tokens.push(mtmdd2::Token::Add),
                    MddOp::Sub => tokens.push(mtmdd2::Token::Sub),
                    MddOp::Mul => tokens.push(mtmdd2::Token::Mul),
                    MddOp::Div => tokens.push(mtmdd2::Token::Div),
                    MddOp::Eq => tokens.push(mtmdd2::Token::Eq),
                    MddOp::Neq => tokens.push(mtmdd2::Token::Neq),
                    MddOp::Lt => tokens.push(mtmdd2::Token::Lt),
                    MddOp::Lte => tokens.push(mtmdd2::Token::Lte),
                    MddOp::Gt => tokens.push(mtmdd2::Token::Gt),
                    MddOp::Gte => tokens.push(mtmdd2::Token::Gte),
                }
                tokens
            }
            _SymbolicNode::Var { id, label, range } => {
                let node = {
                    let symgr = mgr.borrow();
                    let mdd = &symgr.mddmgr;
                    mdd.var(&label)
                };
                if let Some(node) = node {
                    vec![mtmdd2::Token::Value(node.node.clone())]
                } else {
                    let symgr = mgr.borrow();
                    let mdd = &symgr.mddmgr;
                    let x = mdd.defvar(&label, range.clone());
                    vec![mtmdd2::Token::Value(x.node.clone())]
                }
            }
            _SymbolicNode::Bool(value) => {
                let symgr = mgr.borrow();
                let mdd = &symgr.mddmgr;
                if *value == true {
                    let n = mdd.mdd.borrow().one();
                    vec![mtmdd2::Token::Value(n)]
                } else {
                    let n = mdd.mdd.borrow().zero();
                    vec![mtmdd2::Token::Value(n)]
                }
            }
            _SymbolicNode::Val(value) => {
                let symgr = mgr.borrow();
                let mdd = &symgr.mddmgr;
                let n = mdd.mdd.borrow_mut().value(*value);
                vec![mtmdd2::Token::Value(n)]
            }
        }
    }
}

#[pymethods]
impl SymbolicMgr {
    #[new]
    fn new() -> Self {
        SymbolicMgr {
            mgr: Rc::new(RefCell::new(_SymbolicMgr {
                id_counter: RefCell::new(0),
                mddmgr: MddMgr::new(),
                mddnodes: HashMap::new(),
                vars: HashMap::new(),
            })),
        }
    }

    fn val(&self, value: i64) -> SymbolicNode {
        _SymbolicMgr::val(&self.mgr, value)
    }

    fn bool(&self, value: bool) -> SymbolicNode {
        _SymbolicMgr::bool(&self.mgr, value)
    }

    fn defvar(&self, label: &str, range: Vec<i64>) -> SymbolicNode {
        _SymbolicMgr::var(&self.mgr, label, range)
    }

    fn and(&self, args: Vec<SymbolicNode>) -> SymbolicNode {
        _SymbolicMgr::and(&self.mgr, args)
    }

    fn or(&self, args: Vec<SymbolicNode>) -> SymbolicNode {
        _SymbolicMgr::or(&self.mgr, args)
    }

    fn not(&self, node: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::not(&self.mgr, node)
    }

    fn ifelse(&self, cond: SymbolicNode, then: SymbolicNode, els: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::ifelse(&self.mgr, cond, then, els)
    }
}

impl _SymbolicNode {
    fn rpn_string(&self) -> String {
        match self {
            _SymbolicNode::Operation { id, op, args } => {
                let args_str = args
                    .iter()
                    .map(|x| format!("{}", x.node.rpn_string()))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{} {}", args_str, op)
            },
            _SymbolicNode::Var { id, label, range } => format!("{}", label),
            _SymbolicNode::Bool(value) => {
                if *value == true {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            },
            _SymbolicNode::Val(value) => format!("{}", value),
        }
    }
}

#[pymethods]
impl SymbolicNode {
    fn __str__(&self) -> String {
        match &*self.node {
            _SymbolicNode::Operation { id, op, args } => {
                let args_str = args
                    .iter()
                    .map(|x| format!("{}", x.__str__()))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{} {}", args_str, op)
            },
            _SymbolicNode::Var { id, label, range } => format!("{}", label),
            _SymbolicNode::Bool(value) => {
                if *value == true {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            },
            _SymbolicNode::Val(value) => format!("{}", value),
        }
    }

    fn __add__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Add,
            self.clone(),
            other,
        )
    }

    fn __sub__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Sub,
            self.clone(),
            other,
        )
    }

    fn __mul__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Mul,
            self.clone(),
            other,
        )
    }

    fn __div__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Div,
            self.clone(),
            other,
        )
    }

    fn __eq__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Eq,
            self.clone(),
            other,
        )
    }

    fn __ne__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Neq,
            self.clone(),
            other,
        )
    }

    fn __lt__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Lt,
            self.clone(),
            other,
        )
    }

    fn __le__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Lte,
            self.clone(),
            other,
        )
    }

    fn __gt__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Gt,
            self.clone(),
            other,
        )
    }

    fn __ge__(&self, other: SymbolicNode) -> SymbolicNode {
        _SymbolicMgr::binary(
            &self.parent.upgrade().unwrap(),
            MddOp::Gte,
            self.clone(),
            other,
        )
    }

    pub fn mdd(&self) -> MddNode {
        let mdd = self.parent.upgrade().unwrap();
        let tokens = _SymbolicMgr::tokens(&mdd, self);
        let x = mdd.borrow();
        x.mddmgr.from_token(&tokens)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_mdd_mgr() {
        let mgr = MddMgr::new();
        let zero = mgr.zero();
        let one = mgr.one();
        let two = mgr.val(2);
        let x = mgr.defvar("x", vec![0, 1, 2]);
        let y = mgr.defvar("y", vec![0, 1, 2]);
        let z = mgr.defvar("z", vec![0, 1, 2]);
        // println!("vars: {:?}", mgr.vars.borrow());
        let rpn = "x y z + *";
        let node = mgr.rpn(rpn);
        println!("{}", node.dot());
    }
}
