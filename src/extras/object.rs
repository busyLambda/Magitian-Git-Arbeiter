use git2::{Blob, Repository, Tree, TreeIter};
use serde::{Deserialize, Serialize};

/*
    TODO: Make two separate functions for this or two iterators
    even since it would be better if one function was looking for one thing.
*/
#[derive(Deserialize, Serialize)]
pub struct TrObject {
    pub id: String,
    pub name: String,
    pub contents: Vec<TB>,
}

impl TrObject {
    // Convert a git2::Tree and a String into a TrObject
    pub fn from_tree(t: Tree, name: String) -> Option<Self> {
        Some(Self {
            id: t.id().to_string(),
            name,
            contents: match TB::from_tree_entries(t.iter()) {
                Some(vtb) => vtb,
                _ => return None,
            },
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct BlObject {
    pub id: String,
    pub name: String,
    pub contents: String,
}

impl BlObject {
    // Create a BlObject from a git2::Blob and a String
    fn from_blob(b: Blob, name: String) -> Self {
        Self {
            id: b.id().to_string(),
            name,
            contents: String::from_utf8_lossy(b.content()).to_string(),
        }
    }
}

pub enum BoTo {
    Tree(TrObject),
    Blob(BlObject),
}

#[derive(Deserialize, Serialize)]
pub enum TB {
    Tree(String),
    Blob(String),
    SubModule(String),
}

#[derive(Debug)]
pub enum Component {
    Tree(String),
    Final(String),
}
impl TB {
    // TODO: Add submodule recognition.
    pub fn from_tree_entries(ti: TreeIter) -> Option<Vec<Self>> {
        let comps: Vec<TB> = ti
            .filter_map(|e| {
                match match e.kind() {
                    Some(e) => e,
                    _ => return None,
                } {
                    git2::ObjectType::Tree => Some(TB::Tree(match e.name() {
                        Some(t) => t.to_string(),
                        _ => return None,
                    })),
                    git2::ObjectType::Blob => Some(TB::Blob(match e.name() {
                        Some(b) => b.to_string(),
                        _ => return None,
                    })),
                    _ => None,
                }
            })
            .collect();

        Some(comps)
    }
}

impl Component {
    // Convert a git2::TreeIter into a Vec<Self>
    // Parse a string as a Vec<Component>
    pub fn from_string(path: String) -> Vec<Self> {
        // Split up the path into a Vec<&str> by dividing it up at the '/' character
        let parts: Vec<&str> = path.split("/").collect();
        // Transform the parts into a Vec<Component>
        let len = parts.len().to_owned();

        /*
            TODO: Redo the Component struct So that instead of Tree and Blob we have
            Final and Tree so that we can use this same thing for the tree route as
            well also we could just have two parse functions for the separate routes.
        */
        parts
            .into_iter()
            .enumerate()
            .map(|(index, part)| {
                if index == len - 1 {
                    Component::Final(part.to_string())
                } else {
                    Component::Tree(part.to_string())
                }
            })
            .collect()
    }
}

// Used to retrieve a blob or a tree based on a path.
/// This will iterate over the components and get the desired component.
pub struct TreeIterator<'a> {
    repo: &'a Repository,
    tree: git2::Tree<'a>,
    components: Vec<Component>,
    index: usize,
}
impl<'a> TreeIterator<'a> {
    pub fn new(repo: &'a Repository, tree: git2::Tree<'a>, components: Vec<Component>) -> Self {
        Self {
            repo,
            tree,
            components,
            index: 0,
        }
    }
}

// TODO: introduce error handling and rewrite some of the parts such as the clone() call on ```self.tree```
impl<'a> Iterator for TreeIterator<'a> {
    type Item = Result<Option<BoTo>, git2::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.components.len() {
            return None;
        }
        // TODO: remove clone()
        let component = &self.components[self.index];
        match component {
            Component::Tree(name) => {
                let tree2 = self.tree.clone();
                let entry = tree2.get_name(name.as_str())?;
                match entry.kind() {
                    Some(git2::ObjectType::Tree) => {
                        self.tree = match self.repo.find_tree(entry.id()) {
                            Ok(o) => o,
                            Err(e) => return Some(Err(e)),
                        };
                        self.index += 1;
                        Some(Ok(None))
                    }
                    _ => Some(Ok(None)),
                }
            }
            Component::Final(name) => {
                let entry = self.tree.get_name(name.as_str())?;
                match entry.kind() {
                    Some(git2::ObjectType::Blob) => {
                        let blob = match self.repo.find_blob(entry.id()) {
                            Ok(o) => o,
                            Err(e) => return Some(Err(e)),
                        };
                        self.index = self.components.len();
                        Some(Ok(Some(BoTo::Blob(BlObject::from_blob(
                            blob,
                            name.as_str().to_string(),
                        )))))
                    }
                    Some(git2::ObjectType::Tree) => {
                        let tree = match self.repo.find_tree(entry.id()) {
                            Ok(o) => o,
                            Err(e) => return Some(Err(e)),
                        };
                        self.index = self.components.len();
                        Some(Ok(Some(BoTo::Tree(
                            match TrObject::from_tree(tree, name.as_str().to_string()) {
                                Some(tr) => tr,
                                _ => return None,
                            },
                        ))))
                    }
                    _ => Some(Ok(None)),
                }
            }
        }
    }
}
