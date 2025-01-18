use crate::interval::Interval;
use pyo3::{exceptions::PyValueError, prelude::*};
use rust_mss::prelude::*;

impl ProbValue for Interval {}

#[pyclass(unsendable)]
pub struct PyBddMgr(BddMgr);

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct PyBddNode(BddNode);

#[pymethods]
impl PyBddMgr {
    // constructor
    #[new]
    pub fn new() -> Self {
        PyBddMgr(BddMgr::new())
    }

    // size
    pub fn size(&self) -> (usize, usize, usize) {
        self.0.size()
    }

    // zero
    pub fn zero(&self) -> PyBddNode {
        PyBddNode(self.0.zero())
    }

    // one
    pub fn one(&self) -> PyBddNode {
        PyBddNode(self.0.one())
    }

    // defvar
    pub fn defvar(&mut self, var: &str) -> PyBddNode {
        PyBddNode(self.0.defvar(var))
    }

    pub fn get_varorder(&self) -> Vec<String> {
        self.0.get_varorder()
    }

    pub fn rpn(&mut self, expr: &str) -> PyResult<PyBddNode> {
        if let Ok(node) = self.0.rpn(expr) {
            Ok(PyBddNode(node))
        } else {
            Err(PyValueError::new_err("Invalid expression"))
        }
    }

    pub fn And(&self, nodes: Vec<PyBddNode>) -> PyBddNode {
        let xs = nodes.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
        PyBddNode(self.0.and(&xs))
    }

    pub fn Or(&self, nodes: Vec<PyBddNode>) -> PyBddNode {
        let xs = nodes.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
        PyBddNode(self.0.or(&xs))
    }
}

#[pymethods]
impl PyBddNode {
    pub fn dot(&self) -> String {
        self.0.dot()
    }

    pub fn __and__(&self, other: &PyBddNode) -> PyBddNode {
        PyBddNode(self.0.and(&other.0))
    }

    pub fn __or__(&self, other: &PyBddNode) -> PyBddNode {
        PyBddNode(self.0.or(&other.0))
    }

    pub fn __xor__(&self, other: &PyBddNode) -> PyBddNode {
        PyBddNode(self.0.xor(&other.0))
    }

    fn __invert__(&self) -> PyBddNode {
        PyBddNode(self.0.not())
    }

    pub fn ifelse(&self, then: &PyBddNode, else_: &PyBddNode) -> PyBddNode {
        PyBddNode(self.0.ite(&then.0, &else_.0))
    }

    pub fn prob(&self, pv: HashMap<String, f64>, ss: Vec<bool>) -> f64 {
        self.0.prob(&pv, &ss)
    }

    pub fn bmeas(&self, pv: HashMap<String, f64>, ss: Vec<bool>) -> HashMap<String, f64> {
        self.0.bmeas(&pv, &ss)
    }

    pub fn prob_interval(&self, pv: HashMap<String, Interval>, ss: Vec<bool>) -> Interval {
        self.0.prob(&pv, &ss)
    }

    pub fn bmeas_interval(
        &self,
        pv: HashMap<String, Interval>,
        ss: Vec<bool>,
    ) -> HashMap<String, Interval> {
        self.0.bmeas(&pv, &ss)
    }

    pub fn minpath(&self) -> PyBddNode {
        PyBddNode(self.0.minpath())
    }

    pub fn bdd_count(&self, ss: Vec<bool>) -> u64 {
        self.0.bdd_count(&ss)
    }

    pub fn zdd_count(&self, ss: Vec<bool>) -> u64 {
        self.0.zdd_count(&ss)
    }

    pub fn bdd_extract(&self, ss: Vec<bool>) -> PyBddPath {
        PyBddPath::new(&self, ss.clone())
    }

    pub fn zdd_extract(&self, ss: Vec<bool>) -> PyZddPath {
        PyZddPath::new(&self, ss.clone())
    }

    pub fn size(&self) -> (u64, u64, u64) {
        self.0.size()
    }
}

#[pyclass(unsendable)]
pub struct PyBddPath {
    bddnode: BddNode,
    bddpath: BddPath,
    domain: Vec<bool>,
}

#[pyclass(unsendable)]
pub struct PyZddPath {
    bddnode: BddNode,
    bddpath: ZddPath,
    domain: Vec<bool>,
}

#[pymethods]
impl PyBddPath {
    #[new]
    fn new(node: &PyBddNode, ss: Vec<bool>) -> Self {
        let bddpath = node.0.bdd_extract(&ss);
        PyBddPath {
            bddnode: node.0.clone(),
            bddpath,
            domain: ss.clone(),
        }
    }

    fn __len__(&self) -> usize {
        self.bddnode.bdd_count(&self.domain) as usize
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Vec<String>> {
        slf.bddpath.next()
    }
}

#[pymethods]
impl PyZddPath {
    #[new]
    fn new(node: &PyBddNode, ss: Vec<bool>) -> Self {
        let bddpath = node.0.zdd_extract(&ss);
        PyZddPath {
            bddnode: node.0.clone(),
            bddpath,
            domain: ss.clone(),
        }
    }

    fn __len__(&self) -> usize {
        self.bddnode.bdd_count(&self.domain) as usize
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Vec<String>> {
        slf.bddpath.next()
    }
}
