use serde::Serialize;
use std::cmp::Reverse;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize)]
pub struct TreeNode<T>
where
    T: Clone + Default + Debug,
{
    #[serde(flatten)]
    item: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T>
where
    T: Clone + Default + Debug,
{
    pub fn build_tree<F1, F2, F3>(
        items: Vec<T>,
        root: String,
        get_key: F1,
        get_parent_key: F2,
        sort: F3,
    ) -> Vec<TreeNode<T>>
    where
        F1: Fn(&T) -> String,
        F2: Fn(&T) -> String,
        F3: Fn(&T) -> i64,
    {
        let mut map = std::collections::HashMap::new();
        for item in items {
            map.entry(get_parent_key(&item))
                .or_insert_with(Vec::new)
                .push(TreeNode {
                    item,
                    children: Vec::new(),
                });
        }
        for (_, children) in map.iter_mut() {
            children.sort_by_key(|child| sort(&child.item));
        }
        Self::build(&mut map, root, &get_key, &sort)
    }

    pub fn build(
        map: &mut std::collections::HashMap<String, Vec<TreeNode<T>>>,
        root: String,
        key: &impl Fn(&T) -> String,
        sort: &impl Fn(&T) -> i64,
    ) -> Vec<TreeNode<T>> {
        map.remove(&root)
            .map(|mut children| {
                children.sort_by_key(|child| Reverse(sort(&child.item)));
                for child in &mut children {
                    child.children = Self::build(map, key(&child.item), key, sort);
                }
                children
            })
            // .unwrap_or(Vec::new())
            .unwrap_or_default() //unwrap_or和unwrap_or_default都可以，采用unwrap_or_default可以避免创建一个空的Vec对象作为默认值
    }
}
