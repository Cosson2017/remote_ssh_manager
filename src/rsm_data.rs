use std::cmp::*;
use std::ops::Deref;

use failure::{bail, ensure, format_err, Error, Fallible};

macro_rules! search_then {
    () => {};
}

#[derive(Eq)]
pub enum InfoNode {
    Info(String),
    Dir(String, bool, Vec<InfoNode>),
}

impl InfoNode {
    fn get_name(&self) -> &String {
        match self {
            InfoNode::Info(name) => name,
            InfoNode::Dir(name, ..) => name,
        }
    }
}

impl PartialEq for InfoNode {
    fn eq(&self, other: &InfoNode) -> bool {
        self.get_name() == other.get_name()
    }
}

impl PartialOrd for InfoNode {
    fn partial_cmp(&self, other: &InfoNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InfoNode {
    fn cmp(&self, other: &InfoNode) -> Ordering {
        self.get_name().cmp(other.get_name())
    }
}

struct InfoTree {
    view: Vec<String>,
    root: InfoNode,
}

impl InfoTree {
    fn insert(&mut self, path: String, is_dir: bool) -> Fallible<()> {
        let path_vec: Vec<&str> = path.split('/').filter(|node| node != &"").collect();
        let path_len = path_vec.len();
        ensure!(path_len > 0, format_err!("path error: {}", path));
        let last_index = path_len - 1;
        let mut cur_node = &mut self.root;
        for pn in path_vec[0..last_index].iter() {
            match cur_node {
                InfoNode::Info(name) => {
                    return Err(format_err!("node: {} is info node, need dir node", name))
                }
                InfoNode::Dir(_, _, node_vec) => {
                    match node_vec.binary_search_by(|node| pn.deref().cmp(node.get_name())) {
                        Ok(index) => {
                            cur_node = unsafe { node_vec.get_unchecked_mut(index) };
                        }
                        Err(index) => {
                            node_vec.insert(index, InfoNode::Dir(pn.to_string(), true, Vec::new()));
                            cur_node = unsafe { node_vec.get_unchecked_mut(index) };
                        }
                    }
                }
            }
        }

        match cur_node {
            InfoNode::Info(name) => {
                return Err(format_err!("node: {} is info node, need dir node", name))
            }
            InfoNode::Dir(_, _, node_vec) => {
                let name = unsafe { path_vec.get_unchecked(last_index) };
                match node_vec.binary_search_by(|node| name.deref().cmp(node.get_name())) {
                    Ok(_) => return Err(format_err!("path exist")),
                    Err(index) => {
                        let inserted_node = if is_dir {
                            InfoNode::Dir(name.to_string(), true, Vec::new())
                        } else {
                            InfoNode::Info(name.to_string())
                        };
                        node_vec.insert(index, inserted_node);
                    }
                }
            }
        }

        Ok(())
    }

    fn visit(&self) {}

    fn update(&mut self, old_path: String, new_path: String, is_dir: bool) -> Fallible<()> {
        // 先把旧的删了
        let old_path_vec: Vec<&str> = old_path.split('/').filter(|o| o != &"").collect();
        let old_len = old_path_vec.len();
        if old_len == 0 {
            return self.insert(new_path, is_dir);
        }
        let old_last = old_len - 1;
        let mut node_iter = &mut self.root;
        for old in old_path_vec[..old_last].iter() {
            match node_iter {
                InfoNode::Info(name) => return Err(format_err!("node {} is info, need dir", name)),
                InfoNode::Dir(_, _, node_vec) => {
                    match node_vec.binary_search_by(|n| old.deref().cmp(n.get_name())) {
                        Err(_) => return Err(format_err!("node {} is not exist", old)),
                        Ok(index) => node_iter = unsafe { node_vec.get_unchecked_mut(index) },
                    }
                }
            }
        }

        match node_iter {
            InfoNode::Info(name) => return Err(format_err!("node {} is info, need dir", name)),
            InfoNode::Dir(_, _, node_vec) => {

 //               #[allow(unused)]
                node_vec
                    .binary_search_by(|n| unsafe {
                        old_path_vec
                            .get_unchecked(old_last)
                            .deref()
                            .cmp(n.get_name())
                    })
                    .and_then(|index| {
                        node_vec.remove(index);
                        Ok(())
                    });
            }
        }

        self.insert(new_path, is_dir)
    }
}
