# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

# List available recipes
default:
  @just --list

# Run the entire launchpad service locally
launch:
  @echo 'Main engine starting...'
  @for i in $(seq 10 -1 1); do echo 'T-Minus: '$i'...' && sleep 1; done
  @echo 'Liftoff!'
  @cat .rocket.txt
  docker-compose up --build --detach --remove-orphans

# Tail the container logs
logs:
  docker-compose logs --follow

# Take down all running launchpad services, including database volumes
crash:
  @echo 'Blast that piece of junk out of the sky!'
  docker-compose down

# Initialize dependencies
initialize:
  @echo 'Installing web dependencies...'
  cd web/ && just initialize

# Build a specific Launchpad component
build target:
  @echo 'Building {{target}}...'
  cd services/{{target}} && just build

# Start a specific Launchpad component in development mode, watching changes
start target:
  @echo 'Starting {{target}}...'
  cd services/{{target}} && just start

# Run a specific Launchpad component
run target:
  @echo 'Running {{target}}...'
  cd services/{{target}} && just run

# Test a specific Launchpad component
test target:
  @echo 'Testing {{target}}...'
  cd services/{{target}} && just test

# Watch a specific Launchpad component for changes
watch target:
  @echo 'Watching {{target}}...'
  cd services/{{target}} && just watch

# Pass commands to the justfile in production
production command:
  @echo 'Calling production {{command}}...'
  cd production && just {{command}}

# Open the local version of the app
app:
  open http://127.0.0.1

# Open the local mail server
mail:
  open http://127.0.0.1:4436