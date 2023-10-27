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
Timestamps are ISO 8601.

# TODO

- [x] Save/get weight from database
- [x] Create API to list weights
- [x] Create API to add weights
- [x] Make accessible online
- [x] Create github workflow to auto-build docker image on commit to main
- [x] Create github workflow to (manually) ship latest docker image to production server
- [ ] Create client to list, add, delete, update weights
- [ ] Create API to delete weights
- [ ] Create API to update weights
- [ ] Create UI to list weights
- [ ] Create UI to add weight
- [ ] Create UI to delete weight
- [ ] Create UI to update weight
- [ ] Create UI to graph weights over time
- [ ] Add authentication
- [ ] Add tls / https
- [x] Decide on timestamp format(s)
- [ ] Add paging and filtering to GET /weights
