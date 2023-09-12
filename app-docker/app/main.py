from fastapi import FastAPI, HTTPException, status, Request, Response
import socket
import os
import random

# -------------------------------------------------------------
# === code for k8s readinessProbe simulation ==================
# -------------------------------------------------------------
# this code will be executed once,
# before the application starts receiving requests
# https://fastapi.tiangolo.com/advanced/events/#lifespan-events

from contextlib import asynccontextmanager

def request_author() -> bool:
    # lets say that this request can fail due to some reasons
    return bool(random.randint(0,1))


app_state = {}

@asynccontextmanager
async def lifespan(app: FastAPI):
    app_state["request_author"] = request_author()
    yield

# =============================================================


# app initialization
app = FastAPI(lifespan=lifespan)


REQUEST_TOKEN = "15708702692062422508"


@app.get("/", status_code=200)
def index(req: Request, resp: Response):
    """
    Root endpoint of server.

    Allows to check access rights by requiring REQUEST_TOKEN in request Header.
    """
    token = req.headers.get("REQUEST_TOKEN")
    if token == REQUEST_TOKEN:
        return { "access": "OK" }
    elif token == "HealthCheck": # for k8s livenessProbe
        return { "server_status":  status.HTTP_200_OK }
    else:
        resp.status_code = status.HTTP_403_FORBIDDEN
        return { "access": "HTTP_403_FORBIDDEN" }


@app.get("/hostname")
def get_hostname():
    """
    Returns hostname
    """
    return { "hostname": socket.gethostname() }


@app.get("/author")
def get_author():
    """
    Returns author name
    """
    if app_state["request_author"]:
        return { "author": os.environ.get("AUTHOR", "unknown author") }
    else:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE)



@app.get("/id")
def get_id():
    """
    Returns value from UUID env variable (if k8s is used: it should be Pod ID)
    """
    return { "uuid": os.environ.get("UUID", "unknown UUID") }


# === readiness check ===

# @app.get("/health")
# def liveness():
#     return { "status": "server is live" }


@app.get("/readiness", status_code=200)
def readiness(resp: Response):
    if app_state["request_author"]:
        return f"Access Denied (status code: {status.HTTP_200_OK})"
    else:
        resp.status_code = status.HTTP_409_CONFLICT
        return f"Access Denied (status code: {status.HTTP_409_CONFLICT})"
        # raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE)


# TO TEST APP LOCALLY (.env file is required):
# uvicorn app.main:app --reload \
#       --host 127.0.0.1 \
#       --port 8000 \
#       --env-file .env.dev
