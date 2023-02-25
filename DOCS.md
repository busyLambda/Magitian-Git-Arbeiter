# Docs

# Api endpoints

All of these endpoints are prefixed by ```/api```.

## repository
Root:```/repository```
### Create a new repository
```/repository/new```
Include the following query parameters:
- user_dir <- The directory that the users repos are stored in
- repo_name <- The name of the repository

## Objects
```/object```
### Retrieve an object via it's path

- Blob:```/object/{path_to_repo}/blob/{path_to_blob}```
- Tree:```/object/{path_to_repo}/tree/{path_to_tree}```

- Root:```/object/{path_to_repo}/tree``` **IMPORTANT**: if you put a '/' at the end it will **NOT** match since that is the Tree path.

In the case of blobs the blobs contents will be returned in the body and in the case of a tree you will recieve a tree structure.

## Collab
```/collab```
Used for "code review", diffs, and merge requests.
### Diff
```/diff```
- Compare two commits: ```/show/{user_dir}/{repo_dir}?from={Older Commit OID}&to={Newer commit OID}```
  - example: 
  ```
  http://localhost:8984/api/collab/diff/show/asd/linux?from=12b46a201db6d7072a1c82ac6534d37808b1ab3f&to=ca7a0af2d86a14f73ca0f1191bb17240dfae9ac0
  ```

### Merge
