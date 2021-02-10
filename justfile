# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

# List available recipes
default:
  @just --list

# Build a specific Launchpad component
build target:
  @echo 'Building {{target}}...'
  cd {{target}} && just build

# Start a specific Launchpad component in development mode, watching changes
start target:
  @echo 'Starting {{target}}...'
  cd {{target}} && just start

# Run a specific Launchpad component
run target:
  @echo 'Running {{target}}...'
  cd {{target}} && just run

# Test a specific Launchpad component
test target:
  @echo 'Testing {{target}}...'
  cd {{target}} && just test

# Watch a specific Launchpad component for changes
watch target:
  @echo 'Watching {{target}}...'
  cd {{target}} && just watch
