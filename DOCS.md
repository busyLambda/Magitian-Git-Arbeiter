# Docs

## Api endpoints

All of these endpoints are prefixed by ```/api```.

### repository
Root:```/repository```
#### Create a new repository
```/repository/new```
Include the following query parameters:
- user_dir <- The directory that the users repos are stored in
- repo_name <- The name of the repository

### Objects
```/object```
#### Retrieve an object via it's path

- Blob:```/object/blob/{path_to_blob}```
- Tree:```/object/tree/{path_to_tree}```

In the case of blobs the blobs contents will be returned in the body and in the case of a tree you will recieve a tree structure.

***Todos: This is not yet done you cannot retrieve stuff that is inside of a subtree of the commit also there is no branch selection yet so these are the feature that await their implementation.***
