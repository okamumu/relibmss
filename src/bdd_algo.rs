// mod ft

use std::collections::HashMap;
use std::ops::{Add, Mul, MulAssign, Sub};
use std::result;

use dd::bdd::{self, Bdd};
use dd::common::Level;
use dd::common::NodeId;
use dd::nodes::NodeHeader;
use dd::nodes::NonTerminal;

use crate::bdd::BddNode;

pub fn prob<T>(
    bdd: &mut bdd::Bdd,
    node: &bdd::BddNode,
    pv: &HashMap<String, T>,
    cache: &mut HashMap<NodeId, T>,
) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone + Copy + PartialEq + From<f64>,
{
    let key = node.id();
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                bdd::BddNode::Zero => T::from(0.0),
                bdd::BddNode::One => T::from(1.0),
                bdd::BddNode::NonTerminal(fnode) => {
                    let x = fnode.header().label();
                    let fp = *pv.get(x).unwrap_or(&T::from(0.0));
                    let low = prob(bdd, &fnode[0], pv, cache);
                    let high = prob(bdd, &fnode[1], pv, cache);
                    (T::from(1.0) - fp) * low + fp * high
                }
            };
            cache.insert(key, result);
            result
        }
    }
}

pub fn minsol(
    dd: &mut bdd::Bdd,
    node: &bdd::BddNode,
    cache1: &mut HashMap<NodeId, bdd::BddNode>,
    cache2: &mut HashMap<(NodeId, NodeId), bdd::BddNode>,
) -> bdd::BddNode {
    let key = node.id();
    match cache1.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                bdd::BddNode::Zero => dd.zero(),
                bdd::BddNode::One => dd.one(),
                bdd::BddNode::NonTerminal(fnode) => {
                    let tmp = minsol(dd, &fnode[1], cache1, cache2);
                    let high = bdd_without(dd, &tmp, &fnode[0], cache2);
                    let low = minsol(dd, &fnode[0], cache1, cache2);
                    dd.create_node(fnode.header(), &low, &high)
                }
            };
            cache1.insert(key, result.clone());
            result
        }
    }
}

enum BddStackValue<'a, 'b> {
    Bdd2(&'a bdd::BddNode, &'a bdd::BddNode),
    BddHeader((NodeId, NodeId), &'b dd::nodes::NodeHeader),
}

// pub fn minsol_stack(
//     dd: &mut bdd::Bdd,
//     node: &bdd::BddNode,
//     cache1: &mut HashMap<NodeId, bdd::BddNode>,
//     cache2: &mut HashMap<(NodeId, NodeId), bdd::BddNode>,
// ) -> bdd::BddNode {
//     let mut next_stack = Vec::with_capacity(2048);
//     let mut result_stack = Vec::with_capacity(2048);
//     next_stack.push(node);
//     while let Some(x) = next_stack.pop() {
//         if let Some(result) = cache1.get(&x.id()) {
//             result_stack.push(result.clone());
//             continue;
//         }
//         match x {
//             bdd::BddNode::Zero => {
//                 let result = dd.zero();
//                 result_stack.push(result.clone());
//             }
//             bdd::BddNode::One => {
//                 let result = dd.one();
//                 result_stack.push(result.clone());
//             }
//             bdd::BddNode::NonTerminal(fnode) => {
//                 next_stack.push(&fnode[0]);
//                 next_stack.push(&fnode[1]);
//                 next_stack.push(fnode);
//             }
//         }
//     }
//     let key = node.id();
//     match cache1.get(&key) {
//         Some(x) => x.clone(),
//         None => {
//             let result = match node {
//                 bdd::BddNode::Zero => dd.zero(),
//                 bdd::BddNode::One => dd.one(),
//                 bdd::BddNode::NonTerminal(fnode) => {
//                     let tmp = minsol(dd, &fnode[1], cache1, cache2);
//                     let high = bdd_without_stack(dd, &tmp, &fnode[0], cache2);
//                     let low = minsol(dd, &fnode[0], cache1, cache2);
//                     dd.create_node(fnode.header(), &low, &high)
//                 }
//             };
//             cache1.insert(key, result.clone());
//             result
//         }
//     }
// }

pub fn bdd_without_stack(
    dd: &mut bdd::Bdd,
    f: &bdd::BddNode, // minsol tree
    g: &bdd::BddNode,
    cache: &mut HashMap<(NodeId, NodeId), bdd::BddNode>,
) -> bdd::BddNode {
    let mut next_stack = Vec::new(); //with_capacity(2048);
    let mut result_stack = Vec::new(); //with_capacity(2048);
    next_stack.push(BddStackValue::Bdd2(f, g));
    while let Some(stackvalue) = next_stack.pop() {
        match stackvalue {
            BddStackValue::BddHeader(key, header) => {
                let high = result_stack.pop().unwrap();
                let low = result_stack.pop().unwrap();
                let result = dd.create_node(&header, &low, &high);
                cache.insert(key, result.clone());
                result_stack.push(result.clone());
            }
            BddStackValue::Bdd2(f, g) => {
                let key = (f.id(), g.id());
                if let Some(x) = cache.get(&key) {
                    result_stack.push(x.clone());
                    continue;
                }
                match (f, g) {
                    (bdd::BddNode::Zero, _) => {
                        let result = dd.zero();
                        cache.insert(key, result.clone());
                        result_stack.push(result.clone());
                    }
                    (_, bdd::BddNode::Zero) => {
                        let result = f;
                        cache.insert(key, result.clone());
                        result_stack.push(result.clone());
                    }
                    (_, bdd::BddNode::One) => {
                        let result = dd.zero();
                        cache.insert(key, result.clone());
                        result_stack.push(result.clone());
                    }
                    (bdd::BddNode::One, bdd::BddNode::NonTerminal(gnode)) => {
                        next_stack.push(BddStackValue::BddHeader(key, gnode.header()));
                        next_stack.push(BddStackValue::Bdd2(f, &gnode[1]));
                        next_stack.push(BddStackValue::Bdd2(f, &gnode[0]));
                    }
                    (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode))
                        if fnode.id() == gnode.id() =>
                    {
                        let result = dd.zero();
                        cache.insert(key, result.clone());
                        result_stack.push(result.clone());
                    }
                    (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode))
                        if fnode.level() > gnode.level() =>
                    {
                        next_stack.push(BddStackValue::BddHeader(key, fnode.header()));
                        next_stack.push(BddStackValue::Bdd2(&fnode[1], g));
                        next_stack.push(BddStackValue::Bdd2(&fnode[0], g));
                    }
                    (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode))
                        if fnode.level() < gnode.level() =>
                    {
                        next_stack.push(BddStackValue::Bdd2(f, &gnode[0]));
                    }
                    (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode)) => {
                        next_stack.push(BddStackValue::BddHeader(key, fnode.header()));
                        next_stack.push(BddStackValue::Bdd2(&fnode[1], &gnode[1]));
                        next_stack.push(BddStackValue::Bdd2(&fnode[0], &gnode[0]));
                    }
                }
            }
        }
    }
    if let Some(node) = result_stack.pop() {
        node.clone()
    } else {
        panic!("result stack is empty");
    }
}

pub fn bdd_without(
    dd: &mut bdd::Bdd,
    f: &bdd::BddNode, // minsol tree
    g: &bdd::BddNode,
    cache: &mut HashMap<(NodeId, NodeId), bdd::BddNode>,
) -> bdd::BddNode {
    let key = (f.id(), g.id());
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let node = match (f, g) {
                (bdd::BddNode::Zero, _) => dd.zero(),
                (_, bdd::BddNode::Zero) => f.clone(),
                (_, bdd::BddNode::One) => dd.zero(),
                (bdd::BddNode::One, bdd::BddNode::NonTerminal(gnode)) => {
                    let low = bdd_without(dd, f, &gnode[0], cache);
                    let high = bdd_without(dd, f, &gnode[1], cache);
                    dd.create_node(gnode.header(), &low, &high)
                }
                (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode))
                    if fnode.id() == gnode.id() =>
                {
                    dd.zero()
                }
                (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode))
                    if fnode.level() > gnode.level() =>
                {
                    let low = bdd_without(dd, &fnode[0], g, cache);
                    let high = bdd_without(dd, &fnode[1], g, cache);
                    dd.create_node(fnode.header(), &low, &high)
                }
                (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode))
                    if fnode.level() < gnode.level() =>
                {
                    bdd_without(dd, f, &gnode[0], cache)
                }
                (bdd::BddNode::NonTerminal(fnode), bdd::BddNode::NonTerminal(gnode)) => {
                    let low = bdd_without(dd, &fnode[0], &gnode[0], cache);
                    let high = bdd_without(dd, &fnode[1], &gnode[1], cache);
                    dd.create_node(fnode.header(), &low, &high)
                }
            };
            cache.insert(key, node.clone());
            node
        }
    }
}

pub fn count_set<T>(node: &bdd::BddNode, cache: &mut HashMap<NodeId, T>) -> T
where
    T: Add<Output = T> + Clone + From<u32>,
{
    let key = node.id();
    match cache.get(&key) {
        Some(x) => x.clone(),
        None => {
            let result = match node {
                bdd::BddNode::One => T::from(1),
                bdd::BddNode::Zero => T::from(0),
                bdd::BddNode::NonTerminal(fnode) => {
                    count_set(&fnode[0], cache) + count_set(&fnode[1], cache)
                }
            };
            cache.insert(key, result.clone());
            result
        }
    }
}

pub fn extract(node: &bdd::BddNode, path: &mut Vec<String>, pathset: &mut Vec<Vec<String>>) {
    match node {
        bdd::BddNode::Zero => (),
        bdd::BddNode::One => pathset.push(path.clone()),
        bdd::BddNode::NonTerminal(fnode) => {
            let x = fnode.header().label();
            path.push(x.to_string());
            extract(&fnode[1], path, pathset);
            path.pop();
            extract(&fnode[0], path, pathset);
        }
    }
}
