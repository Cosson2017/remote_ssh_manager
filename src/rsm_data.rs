use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
//use std::ops::Deref;

#[derive(Clone)]
enum InfoNode {
    Info(String),
    Dir(String, bool, Rc<RefCell<Vec<InfoNode>>>),
}

impl InfoNode {
    fn get_node_name(&self) -> &String {
        match self {
            InfoNode::Info(name) => name,
            InfoNode::Dir(name, _, _) => name,
        }
    }

    fn set_node_name(&mut self, name: String) {
        match self {
            InfoNode::Info(sn) => *sn = name,
            InfoNode::Dir(dn, _, _) => *dn = name,
        }
    }

    /// 打开目录
    fn expand_dir(&mut self) {
        if let InfoNode::Dir(_, is_expand, _) = self {
            *is_expand = true;
        }
    }
}

struct InfoTree {
    view: Vec<String>,
    root: InfoNode,
}

impl InfoTree {
    /// return is_continue walk 是否继续遍历
    /// F: FnMut(pwd, cur_node) -> is_continue
    /// pwd: 当前节点的目录， cur_node: 当前节点 is_continue: 是否继续遍历
    fn walk_node<F>(pwd: &String, node: &mut InfoNode, f: &mut F) -> bool
    where
        F: FnMut(&String, &mut InfoNode) -> bool,
    {
        if pwd != "" && !f(pwd, node) {
            return false;
        }

        if let InfoNode::Dir(name, is_expand, node_vec) = node {
            if !*is_expand {
                return true;
            }

            let mut pwd = pwd.clone();
            pwd.push('/');
            pwd.push_str(name);
            for node in node_vec.borrow_mut().iter_mut() {
                if !InfoTree::walk_node(&pwd, node, f) {
                    return false;
                }
            }
        }
        true
    }

    /// 更新视图
    fn update_view(&mut self) {
        let root = &mut self.root;
        let view = &mut self.view;
        InfoTree::walk_node(&"".into(), root, &mut |pwd, node| {
            view.push(pwd.clone() + node.get_node_name());
            true
        });
    }

    /// node: 需要插入的节点
    /// view_index: 插入的位置在view中的索引
    /// return： 是否成功
    fn insert_node(&mut self, elem: InfoNode, view_index: isize) {
        let root = &mut self.root;
        let mut cur_index = view_index;
        let mut map_node = HashMap::new();
        if let InfoNode::Dir(_, _, rc_node_vec) = root {
            map_node.insert(String::from("/"), rc_node_vec.clone());
        }
        InfoTree::walk_node(&"".into(), root, &mut |pwd, node| {
            if cur_index > 0 {
                if let InfoNode::Dir(name, is_expand, nv) = node {
                    if *is_expand {
                        let mut pwd = pwd.clone();
                        pwd.push('/');
                        pwd.push_str(name);
                        map_node.entry(pwd).or_insert(nv.clone());
                    }
                }
                cur_index -= 1;
                return true;
            }

            let node_vec = map_node.get_mut(pwd).unwrap();
            node_vec.borrow_mut().push(elem.clone());
            false
        });
        self.update_view();
    }

    /// 更新节点(重命名)
    /// 通过view索引去定位元素
    /// return: 是否成功
    fn update_node(&mut self, node_name: &String, view_index: isize) {
        let mut index = view_index;
        let root = &mut self.root;
        InfoTree::walk_node(&"".into(), root, &mut |_, node| {
            if index > 0 {
                index -= 1;
                return true
            }
            
            node.set_node_name(node_name.clone());
            false
        });
        self.update_view();
    }

    /// 打开目录
    fn expand_dir(&mut self, view_index: isize) {
        let mut index = view_index;
        let root = &mut self.root;
        InfoTree::walk_node(&"".into(), root, &mut |_, node| {
            if index > 0 {
                index -= 1;
                return true
            }

            node.expand_dir();
            false
        });
        self.update_view();
    }

    /// 通过视图索引删除节点
    /// return: 是否成功
    fn delete_node_by_index(&mut self, view_index: isize) {
        let mut index = view_index;
        let root  = &mut self.root;
        let mut dir_cache = HashMap::new();
        let mut index_cache = HashMap::new();
        InfoTree::walk_node(&"".into(), root, &mut |pwd, node| {
            if index > 0 {
                index -= 1;
                if let InfoNode::Dir(name, is_expand, node_vec) = node {
                    if ! *is_expand {
                        return true
                    }

                    let mut node_pwd = pwd.clone();
                    node_pwd.push('/');
                    node_pwd.push_str(name);
                    dir_cache.insert(node_pwd.clone(), node_vec.clone());
                    index_cache.insert(node_pwd, 0);
                }
                *index_cache.get_mut(pwd).unwrap() += 1;
                return true
            }

            // 删除
            let index = *index_cache.get_mut(pwd).unwrap();
            let v = dir_cache.get_mut(pwd).unwrap();
            v.borrow_mut().remove(index);
            false
        });
        self.update_view();
    }
}
