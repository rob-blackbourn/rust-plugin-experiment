# Rust Plugin Experiment

An attempt to create a plugin for a rust system using separate
processes communicating using stdin/stdout.

# Experiment a

Spawn the plugin in a new process. As the process isn't on the path,
this must be run directly from the target/debug folder.
