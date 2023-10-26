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

```json
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

To add weights, the client POSTs this data as json to /api/weights :

```json
{
  add_weights : [
    {
      weight_kg: 70,
      measured_at: <timestamp>
    },
    ...
  ]
}
```

To update weights, the client POSTs (or PATCHes) this data as json to /api/weights :

```json
{
  update_weights : [
    {
      id: 10,
      weight_kg: 70,
      measured_at: <timestamp>
    },
    ...
  ]
}
```

Timestamp formats (in the database and the client) will be worked out in the future.

To delete weights, the client POSTS this data to /api/weights :

```json
{
  "delete_weights": [10, 12, 13]
}
```

The numbers in this example are weight record IDs.

These requests can be combined. For example, this is valid:

```json
{

  add_weights : [
    {
      weight_kg: 70,
      measured_at: <timestamp>
    },
  ],

  update_weights: [
    {
      id: 10,
      weight_kg: 70,
      measured_at: <timestamp>
    }
  ],

  delete_weights: [ 12, 13 ]

}
```

The server will respond to POST requests with a json payload indicating the results of the requested operations, or any errors.

```json
{
  add_weights : {
    status: 200,
    data: [
      {
        id: 23,
        weight_kg: 70,
        measured_at: <timestamp>
      }
    ]
  },
  "update_weights" : [
    {
      status: 200,
      data: {
        id: 10,
        weight_kg: 70,
        measured_at: <timestamp>
      }
    }
  ],
  "delete_weights" : [
    {
      status: 200,
      id: 12,
    },
    {
      status: 200,
      id: 13,
    },
  ]
}

```

The server is written in rust.
The client is written in a mixture of rust (compiled to wasm) and some html, css, and javascript as needed.

# TODO

- [x] Save/get weight from database
- [x] Create API to list weights
- [x] Create API to add weights
- [x] Make accessible online
- [ ] Create github action to auto-deploy code to production on commit to main
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
- [ ] Decide on timestamp format(s)
- [ ] Add paging and filtering to GET /weights
