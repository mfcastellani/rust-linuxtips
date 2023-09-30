# Axum test

- [x] Start service
- [x] Basic logging
- [x] Handle root path
- [x] Improve logging with env var
- [ ] Connect to postgres
- [ ] Store posts POST /posts
- [ ] Read post GET /posts/[id]
- [ ] Update post PUT /posts/[id]
- [ ] Delete post DELETE /posts/[id]
- [ ] List all posts GET /posts
- [x] Liveness endpoint in /status/liveness
- [ ] Readiness endpoint in /status/readiness
- [ ] Swagger or similar
- [x] CORS Handling
- [ ] Authenticate user for POST, PUT and DELETE
- [x] Customize logs

## Refs

- https://github.com/tower-rs/tower-http/tree/master/examples
- https://docs.rs/axum/latest/axum/middleware/index.html
- https://github.com/intelliconnect/api_with_axum/blob/main/src/main.rs
- https://medium.com/intelliconnect-engineering/using-axum-framework-to-create-rest-api-part-1-7d434d2c5de4
- https://docs.rs/axum/latest/axum/

## Cargo make

```
$ cargo install cargo-make
$ cargo make watch
```