# Rust Plugin Experiment

An attempt to create a plugin for a rust system using separate
processes communicating using stdin/stdout.

## The Idea

Rather than import a dynamic library, this approach uses out of process
executables than are launched by a service. Communication is achieved using
stdin/stdout.

## The Experiments

### Experiment a

The service spawns the plugin in a new process.

The service receives lines from the terminal, sand sends them to the plugin.
The plugin reads lines from stdin and echoes them to stdout. The service
reads lines from the stdout of the plugin, and sends them to stdout.

As the plugin isn't on the path, the plugin must have an absolute path
(`./a-plugin`) which means the service must be run from the same folder
(`target/debug`).

### Experiment b

This experiment solves the issue in *experiment a* whereby the plugin must have
an absolute path by discovering the folder of the service and adding it to the
path.

### Experiment c

This experiment introduces configuration. The service takes two command line
arguments: `--plugin-path` and `--plugin-cmdline`. It adds the plugin path to
the environment before launching the plugin, and passes the provided command
line to the plugin.

The plugin command line must be passed as a vector of args (as if they were
provided by the shell). To do this it uses the
[shell-words](https://github.com/tmiasko/shell-words) package.
