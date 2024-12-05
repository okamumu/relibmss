// mod ft

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use dd::bdd::*;
use dd::common::NodeId;
use dd::common::Level;
use dd::nodes::NonTerminal;
use pyo3::pyclass;

pub fn kofn(bdd: &mut Bdd, k: usize, nodes: Vec<BddNode>) -> BddNode {
    match k {
        1 => _or(bdd, nodes),
        _ if nodes.len() == k => _and(bdd, nodes),
        _ => {
            let tmp1 = kofn(bdd, k - 1, nodes[1..].to_vec());
            let tmp2 = kofn(bdd, k, nodes[1..].to_vec());
            bdd.ite(&nodes[0], &tmp1, &tmp2)
        }
    }
}

pub fn _and(bdd: &mut Bdd, nodes: Vec<BddNode>) -> BddNode {
    let mut res = bdd.one();
    for node in nodes.iter() {
        res = bdd.and(&res, &node);
    }
    res
}

pub fn _or(bdd: &mut Bdd, nodes: Vec<BddNode>) -> BddNode {
    let mut res = bdd.zero();
    for node in nodes.iter() {
        res = bdd.or(&res, &node);
    }
    res
}

// prob
pub fn prob(bdd: &mut Bdd, node: &BddNode, pv: HashMap<String,f64>) -> f64 {
    let cache = &mut HashMap::new();
    _prob(bdd, &node, &pv, cache)
}

fn _prob(bdd: &mut Bdd, node: &BddNode, pv: &HashMap<String,f64>, cache: &mut HashMap<NodeId,f64>) -> f64 {
    let key = node.id();
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                BddNode::Zero => 1.0,
                BddNode::One => 0.0,
                BddNode::NonTerminal(fnode) => {
                    let x = fnode.header().label();
                    let fp = pv.get(x).unwrap_or(&0.0).clone();
                    let low = _prob(bdd, &fnode[0], pv, cache);
                    let high = _prob(bdd, &fnode[1], pv, cache);
                    fp * low + (1.0 - fp) * high
                },
            };
            cache.insert(key, result);
            result
        }
    }
}

pub fn minsol(bdd: &mut Bdd, node: &BddNode) -> BddNode {
    let cache1 = &mut HashMap::new();
    let cache2 = &mut HashMap::new();
    _minsol(bdd, &node, cache1, cache2)
}

fn _minsol(bdd: &mut Bdd, node: &BddNode, cache1: &mut HashMap<NodeId,BddNode>, cache2: &mut HashMap<(NodeId,NodeId),BddNode>) -> BddNode {
    let key = node.id();
    match cache1.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                BddNode::Zero => bdd.zero(),
                BddNode::One => bdd.one(),
                BddNode::NonTerminal(fnode) => {
                    let x1 = _minsol(bdd, &fnode[1], cache1, cache2);
                    let high = without(bdd, &x1, &fnode[0], cache2);
                    let low = _minsol(bdd, &fnode[0], cache1, cache2);
                    bdd.node(fnode.header(), &vec![low, high]).ok().unwrap()
                },
            };
            cache1.insert(key, result.clone());
            result
        }
    }
}

fn without(bdd: &mut Bdd, f: &BddNode, g: &BddNode, cache: &mut HashMap<(NodeId,NodeId),BddNode>) -> BddNode {
    let key = (f.id(), g.id());
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match (f, g) {
                (BddNode::Zero, _) => bdd.zero(),
                (_, BddNode::One) => bdd.zero(),
                (_, BddNode::Zero) => f.clone(),
                (BddNode::One, _) => bdd.not(&g),
                (BddNode::NonTerminal(fnode), BddNode::NonTerminal(gnode)) if fnode.level() > gnode.level() => {
                    let low = without(bdd, &fnode[0], g, cache);
                    let high = without(bdd, &fnode[1], g, cache);
                    bdd.node(fnode.header(), &vec![low, high]).ok().unwrap()
                },
                (BddNode::NonTerminal(fnode), BddNode::NonTerminal(gnode)) if fnode.level() < gnode.level() => {
                    let low = without(bdd, f, &gnode[0], cache);
                    let high = without(bdd, f, &gnode[1], cache);
                    bdd.node(fnode.header(), &vec![low, high]).ok().unwrap()
                },
                (BddNode::NonTerminal(fnode), BddNode::NonTerminal(gnode)) if fnode.level() == gnode.level() => {
                    let low = without(bdd, &fnode[0], &gnode[0], cache);
                    let high = without(bdd, &fnode[1], &gnode[1], cache);
                    bdd.node(fnode.header(), &vec![low, high]).ok().unwrap()
                },
                _ => panic!("Unexpected case"),
            };
            cache.insert(key, result.clone());
            result
        }
    }
}

pub fn extract(bdd: &mut Bdd, node: &BddNode) -> Vec<Vec<String>> {
    let mut pathset = Vec::new();
    _extract(node, &mut Vec::new(), &mut pathset);
    pathset
}

fn _extract(node: &BddNode, path: &mut Vec<String>, pathset: &mut Vec<Vec<String>>) {
    match node {
        BddNode::Zero => (),
        BddNode::One => pathset.push(path.clone()),
        BddNode::NonTerminal(fnode) => {
            let x = fnode.header().label();
            path.push(x.to_string());
            _extract(&fnode[1], path, pathset);
            path.pop();
            _extract(&fnode[0], path, pathset);
        },
    }
}
