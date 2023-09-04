# Kubernetes cluster

## Usage

```bash
# run deployment
$ kubectl apply -f setup.yml

# stop server and free resourses
$ kubectl delete -f setup.yml
```

Test app responses:

```bash
$ curl localhost:8080/
{"status":"FastAPI server is running"}
```

```bash
$ curl localhost:8080/id
{"uuid":"37ffea85-62af-46de-87fe-5e5a6c0c99a7"}
```

```bash
$ curl localhost:8080/author
{"author":"Gleb Komissarov"}
```

## Step by step

Check that kubernetes is working and ready to go:

```bash
# check that cluster is live
$ kubectl cluster-info
```

```bash
# run deployment
$ kubectl apply -f setup.yml

# check that deployment is running
$ kubectl get deploy -n cloudns
```

```bash
$ kubectl get pods -n cloudns
```

```bash

$ kubectl logs <pod-name> -n cloudns

$ k describe deploy fastapi -n cloudns

$ k describe replicaset <rs-name> -n cloudns
```
