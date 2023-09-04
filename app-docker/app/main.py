from fastapi import FastAPI, HTTPException, status
import socket
import os
import random


app = FastAPI()


@app.get("/") # endpoint
def index():
    return { "status": "FastAPI server is running" }


@app.get("/hostname")
def get_hostname():
    return { "hostname": socket.gethostname() }


@app.get("/author")
def get_author():
    return { "author": os.environ.get("AUTHOR", "unknown author") }


@app.get("/id")
def get_id():
    return { "uuid": os.environ.get("UUID", "unknown UUID") }


# === health check ===

@app.get("/liveness")
def liveness():
    return { "status": "server is live" }


@app.get("/readiness")
def readiness():
    n = random.randint(0,1)
    if n == 1:
        return { "status": "server is ready" }
    else:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE)


# uvicorn app.main:app --reload \
#       --host 127.0.0.1 \
#       --port 8000 \
#       --env-file .env.dev
