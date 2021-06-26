# minimal-http-responder

This `minimal-http-responder` is intended to create the minimal container that can return a particular HTTP status code.
It returns 200 OK by default, and can be configured by environment variable.

Usage:
```
minimal-http-responder
```

This program listens on all interfaces on port 2020.

Environment variables:

- `STATUS_CODE`: the status code that the `minimal-http-responder` should return (e.g. 200, 403, etc)
- `TEXT`: the body text that the `minimal-http-responder` should return. If not specified, a default value is chosen based on the status code.
