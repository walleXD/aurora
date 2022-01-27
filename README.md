# Aurora

## Manage hostfiles from the cli

### Motivation
Control over hostfiles is a very common task. Currently most of the cli apps feel cumbersome. There are many GUI experiences for managing hostfiles but they often feel too clunky.

Inspired by `sheldon`, which is a zsh plugin manager, `aurora` provides a simple interface to manage hostfiles like plugins.

### Planned plugin types
- [ ] local: load plugins from local file
- [ ] remote: load plugins from remote url
- [ ] github: load plugins from github
- [ ] git: load plugins from remote git repo

### Planned commands
- [ ] lock: installs and loads up all plugins
- [ ] add: add a new plugin
- [ ] rm: remove a plugin
- [ ] ls: list all plugins