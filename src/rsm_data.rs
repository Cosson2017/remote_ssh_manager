use std::cmp::*;
use std::ops::Deref;

use failure::{bail, ensure, format_err, Error, Fail, Fallible};

macro_rules! return_err {
    ($e: expr) => {
        return Err(Error::from($e));
    };
}

#[derive(Debug, Fail)]
enum NodeErr {
    #[fail(display = "node => {} is not dir", _0)]
    NotDir(String),
    #[fail(display = "node => {} already exist", _0)]
    Exist(String),
    #[fail(display = "node => {} is not exist", _0)]
    NotExist(String),
    #[fail(display = "path => {} is err", _0)]
    PathErr(String),
    #[fail(display = "name => {} is err", _0)]
    NameErr(String),
}

use NodeErr::*;

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

    fn walk(dir: &String, node: &InfoNode, f: &mut impl FnMut(&String, &InfoNode)) {
        f(dir, node);
        use InfoNode::*;
        if let Dir(name, is_expand, node_vec) = node {
            if !is_expand {
                return;
            }

            let mut next_dir = dir.clone();
            next_dir.push('/');
            next_dir.push_str(name);

            for next_node in node_vec {
                Self::walk(&next_dir, next_node, f);
            }
        }
    }

    fn get_or_insert_with(&mut self, name: &str, f: impl Fn() -> InfoNode) -> Fallible<&mut Self> {
        match self {
            InfoNode::Info(name) => return_err!(NotDir(name.clone())),
            InfoNode::Dir(.., node_vec) => {
                match node_vec.binary_search_by(|bn| name.cmp(bn.get_name())) {
                    Ok(index) => Ok(unsafe { node_vec.get_unchecked_mut(index) }),
                    Err(index) => {
                        let new_node = f();
                        if new_node.get_name() != name {
                            return_err!(NameErr(name.to_string()))
                        }
                        node_vec.insert(index, new_node);
                        Ok(unsafe { node_vec.get_unchecked_mut(index) })
                    }
                }
            }
        }
    }

    fn insert(&mut self, node: InfoNode) -> Fallible<()> {
        use InfoNode::*;
        match self {
            Info(name) => return_err!(NotDir(name.clone())),
            Dir(.., node_vec) => match node_vec.binary_search_by(|bn| node.cmp(bn)) {
                Ok(_) => return_err!(Exist(node.get_name().clone())),
                Err(index) => node_vec.insert(index, node),
            },
        };
        Ok(())
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Self> {
        use InfoNode::*;
        match self {
            Info(_) => None,
            Dir(.., node_vec) => match node_vec.binary_search_by(|bn| name.cmp(bn.get_name())) {
                Err(_) => None,
                Ok(index) => Some(unsafe { node_vec.get_unchecked_mut(index) }),
            },
        }
    }

    fn remove(&mut self, name: &str) {
        use InfoNode::*;
        match self {
            Info(_) => return,
            Dir(.., node_vec) => match node_vec.binary_search_by(|bn| name.cmp(bn.get_name())) {
                Err(_) => return,
                Ok(index) => node_vec.remove(index),
            },
        };
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

pub enum NodeView {
    InfoType(String),
    DirType(String),
}

struct InfoTree {
    view: Vec<NodeView>,
    root: InfoNode,
}

impl InfoTree {
    fn visit(&mut self) {
        let mut view = Vec::new();
        InfoNode::walk(&"".into(), &self.root, &mut |dir, node| {
            use InfoNode::*;
            use NodeView::*;

            let mut path = dir.clone();
            path.push_str(node.get_name());
            match node {
                Info(..) => view.push(InfoType(path)),
                Dir(..) => view.push(DirType(path)),
            }
        });
        self.view = view;
    }

    fn insert(&mut self, path: String, is_dir: bool) -> Fallible<()> {
        let path_vec: Vec<&str> = path.split('/').filter(|node| node != &"").collect();
        let path_len = path_vec.len();
        ensure!(path_len > 0, format_err!("path error: {}", path));
        let last_index = path_len - 1;
        let mut cur_node = &mut self.root;
        for pn in path_vec[0..last_index].iter() {
            cur_node = cur_node.get_or_insert_with(pn.deref(), || {
                InfoNode::Dir(pn.to_string(), true, Vec::new())
            })?;
        }

        let name = unsafe { path_vec.get_unchecked(last_index) };
        let new_node = if is_dir {
            InfoNode::Dir(name.to_string(), true, Vec::new())
        } else {
            InfoNode::Info(name.to_string())
        };
        cur_node.insert(new_node)?;
        Ok(())
    }

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
            if let Some(nn) = node_iter.get_mut(old.deref()) {
                node_iter = nn;
                continue
            }
            return_err!(NotExist(old.to_string()))
            //match node_iter {
            //    InfoNode::Info(name) => return Err(format_err!("node {} is info, need dir", name)),
            //    InfoNode::Dir(_, _, node_vec) => {
            //        match node_vec.binary_search_by(|n| old.deref().cmp(n.get_name())) {
            //            Err(_) => return Err(format_err!("node {} is not exist", old)),
            //            Ok(index) => node_iter = unsafe { node_vec.get_unchecked_mut(index) },
            //        }
            //    }
            //}
        }
        node_iter.remove(unsafe{old_path_vec.get_unchecked(old_last).deref()});

//        match node_iter {
//            InfoNode::Info(name) => return Err(format_err!("node {} is info, need dir", name)),
//            InfoNode::Dir(_, _, node_vec) => {
//                node_vec
//                    .binary_search_by(|n| unsafe {
//                        old_path_vec
//                            .get_unchecked(old_last)
//                            .deref()
//                            .cmp(n.get_name())
//                    })
//                    .and_then(|index| {
//                        node_vec.remove(index);
//                        Ok(())
//                    });
//            }
//        }

        self.insert(new_path, is_dir)
    }
}
