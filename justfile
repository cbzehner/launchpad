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

# Run a specific Launchpad component
run target:
  @echo 'Running {{target}}...'
  cd {{target}} && just run

# Test a specific Launchpad component
test target:
  @echo 'Testing {{target}}...'
  cd {{target}} && just test

