use dd::count::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::rc::Weak;

use dd::dot::Dot;
use dd::mtmdd2;
use dd::mtmdd2::build_from_rpn;
use dd::mtmdd2::gen_var;
use pyo3::exceptions::PyValueError;
use pyo3::pyclass;
use pyo3::pymethods;
use pyo3::PyResult;
use std::collections::HashMap;

use crate::interval::Interval;
use crate::mdd_algo;

#[pyclass(unsendable)]
pub struct MddMgr {
    mdd: Rc<RefCell<mtmdd2::MtMdd2<i64>>>,
    vars: HashMap<String, MddNode>,
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
}

#[pymethods]
impl MddMgr {
    #[new]
    pub fn new() -> Self {
        MddMgr {
            mdd: Rc::new(RefCell::new(mtmdd2::MtMdd2::new())),
            vars: HashMap::new(),
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

    pub fn defvar(&mut self, label: &str, range: usize) -> MddNode {
        if let Some(node) = self.vars.get(label) {
            return node.clone();
        } else {
            let level = self.vars.len();
            let result = {
                let mut mdd = self.mdd.borrow_mut();
                let range_ = (0..range).map(|x| x as i64).collect::<Vec<_>>(); // TODO: it should be changed in gen_var
                let node = gen_var(&mut mdd, label, level, &range_);
                MddNode::new(self.mdd.clone(), node)
            };
            self.vars.insert(label.to_string(), result.clone());
            result
        }
    }

    pub fn var(&self, label: &str) -> Option<MddNode> {
        if let Some(node) = self.vars.get(label) {
            Some(node.clone())
        } else {
            None
        }
    }

    pub fn rpn(&mut self, rpn: &str, vars: HashMap<String, usize>) -> PyResult<MddNode> {
        let mut stack = Vec::new();
        let mut cache = HashMap::new();

        for token in rpn.split_whitespace() {
            match token {
                "+" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.add(&a, &b);
                    stack.push(tmp);
                }
                "-" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.sub(&a, &b);
                    stack.push(tmp);
                }
                "*" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.mul(&a, &b);
                    stack.push(tmp);
                }
                "/" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.div(&a, &b);
                    stack.push(tmp);
                }
                "==" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.eq(&a, &b);
                    stack.push(tmp);
                }
                "!=" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.neq(&a, &b);
                    stack.push(tmp);
                }
                "<" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.lt(&a, &b);
                    stack.push(tmp);
                }
                "<=" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.lte(&a, &b);
                    stack.push(tmp);
                }
                ">" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.gt(&a, &b);
                    stack.push(tmp);
                }
                ">=" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.gte(&a, &b);
                    stack.push(tmp);
                }
                "&&" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.and(&a, &b);
                    stack.push(tmp);
                }
                "||" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.or(&a, &b);
                    stack.push(tmp);
                }
                "!" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.not(&a);
                    stack.push(tmp);
                }
                "?" => {
                    let mut mdd = self.mdd.borrow_mut();
                    let c = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let tmp = mdd.ifelse(&a, &b, &c);
                    stack.push(tmp);
                }
                "True" => {
                    let node = {
                        let mdd = self.mdd.borrow();
                        mdd.one()
                    };
                    stack.push(node);
                }
                "False" => {
                    let node = {
                        let mdd = self.mdd.borrow();
                        mdd.zero()
                    };
                    stack.push(node);
                }
                _ if token.starts_with("save(") && token.ends_with(")") => {
                    let name = &token[5..token.len() - 1];
                    if let Some(node) = stack.last() {
                        cache.insert(name.to_string(), node.clone());
                    } else {
                        return Err(PyValueError::new_err("Stack is empty for save operation"));
                    }
                }
                _ if token.starts_with("load(") && token.ends_with(")") => {
                    let name = &token[5..token.len() - 1];
                    if let Some(node) = cache.get(name) {
                        stack.push(node.clone());
                    } else {
                        return Err(PyValueError::new_err(format!("No cached value for {}", name)));
                    }
                }
                _ => {
                    // parse whether it is a number or a variable
                    match token.parse::<i64>() {
                        Ok(val) => {
                            let node = {
                                let mut mdd = self.mdd.borrow_mut();
                                mdd.value(val)
                            };
                            stack.push(node);
                        }
                        Err(_) => {
                            let result = self.vars.get(token);
                            if let Some(node) = result {
                                stack.push(node.node.clone());
                            } else {
                                match vars.get(token) {
                                    Some(range) => {
                                        let node = self.defvar(token, range.clone());
                                        stack.push(node.node.clone());
                                    }
                                    None => panic!("Unknown variable: {}", token),
                                }
                            }
                        }
                    }
                }
            }
        }
        if stack.len() == 1 {
            let node = stack.pop().unwrap();
            Ok(MddNode::new(self.mdd.clone(), node))
        } else {
            Err(PyValueError::new_err("Invalid expression"))
        }
    }

    // pub fn rpn(&mut self, rpn: &str, vars: HashMap<String, usize>) -> PyResult<MddNode> {
    //     let tokens = rpn
    //         .split_whitespace()
    //         .map(|x| {
    //             match x {
    //                 "+" => mtmdd2::Token::Add,
    //                 "-" => mtmdd2::Token::Sub,
    //                 "*" => mtmdd2::Token::Mul,
    //                 "/" => mtmdd2::Token::Div,
    //                 "==" => mtmdd2::Token::Eq,
    //                 "!=" => mtmdd2::Token::Neq,
    //                 "<" => mtmdd2::Token::Lt,
    //                 "<=" => mtmdd2::Token::Lte,
    //                 ">" => mtmdd2::Token::Gt,
    //                 ">=" => mtmdd2::Token::Gte,
    //                 "&&" => mtmdd2::Token::And,
    //                 "||" => mtmdd2::Token::Or,
    //                 "!" => mtmdd2::Token::Not,
    //                 "?" => mtmdd2::Token::IfElse,
    //                 "True" => {
    //                     let node = {
    //                         let mdd = self.mdd.borrow();
    //                         mdd.one()
    //                     };
    //                     mtmdd2::Token::Value(node)
    //                 }
    //                 "False" => {
    //                     let node = {
    //                         let mdd = self.mdd.borrow();
    //                         mdd.zero()
    //                     };
    //                     mtmdd2::Token::Value(node)
    //                 }
    //                 _ => {
    //                     // parse whether it is a number or a variable
    //                     match x.parse::<i64>() {
    //                         Ok(val) => {
    //                             let node = {
    //                                 let mut mdd = self.mdd.borrow_mut();
    //                                 mdd.value(val)
    //                             };
    //                             mtmdd2::Token::Value(node)
    //                         }
    //                         Err(_) => {
    //                             let result = self.vars.get(x);
    //                             if let Some(node) = result {
    //                                 mtmdd2::Token::Value(node.node.clone())
    //                             } else {
    //                                 match vars.get(x) {
    //                                     Some(range) => {
    //                                         let node = self.defvar(x, range.clone());
    //                                         mtmdd2::Token::Value(node.node.clone())
    //                                     }
    //                                     None => panic!("Unknown variable: {}", x),
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         })
    //         .collect::<Vec<_>>();
    //     let mut mdd = self.mdd.borrow_mut();
    //     if let Ok(node) = build_from_rpn(&mut mdd, &tokens) {
    //         Ok(MddNode::new(self.mdd.clone(), node))
    //     } else {
    //         Err(PyValueError::new_err("Invalid expression"))
    //     }
    // }

    pub fn _and(&mut self, nodes: Vec<MddNode>) -> MddNode {
        let mut mdd = self.mdd.borrow_mut();
        let xs = nodes.iter().map(|x| &x.node).collect::<Vec<_>>();
        let mut result = mdd.one();
        for node in xs {
            result = mdd.and(&result, &node);
        }
        MddNode::new(self.mdd.clone(), result)
    }

    pub fn _or(&mut self, nodes: Vec<MddNode>) -> MddNode {
        let mut mdd = self.mdd.borrow_mut();
        let xs = nodes.iter().map(|x| &x.node).collect::<Vec<_>>();
        let mut result = mdd.zero();
        for node in xs {
            result = mdd.or(&result, &node);
        }
        MddNode::new(self.mdd.clone(), result)
    }

    pub fn _not(&mut self, node: &MddNode) -> MddNode {
        let mut mdd = self.mdd.borrow_mut();
        let result = mdd.not(&node.node);
        MddNode::new(self.mdd.clone(), result)
    }

    pub fn ifelse(&mut self, cond: &MddNode, then: &MddNode, els: &MddNode) -> MddNode {
        let mut mdd = self.mdd.borrow_mut();
        let result = mdd.ifelse(&cond.node, &then.node, &els.node);
        MddNode::new(self.mdd.clone(), result)
    }
}

#[pymethods]
impl MddNode {
    pub fn dot(&self) -> String {
        self.node.dot_string()
    }

    pub fn add(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.add(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn sub(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.sub(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn mul(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.mul(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn div(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.div(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn eq(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.eq(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn ne(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.neq(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn lt(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.lt(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn le(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.lte(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn gt(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.gt(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn ge(&self, other: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.gte(&self.node, &other.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn ifelse(&self, then: &MddNode, els: &MddNode) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.ifelse(&self.node, &then.node, &els.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn value(&self, other: i64) -> MddNode {
        let mddmgr = self.parent.upgrade().unwrap();
        let mut mdd = mddmgr.borrow_mut();
        let node = mdd.value(other);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn boolean(&self, other: bool) -> MddNode {
        if other {
            let mddmgr = self.parent.upgrade().unwrap();
            let mdd = mddmgr.borrow();
            let node = mdd.one();
            MddNode::new(self.parent.upgrade().unwrap(), node)
        } else {
            let mddmgr = self.parent.upgrade().unwrap();
            let mdd = mddmgr.borrow();
            let node = mdd.zero();
            MddNode::new(self.parent.upgrade().unwrap(), node)
        }
    }

    pub fn prob(&mut self, pv: HashMap<String, Vec<f64>>, ss: HashSet<i64>) -> f64 {
        let mgr = self.parent.upgrade().unwrap();
        let mut mdd = mgr.borrow_mut();
        mdd_algo::mddprob(&mut mdd, &self.node, &pv, &ss)
    }

    pub fn prob_interval(
        &mut self,
        pv: HashMap<String, Vec<Interval>>,
        ss: HashSet<i64>,
    ) -> Interval {
        let mgr = self.parent.upgrade().unwrap();
        let mut mdd = mgr.borrow_mut();
        mdd_algo::mddprob(&mut mdd, &self.node, &pv, &ss)
    }

    pub fn mpvs(&mut self) -> MddNode {
        let mgr = self.parent.upgrade().unwrap();
        let mut mdd = mgr.borrow_mut();
        let node = mdd_algo::mddminsol(&mut mdd, &self.node);
        MddNode::new(self.parent.upgrade().unwrap(), node)
    }

    pub fn size(&self) -> (usize, u64) {
        match &self.node {
            mtmdd2::MtMdd2Node::Value(x) => x.count(),
            mtmdd2::MtMdd2Node::Bool(x) => x.count(),
            mtmdd2::MtMdd2Node::Undet => (0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdd_mgr() {
        let mut mgr = MddMgr::new();
        // let zero = mgr.zero();
        // let one = mgr.one();
        // let two = mgr.val(2);
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 3);
        vars.insert("y".to_string(), 3);
        vars.insert("z".to_string(), 3);
        // println!("vars: {:?}", mgr.vars.borrow());
        let rpn = "x y z + *";
        if let Ok(node) = mgr.rpn(rpn, vars) {
            println!("{}", node.dot());
        }
    }
}
