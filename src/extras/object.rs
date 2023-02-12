use git2::{Blob, Repository, Tree, TreeIter};

/*
    TODO: Make two separate functions for this or two iterators
    even since it would be better if one function was looking for one thing.
*/
pub struct TrObject {
    pub id: String,
    pub name: String,
    pub contents: Vec<TB>,
}

impl TrObject {
    // Convert a git2::Tree and a String into a TrObject
    fn from_tree(t: Tree, name: String) -> Self {
        Self {
            id: t.id().to_string(),
            name,
            contents: TB::from_tree_entries(t.iter())
        }
    }
}

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

pub enum BT<'a> {
    Tree(Tree<'a>),
    Blob(Blob<'a>),
}

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
    pub fn from_tree_entries(ti: TreeIter) -> Vec<Self> {
        let comps: Vec<TB> = ti
            .filter_map(|e| match e.kind().unwrap() {
                git2::ObjectType::Tree => Some(TB::Tree(e.name().unwrap().to_string())),
                git2::ObjectType::Blob => Some(TB::Blob(e.name().unwrap().to_string())),
                _ => None,
            })
            .collect();

        comps
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
    type Item = Result<Option<BT<'a>>, git2::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.components.len() {
            return None;
        }
        // TODO: remove clone()
        let component = &self.components[self.index];
        match component {
            Component::Tree(name) => {
                let tree2 = self.tree.clone();
                let entry = tree2.get_name(name.as_str()).unwrap();
                match entry.kind() {
                    Some(git2::ObjectType::Tree) => {
                        self.tree = self.repo.find_tree(entry.id()).unwrap();
                        self.index += 1;
                        Some(Ok(None))
                    }
                    _ => Some(Ok(None)),
                }
            }
            Component::Final(name) => {
                let entry = self.tree.get_name(name.as_str()).unwrap();
                match entry.kind() {
                    Some(git2::ObjectType::Blob) => {
                        let blob = self.repo.find_blob(entry.id()).unwrap();
                        self.index = self.components.len();
                        Some(Ok(Some(BT::Blob(blob))))
                    }
                    Some(git2::ObjectType::Tree) => {
                        let tree = self.repo.find_tree(entry.id()).unwrap();
                        self.index = self.components.len();
                        Some(Ok(Some(BT::Tree(tree))))
                    }
                    _ => Some(Ok(None)),
                }
            }
        }
    }
}
