# Actix Web API

## Docker

### Build image

```shell
docker build -t actix-web-api .
```

### Run built image

```shell
docker run -d -p 8080:8080 actix-web-api
# and the server should start instantly
curl http://localhost:8080
```

### Running unit tests

```shell
docker build -t actix-web-api:test --target base .
docker run --rm actix-web-api:test
```