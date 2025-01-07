use dd::{bdd, nodes::NonTerminal};

pub struct BddPath {
    root: bdd::BddNode,
    next_stack: Vec<StackValue>,
    path: Vec<String>,
}

enum StackValue {
    Node(bdd::BddNode),
    Push(String),
    Pop,
}

impl BddPath {
    pub fn new(node: &bdd::BddNode) -> BddPath {
        let root = node.clone();
        let mut next_stack = Vec::new();
        next_stack.push(StackValue::Node(node.clone()));
        let path = Vec::new();
        BddPath {
            root,
            next_stack,
            path,
        }
    }

    pub fn root(&self) -> bdd::BddNode {
        self.root.clone()
    }
}

impl Iterator for BddPath {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.next_stack.pop() {
            match node {
                StackValue::Node(node) => {
                    match node {
                        bdd::BddNode::Zero => (),
                        bdd::BddNode::One => {
                            let mut result = self.path.clone();
                            result.reverse();
                            return Some(result);
                        }
                        bdd::BddNode::NonTerminal(fnode) => {
                            let x = fnode.header().label();
                            self.next_stack.push(StackValue::Pop);
                            self.next_stack.push(StackValue::Node(fnode[1].clone()));
                            self.next_stack.push(StackValue::Push(x.to_string()));
                            self.next_stack.push(StackValue::Node(fnode[0].clone()));
                        }
                    }
                }
                StackValue::Push(x) => self.path.push(x),
                StackValue::Pop => {
                    self.path.pop();
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dd::bdd::Bdd;
    use dd::bdd::BddNode;

    #[test]
    fn test_bdd_path() {
        let mut bdd = Bdd::new();
        let x = bdd.header(0, "x");
        let y = bdd.header(1, "y");
        let z = bdd.header(2, "z");
        let x0 = bdd.zero();
        let x1 = bdd.one();
        let y0 = bdd.create_node(&y, &x0, &x1);
        let y1 = bdd.create_node(&y, &x1, &x0);
        let z0 = bdd.create_node(&z, &y0, &y1);
        let z1 = bdd.create_node(&z, &y1, &y0);
        let root = bdd.create_node(&x, &z0, &z1);
        let path = BddPath::new(&root);
        let mut count = 0;
        for p in path {
            count += 1;
            println!("{:?}", p);
        }
        assert_eq!(count, 4);
    }
}