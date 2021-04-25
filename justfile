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
  docker-compose --file ./infrastructure/docker/docker-compose.base.yaml --file ./infrastructure/docker/docker-compose.local.yaml --project-dir . up --build --remove-orphans

# Take down all running launchpad services, including database volumes
crash:
  @echo 'Blast that piece of junk out of the sky!'
  docker-compose --file ./infrastructure/docker/docker-compose.base.yaml --file ./infrastructure/docker/docker-compose.local.yaml --project-dir . down

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

# Pass commands to the justfile in infrastructure/deployment
production command:
  @echo 'Calling production {{command}}...'
  cd infrastructure/deployment && just {{command}}

# Open the local version of the app
app:
  open http://127.0.0.1

# Open the local mail server
mail:
  open http://127.0.0.1:4436