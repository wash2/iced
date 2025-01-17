use std::sync::Arc;

use crate::A11yNode;

#[derive(Debug, Clone, Default)]
/// Accessible tree of nodes
pub struct A11yTree {
    /// The root of the current widget, children of the parent widget or the Window if there is no parent widget
    root: Vec<A11yNode>,
    /// The children of a widget and its children
    children: Vec<A11yNode>,
}

impl A11yTree {
    /// Create a new A11yTree
    pub fn new(root: Vec<A11yNode>, children: Vec<A11yNode>) -> Self {
        Self { root, children }
    }

    /// Helper for creating a leaf node
    /// This is a single node with no children
    pub fn leaf_node(node: A11yNode) -> Self {
        Self {
            root: vec![node],
            children: vec![],
        }
    }

    /// Helper for creating an A11y tree with a single root node and some children
    pub fn node_with_child_tree(mut root: A11yNode, child_tree: Self) -> Self {
        root.add_children(
            child_tree.root.iter().map(|n| n.id()).cloned().collect(),
        );
        Self {
            root: vec![root],
            children: child_tree
                .children
                .into_iter()
                .chain(child_tree.root)
                .collect(),
        }
    }

    /// Joins multiple trees into a single tree
    pub fn join<T: Iterator<Item = Self>>(trees: T) -> Self {
        trees.fold(Self::default(), |mut acc, A11yTree { root, children }| {
            acc.root.extend(root);
            acc.children.extend(children);
            acc
        })
    }

    pub fn root(&self) -> &Vec<A11yNode> {
        &self.root
    }

    pub fn children(&self) -> &Vec<A11yNode> {
        &self.children
    }

    pub fn root_mut(&mut self) -> &mut Vec<A11yNode> {
        &mut self.root
    }

    pub fn children_mut(&mut self) -> &mut Vec<A11yNode> {
        &mut self.children
    }
}

impl Into<Vec<(accesskit::NodeId, Arc<accesskit::Node>)>> for A11yTree {
    fn into(self) -> Vec<(accesskit::NodeId, Arc<accesskit::Node>)> {
        self.root
            .into_iter()
            .map(|node| node.into())
            .chain(self.children.into_iter().map(|node| node.into()))
            .collect()
    }
}
