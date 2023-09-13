# let's take Alpine (or Debian) Linux as base image
# FROM python:3.11-slim
FROM python:3.11-alpine

# set working directory
WORKDIR /opt/app

# copy & install required dependencies
COPY ./requirements.txt .
RUN pip install --no-cache-dir --upgrade -r ./requirements.txt

# clean up (but in this case we do not need these commands)
# RUN apt-get -y autoremove \
#     && apt-get clean autoclean \
#     && rm -fr /var/lib/apt/lists/* /tmp/* /var/tmp/*

# copy python files to WORKDIR
COPY app/* .

# tell Docker that app will use 8000 port
EXPOSE 8000

# run uvicorn server with FastAPI app
CMD [ "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000" ]

# ------------------------------------------------------------
# Image on Docker Hub: https://hub.docker.com/r/glebcom/py-app
# Image size:
#   - 1.0.0-debian-slim: 54.65 MB
#   - 1.0.0-alpine: 26.81 MB
# ------------------------------------------------------------