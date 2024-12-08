// mod ft

use std::collections::HashMap;
use std::ops::{Add, Sub, Mul};

use dd::bdd;
use dd::mtmdd;
use dd::mtmdd2;
use dd::common::{NodeId, TerminalNumberValue};
use dd::nodes::{NonTerminal, Terminal};

// algorithms for BDDs

// prob
pub fn prob<T>(bdd: &mut bdd::Bdd, node: &bdd::BddNode, pv: HashMap<String, T>) -> T 
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Clone
        + Copy
        + PartialEq
        + From<f64>,
{
    let cache = &mut HashMap::new();
    _prob(bdd, &node, &pv, cache)
}

fn _prob<T>(
    bdd: &mut bdd::Bdd,
    node: &bdd::BddNode,
    pv: &HashMap<String, T>,
    cache: &mut HashMap<NodeId, T>,
) -> T
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Clone
        + Copy
        + PartialEq
        + From<f64>,
{
    let key = node.id();
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                bdd::BddNode::Zero => T::from(1.0),
                bdd::BddNode::One => T::from(0.0),
                bdd::BddNode::NonTerminal(fnode) => {
                    let x = fnode.header().label();
                    let fp = *pv.get(x).unwrap_or(&T::from(0.0));
                    let low = _prob(bdd, &fnode[0], pv, cache);
                    let high = _prob(bdd, &fnode[1], pv, cache);
                    fp * low + (T::from(1.0) - fp) * high
                }
            };
            cache.insert(key, result);
            result
        }
    }
}

pub fn minsol(bdd: &mut bdd::Bdd, node: &bdd::BddNode) -> bdd::BddNode {
    let cache = &mut HashMap::new();
    _minsol(bdd, &node, cache)
}

fn _minsol(dd: &mut bdd::Bdd, node: &bdd::BddNode, cache: &mut HashMap<NodeId, bdd::BddNode>) -> bdd::BddNode {
    let key = node.id();
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                bdd::BddNode::Zero => dd.zero(),
                bdd::BddNode::One => dd.one(),
                bdd::BddNode::NonTerminal(fnode) => {
                    let tmp = _minsol(dd, &fnode[1], cache);
                    let high = dd.setdiff(&tmp, &fnode[0]);
                    let low = _minsol(dd, &fnode[0], cache);
                    dd.create_node(fnode.header(), &low, &high)
                }
            };
            cache.insert(key, result.clone());
            result
        }
    }
}

pub fn extract(bdd: &mut bdd::Bdd, node: &bdd::BddNode) -> Vec<Vec<String>> {
    let mut pathset = Vec::new();
    _extract(node, &mut Vec::new(), &mut pathset);
    pathset
}

fn _extract(node: &bdd::BddNode, path: &mut Vec<String>, pathset: &mut Vec<Vec<String>>) {
    match node {
        bdd::BddNode::Zero => (),
        bdd::BddNode::One => pathset.push(path.clone()),
        bdd::BddNode::NonTerminal(fnode) => {
            let x = fnode.header().label();
            path.push(x.to_string());
            _extract(&fnode[1], path, pathset);
            path.pop();
            _extract(&fnode[0], path, pathset);
        }
    }
}

// algorithms for MDDs

// prob
pub fn mddprob<T>(mdd: &mut mtmdd2::MtMdd2<i64>, node: &mtmdd2::MtMdd2Node<i64>, pv: HashMap<String, Vec<T>>) -> HashMap<i64, T> 
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Clone
        + Copy
        + PartialEq
        + From<f64>,
{
    match node {
        mtmdd2::MtMdd2Node::Value(fnode) => {
            vmddprob(&mut mdd.mtmdd_mut(), &fnode, pv)
        }
        _ => panic!("Not implemented yet"),
    }
}

pub fn vmddprob<T>(mdd: &mut mtmdd::MtMdd<i64>, node: &mtmdd::MtMddNode<i64>, pv: HashMap<String, Vec<T>>) -> HashMap<i64, T> 
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Clone
        + Copy
        + PartialEq
        + From<f64>,
{
    let cache = &mut HashMap::new();
    _vmddprob(mdd, &node, &pv, cache)
}

fn _vmddprob<T>(
    mdd: &mut mtmdd::MtMdd<i64>,
    node: &mtmdd::MtMddNode<i64>,
    pv: &HashMap<String, Vec<T>>,
    cache: &mut HashMap<NodeId, HashMap<i64, T>>,
) -> HashMap<i64, T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Clone
        + Copy
        + PartialEq
        + From<f64>,
{
    let key = node.id();
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                mtmdd::MtMddNode::Terminal(fnode) => {
                    let mut map = HashMap::new();
                    let value = fnode.value();
                    map.insert(value, T::from(1.0));
                    map
                },
                mtmdd::MtMddNode::NonTerminal(fnode) => {
                    let label = fnode.header().label();
                    let fp = pv.get(label).unwrap();
                    let mut map = HashMap::new();
                    for (i, x) in fnode.iter().enumerate() {
                        let tmp = _vmddprob(mdd, &x, pv, cache);
                        for (k, v) in tmp.iter() {
                            let key = *k;
                            let value = *v;
                            let entry = map.entry(key).or_insert(T::from(0.0));
                            *entry = *entry + fp[i] * value;
                        }
                    }
                    map
                },
                mtmdd::MtMddNode::Undet => HashMap::new(),
            };
            cache.insert(key, result.clone());
            result
        }
    }
}
