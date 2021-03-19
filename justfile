# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

# List available recipes
default:
  @just --list

# Interact with the entire Launchpad service
launch:
  @echo 'T-Minus 10 seconds. Main engine starting...'
  @sleep 1
  @echo '9...'
  @sleep 1
  @echo '8...'
  @sleep 1
  @echo '7...'
  @sleep 1
  @echo '6...'
  @sleep 1
  @echo '5...'
  @sleep 1
  @echo '4...'
  @sleep 1
  @echo '3...'
  @sleep 1
  @echo '2...'
  @sleep 1
  @echo '1...'
  @sleep 1
  @echo 'Blastoff!'
  @sleep 1
  docker-compose up --remove-orphans

# Initialize dependencies
initialize:
  @echo 'Installing web dependencies...'
  cd web/ && just initialize

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
