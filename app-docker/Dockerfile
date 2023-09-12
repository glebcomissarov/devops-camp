FROM python:3.11-slim

WORKDIR /opt/app

COPY ./requirements.txt .
RUN pip install --no-cache-dir --upgrade -r ./requirements.txt

RUN apt-get -y autoremove \
    && apt-get clean autoclean \
    && rm -fr /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY app/* .

EXPOSE 8000

CMD [ "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000" ]

# ------------------------------------------------------------
# Image on Docker Hub: https://hub.docker.com/r/glebcom/py-app
# ------------------------------------------------------------