# Track App

An app to track my weight.

The server exposes an API for listing, adding, deleting, and updating weight data.

Each row in the weight table contains:

- ID
- Weight in kgs
- Timestamp when weight was measured
- Timestamp record created
- Timestamp record updated

The server is the authority of these fields:

- id
- created_at
- updated_at

To list weights, the client GETs /api/weights. Paging, filtering, etc will be worked out in the future. For now, no parameters are expected. The server replies with this payload:

```js
{
  weights: [
    {
      id: 10,
      weight_kg: 70,
      measured_at: <timestamp>
    },
    ...
  ]
}

```

To add a weight, the client POSTs this data as json to /api/weights :

```js
{
  weight_kg: 70,
  measured_at: <timestamp>
}
```

To update a weight, the client PATCHes this data as json to /api/weights :

```js
{
  id: 10,
  weight_kg: 70,
  measured_at: <timestamp>
}
```

The server response to a successful add or an update with a json payload describing the weight (including its ID):

```js
{
  id: 23,
  weight_kg: 70,
  measured_at: <timestamp>
}
```

To delete a weight, the client sends a DELETE to /api/weight/<weight_id> .

HTTP status codes are returned in the [usual way](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status).

The server is written in rust.
The client is written in a mixture of rust (compiled to wasm) and some html, css, and javascript as needed.
Timestamps are the number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”).

# TODO

- [x] Save/get weight from database
- [x] Create API to list weights
- [x] Create API to add weights
- [x] Make accessible online
- [x] Create github workflow to auto-build docker image on commit to main
- [x] Create github workflow to (manually) ship latest docker image to production server
- [x] Decide on timestamp format(s)
- [x] Switch to postgres (from sqlite)
- [x] Manage the postgres and server processes with docker compose
- [x] Set up the dev environment to use docker compose
- [x] Configure prod variables and secrets for github actions/workflows
- [ ] Automate database migration deployment
- [ ] (Re)Create API to list weights
- [ ] (Re)Create API to add weights
- [ ] Create API to delete weights
- [ ] Create API to update weights
- [ ] Create client to list, add, delete, update weights
- [ ] Create UI to list weights
- [ ] Create UI to add weight
- [ ] Create UI to delete weight
- [ ] Create UI to update weight
- [ ] Create UI to graph weights over time
- [ ] Add authentication
- [ ] Add tls / https
- [ ] Add paging and filtering to GET /weights

# Dev Log

## 2023-10-28

I migrated from sqlite to postgres, and started using diesel.

I set up Github workflows to automate building and deploying the application. I have not automated database migrations yet.

Instead, to run database migrations in production I followed these steps:

- I `sshed` into the prod vm.

- I spun up a (throw-away) docker container using a rust image in the same "docker-compose" environment/network:

```sh
docker run --rm -it -v ~/track:/app --network docker_default rust:1.73-slim-buster /bin/bash
```

- I installed some prerequisites in the container. Namely libpq-dev:

```sh
apt-get update && apt-get install -y build-essential libpq-dev
```

- I installed `diesel_cli` in the container:

```sh
cargo install diesel_cli --no-default-features --features postgres
```

- I saved a snapshot of the container as an image I can reuse:

```sh
docker commit a84c48dd9ca7 johnshaughnessy/migration-runner

# sha256:a482b7941204854c84b2d3b1e067ca058a1cca4f8aee777643ef499a84fb8d3b
```

- I made sure the DATABASE_URL environment variable was configured and ran the migrations.

```sh
DATABASE_URL=postgres://postgres:redacted@db:5432/postgres diesel migration run
```

- I made sure that the migrations had run successfully. Since I don't have automated tests yet, I just manually created some entries with `curl` and listed them.

- I shut down the throw-away container.
