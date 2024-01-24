# API

GET /api/healthchecker
Response
```
{
    "status": "success",
    "message": "Outlook helper"
}
```

GET /api/items/:id
Response
```
{
    "status": "success",
    "results": 1,
    "history: [
        {
            "action": "assigned",
            "date": "2024-01-24T08:50:47.097550Z",
            "id": "d94ad36f-3bbe-44d3-999e-bbe2409c6fc0",
            "topic": "1",
            "user": "batman@gotham.com"
        }
    ]
}
```

PATCH /api/items/:id
Request Json
```
{
    "user": "String"
    "action": "String"
}
```

Response
```
{
    "status": "success"
}
```

DELETE /api/history/:id
