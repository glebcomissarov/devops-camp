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

<center>
<figure>
    <img src="../imgs/k8s-cluster-v1.png" height="450">
    <figcaption><i>General schema of K8s cluster</i></figcaption>
</figure>
</center>

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
# get list of pods
$ kubectl logs <pod-name> -n cloudns
# go inside pod
$ kubectl exec -it <pod-name> -- /bin/sh

$ k describe deploy fastapi -n cloudns

$ k describe replicaset <rs-name> -n cloudns
```

### Configure Ingress

As Ingress Controller we will use K8s [Ingress-Nginx](https://kubernetes.github.io/ingress-nginx/deploy/) Controller. Let's run ingress controller.

Ingormation about Ingress-Nginx controller:

```bash
$ kubectl get pods -n ingress-nginx -o wide
```

Setup ingress rules:

```bash
$ kubectl apply -f ingress.yml

# check
$ kubectl get ingress -n cloudns
```

Let's make domain name valid (link ip adress with some domain name). We want to access to Actix app by `https://actix-app.cloud.ru`

```bash
$ sudo nvim /etc/hosts
# and add line:
# 127.0.0.1   actix-app.cloud.ru
```

Delete ingress-nginx controller:

```bash
$ kubectl get ns

$ kubectl delete all --all -n ingress-nginx
```
