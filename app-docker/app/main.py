from fastapi import FastAPI, HTTPException, status, Request, Response
import socket
import os
import random

# -------------------------------------------------------------
# === code for k8s livenessProbe simulation ==================
# -------------------------------------------------------------
# This code will be executed once,
# before the application starts receiving requests [1].
# It will help to simulate readiness of app to send some data.
# [1] https://fastapi.tiangolo.com/advanced/events/#lifespan-events

from contextlib import asynccontextmanager

def request_author() -> bool:
    """
    Simulation of readiness of server to send data.

    Lets say that this request can fail due to some reasons 
    (for instance, internal error or connection to DB failed).

    Returned value can be
    - `True`: OK, data can be send
    - `False`: some error has occured, server should be restarted (k8s Pod, in our case)
    """
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
    Root endpoint of the server.

    Allows to check access rights by requiring REQUEST-TOKEN in request Header.
    """
    token = req.headers.get("REQUEST-TOKEN")

    if token == REQUEST_TOKEN:
        return { "access": "OK" }
    elif token == "HealthCheck": # for k8s readinessProbe
        # Checks readiness of the server to send data.
        # Status code:
        # - `200`: OK, server is working correct
        # - `409_CONFLICT`: server will not receive any traffic

        is_ready = bool(random.randint(0, 5))

        if is_ready:
            return f"readinessProbe (status code: {status.HTTP_200_OK})"
        else:
            resp.status_code = status.HTTP_409_CONFLICT
            return f"readinessProbe (status code: {status.HTTP_409_CONFLICT})"
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
    Returns value from UUID env variable.
    
    If app is running inside k8s Pod - it should be Pod ID
    """
    return { "uuid": os.environ.get("UUID", "unknown UUID") }

# === liveness check ===

@app.get("/liveness", status_code=200)
def liveness(resp: Response):
    """
    Checks liveness of the server to send data.

    Status code:
    - `200`: OK, server is working correct
    - `409_CONFLICT`: server should be restarted
    """
    if app_state["request_author"]:
        return f"livenessProbe (status code: {status.HTTP_200_OK})"
    else:
        resp.status_code = status.HTTP_409_CONFLICT
        return f"livenessProbe (status code: {status.HTTP_409_CONFLICT})"



# TO TEST APP LOCALLY (.env file is required):
# uvicorn app.main:app --reload \
#       --host 127.0.0.1 \
#       --port 8000 \
#       --env-file .env.dev
