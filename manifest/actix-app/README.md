# Actix web app

This is small web application written in Rust using [Actix](https://actix.rs) web framework. It has two endpoints:

- `/` - root endpoint just prints information that app is working
- `/check_fastapi_app` - sends http requests to server with hostname specified at `FASTAPI_SERVICE_HOSTNAME` variable and prints result.

Run with `docker-compose`:

```bash
$ docker-compose --env-file ./.env.dev up -d

$ docker-compose down
```

Test app (run `docker-compose.yml` file):

```bash
curl -i "http://localhost:8080/check_fastapi_app?access_token=cloudru125"
```

```
HTTP/1.1 200 OK
content-length: 329
date: Wed, 13 Sep 2023 11:17:45 GMT

Routes {
    data: [
        (
            "/hostname",
            "Hostname { hostname: \"glebmac.local\" }",
        ),
        (
            "/author",
            "Author { author: \"Gleb Komissarov\" }",
        ),
        (
            "/id",
            "PodID { uuid: \"uuid-should-be-generated\" }",
        ),
    ],
}
```

Usefull commands to build Docker image:

```bash
$ docker build -t glebcom/actix-app:1.0.0-bullseye-slim .

$ docker run -it --rm --name actix -p 8080:8080 -d -e FASTAPI_SERVICE_HOSTNAME="http://localhost:8000" glebcom/actix-app:1.0.0-bullseye-slim

$ docker exec -it actix sh

$ docker push glebcom/actix-app:1.0.0-bullseye-slim
```

## Testing

```bash
$ cargo test --test without_fastapi -- --show-output
$ cargo test --test integration_test -- --show-output
```
