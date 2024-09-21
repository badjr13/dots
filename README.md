# dots

## MVP
- User has a git repository containing items which can be directories or files
- User clones repository locally
- User runs `dots init` inside of repository
  - `dots init` generates a manifest file which manages absolute deployment locations for tracked items to be deployed to
  - This manifest file becomes part of the git repository
    - One manifest file for UNIX and one for Windows
  - Default deployment path are set to:
    - `~/.config` for UNIX
    - `%APPDATA%` for windows
    - Paths can be updated via $EDITOR
      - Absolute paths recommended
      - Prompt user if path starts with `.`
        - Absolute paths enable users to clone the repository anywhere on their system and still be able to deploy to the correct locations
- `dots deploy` will deploy tracked items to defined locations and create symlinks back to repository
  - `dots deploy` will simply deploy updates on subsequent deployments
  - If new items have been added since last deployment, `dots deploy` will add new items to tracking file and prompt user if default location is ok
- `dots destroy` will remove all items from their deployed locations and destroy symlinks

## Additional Features

- `dots track` can be run on a file or directory outside of the repository
  - item will be copied to the repository where it becomes just like any other item
