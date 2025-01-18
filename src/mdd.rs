use crate::interval::Interval;
use pyo3::{exceptions::PyValueError, prelude::*};
use rust_mss::prelude::*;

#[pyclass(unsendable)]
pub struct PyMddMgr(MddMgr<i64>);

#[pyclass(unsendable)]
#[derive(Clone, Debug)]
pub struct PyMddNode(MddNode<i64>);

#[pymethods]
impl PyMddMgr {
    #[new]
    pub fn new() -> Self {
        PyMddMgr(MddMgr::new())
    }

    pub fn size(&self) -> (usize, usize, usize, usize) {
        self.0.size()
    }

    pub fn boolean(&self, val: bool) -> PyMddNode {
        PyMddNode(self.0.boolean(val))
    }

    pub fn value(&self, val: i64) -> PyMddNode {
        PyMddNode(self.0.value(val))
    }

    pub fn defvar(&mut self, label: &str, range: usize) -> PyMddNode {
        PyMddNode(self.0.defvar(label, range))
    }

    pub fn rpn(&mut self, rpn: &str, vars: HashMap<String, usize>) -> PyResult<PyMddNode> {
        if let Ok(node) = self.0.rpn(rpn, &vars) {
            Ok(PyMddNode(node))
        } else {
            Err(PyValueError::new_err("Invalid expression"))
        }
    }

    pub fn And(&mut self, nodes: Vec<PyMddNode>) -> PyMddNode {
        let xs = nodes.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
        PyMddNode(self.0.and(&xs))
    }

    pub fn Or(&mut self, nodes: Vec<PyMddNode>) -> PyMddNode {
        let xs = nodes.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
        PyMddNode(self.0.or(&xs))
    }

    pub fn Not(&mut self, node: &PyMddNode) -> PyMddNode {
        PyMddNode(node.0.not())
    }

    pub fn Min(&mut self, nodes: Vec<PyMddNode>) -> PyMddNode {
        let xs = nodes.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
        PyMddNode(self.0.min(&xs))
    }

    pub fn Max(&mut self, nodes: Vec<PyMddNode>) -> PyMddNode {
        let xs = nodes.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
        PyMddNode(self.0.max(&xs))
    }
}

#[pymethods]
impl PyMddNode {
    pub fn dot(&self) -> String {
        self.0.dot()
    }

    pub fn __add__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.add(&other.0))
    }

    pub fn __sub__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.sub(&other.0))
    }

    pub fn __mul__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.mul(&other.0))
    }

    pub fn __truediv__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.div(&other.0))
    }

    pub fn __eq__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.eq(&other.0))
    }

    pub fn __ne__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.ne(&other.0))
    }

    pub fn __lt__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.lt(&other.0))
    }

    pub fn __le__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.le(&other.0))
    }

    pub fn __gt__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.gt(&other.0))
    }

    pub fn __ge__(&self, other: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.ge(&other.0))
    }

    pub fn ifelse(&self, then: &PyMddNode, els: &PyMddNode) -> PyMddNode {
        PyMddNode(self.0.ite(&then.0, &els.0))
    }

    pub fn prob(&mut self, pv: HashMap<String, Vec<f64>>, ss: Vec<i64>) -> f64 {
        self.0.prob(&pv, &ss)
    }

    pub fn prob_interval(&mut self, pv: HashMap<String, Vec<Interval>>, ss: Vec<i64>) -> Interval {
        self.0.prob(&pv, &ss)
    }

    pub fn minpath(&mut self) -> PyMddNode {
        PyMddNode(self.0.minpath())
    }

    pub fn mdd_count(&self, ss: Vec<i64>) -> u64 {
        let ss = ss.iter().map(|x| *x).collect::<HashSet<_>>();
        self.0.mdd_count(&ss)
    }

    pub fn zmdd_count(&self, ss: Vec<i64>) -> u64 {
        let ss = ss.iter().map(|x| *x).collect::<HashSet<_>>();
        self.0.zmdd_count(&ss)
    }

    pub fn size(&self) -> (u64, u64, u64) {
        self.0.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdd_mgr() {
        let mut mgr = PyMddMgr::new();
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), 3);
        vars.insert("y".to_string(), 3);
        vars.insert("z".to_string(), 3);
        let rpn = "x y z + *";
        if let Ok(node) = mgr.rpn(rpn, vars) {
            println!("{}", node.dot());
        }
    }
}
