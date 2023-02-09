# Magitian Git Arbeiter

## Role
Api for interacting with git repositories it also stores said repos(obviously) but it does not store their representation in the database.
## Access
Access to repositories and which repo is to be interacted with is not handled by this service, Git smart http implementation is the one doing that task by sending requests to the Magitian Core service which handles the database.