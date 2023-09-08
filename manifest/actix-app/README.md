# Actix web app

This is small web application written in Rust using [Actix](https://actix.rs) web framework. It has two endpoints:

- `/` - root endpoint just prints information that app is working
- `/check_fastapi_app` - sends http requests to server with hostname specified at `FASTAPI_SERVICE_HOSTNAME` variable and prints result.

Run with `docker-compose`:

```bash
$ docker-compose --env-file ./.env.dev up -d

$ docker-compose down
```

Test app (in case if it is running with k8s):

```bash
curl -i http://localhost:8090/check_fastapi_app
```

```
HTTP/1.1 200 OK
content-length: 379
date: Fri, 08 Sep 2023 08:31:22 GMT

Routes {
    data: [
        (
            "/hostname",
            "Hostname { hostname: \"fastapi-84d9cfdcdf-vxr6w\" }",
        ),
        (
            "/author",
            "Author { author: \"Gleb Komissarov (taken from k8s ConfigMap)\" }",
        ),
        (
            "/id",
            "PodID { uuid: \"e5b9d8ba-8f50-402c-ad04-efa3ff5a1cd0\" }",
        ),
    ],
}%
```

Usefull commands to build Docker image:

```bash
$ docker build -t glebcom/actix-app .

$ docker run -it --rm --name actix -p 8080:8080 -d -e FASTAPI_SERVICE_HOSTNAME="http://localhost:8000" glebcom/actix-app

$ docker exec -it actix sh

$ docker push glebcom/actix-app
```
