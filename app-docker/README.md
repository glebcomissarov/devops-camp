# FastAPI app & docker image

# Usage

```console
$ docker-compose --env-file ./.env.dev up -d
```

Test server responce:

```console
$ curl localhost:8000/author
```

```
{"author":"Gleb Komissarov"}
```

# Step by step

Download `py-app` image:

```bash
# build image from Dockerfile
$ docker build -t glebcom/py-app .

# or use docker pull
docker pull glebcom/py-app:latest
```

Run container:

```bash
$ docker run -it --name py-app -e AUTHOR=Docker -e UUID="Not Found" -p 8000:8000 -d --rm glebcom/py-app:latest
```

Or run with `docker-compose`:

```bash
$ docker-compose --env-file ./.env.dev up -d
```
