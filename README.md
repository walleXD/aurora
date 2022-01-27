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

### Planned supported platforms

- [ ] macOS M1
- [ ] macOS Intel
- [ ] linux x86-64
- [ ] linux arm64

### Roadmap

v0.1.0

- [ ] implement local type plugin
- [ ] implement lock command

v0.2.0

- [ ] implement github type plugin

v0.3.0

- [ ] implement add command
- [ ] implement rm command
- [ ] implement ls command

v0.4.0

- [ ] implement remote type plugin

v0.5.0

- [ ] implement git type plugin

v0.6.0

- [ ] roll out support for macOS intel
- [ ] publish on homebrew

v0.7.0

- [ ] roll out support for linux x86-64
