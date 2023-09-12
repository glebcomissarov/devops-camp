# FastAPI app & docker image

## Usage

There are two Docker images that you can use:

- `glebcom/py-app:1.0.0-alpine`
- `glebcom/py-app:1.0.0-slim`

```bash
$ docker-compose --env-file ./.env.dev up -d
```

Test server responce:

```console
$ curl -i localhost:8000/hostname
```

```
HTTP/1.1 200 OK
date: Tue, 12 Sep 2023 20:02:56 GMT
server: uvicorn
content-length: 27
content-type: application/json

{"hostname":"23c07c1db6ea"}
```

## Step by step

Download `py-app` image:

```bash
# build image from Dockerfile
$ docker build -t glebcom/py-app:1.0.0-alpine .

# or use docker pull
docker pull glebcom/py-app:1.0.0-alpine
```

Run container:

```bash
$ docker run -it --name py-app -e AUTHOR=Docker -e UUID="Not Found" -p 8000:8000 -d --rm glebcom/py-app:1.0.0-alpine
```

Or run with `docker-compose`:

```bash
$ docker-compose --env-file ./.env.dev up -d

# stop & remove container
$ docker-compose down
```
