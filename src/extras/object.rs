use git2::Repository;

pub enum Component {
    Tree(String),
    Blob(String),
}

impl Component {
    fn parse(path: String) -> Vec<Self> {
        let parts: Vec<&str> = path.split("/").collect();
        parts
            .into_iter()
            .enumerate()
            .map(|(index, part)| {
                if index == parts.len() {
                    Component::Blob(part.to_string())
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
    type Item = Result<Option<String>, git2::Error>;

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
            Component::Blob(name) => {
                let entry = self.tree.get_name(name.as_str()).unwrap();
                match entry.kind() {
                    Some(git2::ObjectType::Blob) => {
                        let blob = self.repo.find_blob(entry.id()).unwrap();
                        let content = String::from_utf8_lossy(blob.content()).to_string();
                        self.index = self.components.len();
                        Some(Ok(Some(content)))
                    }
                    _ => Some(Ok(None)),
                }
            }
        }
    }
}
