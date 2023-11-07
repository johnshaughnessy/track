# Track App

An app to keep track of personal data.

More details are in the [wiki](https://github.com/johnshaughnessy/track/wiki).

# Initial Setup for Local Development

Source the script that creates an alias for docker compose:

```sh
source bin/alias-docker-compose
```

Build the docker dev environment:

```sh
dc up -d --build
```

You must run server migrations manually:

```sh
# Shell into the container:

  dc exec -it track-server bash

# Then, inside the container:

  diesel migration --migration-dir server/migrations/ list   # Show migrations

  diesel migration --migration-dir server/migrations/ run    # Run migrations

# From there, you can exit the container:

  exit
```

If you edit `docker/dev.yml`, `web-client/Dockerfile.dev`, or `server/Dockerfile.dev`, bring down the environment and then bring it up again.

```sh
dc down
dc up -d --build
```

The web client is available at [http://localhost:8888](http://localhost:8888).
The server apis are available at [http://localhost:8080/api](http://localhost:8080/api).

Alternatively, you can start the server with:

```sh
dc exec -it track-server bash

APP_ENV="dev" cargo run --bin server
```

And the client with

```sh
dc exec -it track-web-client bash

trunk serve web-client/index.html --address 0.0.0.0 --port 8888
```
