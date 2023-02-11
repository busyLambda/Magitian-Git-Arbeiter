use git2::Repository;

pub enum Component {
    Tree(String),
    Blob(String),
}

impl Component {
    fn parse(path: String) -> Vec<Self> {
        todo!()
    }
}

// Used to retrieve a blob or a tree based on a path.
/// This will iterate over the components and get the desired component.
pub struct TreeIterator<'a> {
    repo: &'a Repository,
    tree: git2::Tree<'a>,
    components: &'a [Component],
    index: usize,
}
impl<'a> TreeIterator<'a> {
    fn new(repo: &'a Repository, tree: git2::Tree<'a>, components: &'a [Component]) -> Self {
        Self {
            repo,
            tree,
            components,
            index: 0,
        }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Result<Option<String>, git2::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
