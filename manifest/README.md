# Kubernetes cluster

## Usage

```bash
# for ingress:
$ make write

# run config files
$ kubectl apply -f setup.yml # setup Namespace and ComfigMap
$ kubectl apply -f deploy-actix.yml
$ kubectl apply -f deploy-fastapi.yml

# live watch how ReplicaSet will manage Pods
$ kubectl get pods -n cloudns --watch

# print Pod name and Pod ID
$ kubectl get pods -n cloudns -o custom-columns=PodName:.metadata.name,PodUID:.metadata.uid

# stop server, free resourses and delete ip->host link line
$ kubectl delete -f setup.yml
$ make delete_on_mac
```

Test app response:

```bash
$ curl -i "http://actix-app.cloud.ru/check_fastapi_app?access_token=cloudru125"
```

```
HTTP/1.1 200 OK
Date: Wed, 12 Sep 2023 19:43:36 GMT
Content-Length: 379
Connection: keep-alive

Routes {
    data: [
        (
            "/hostname",
            "Hostname { hostname: \"fastapi-74994c7778-vfn72\" }",
        ),
        (
            "/author",
            "Author { author: \"Gleb Komissarov (taken from k8s ConfigMap)\" }",
        ),
        (
            "/id",
            "PodID { uuid: \"cdee5ced-c55e-4f95-86b0-dfaeaa8d992e\" }",
        ),
    ],
}%
```

> [!IMPORTANT]  
> Note that hostname and id are not necessarily belong to the same Pod. To make this response, Actix app sends three http-requests to FastAPI, so when these requests come to ClusterIP, this service _randomly_ choose Pod to handle particular request.

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
# check that deployment is running
$ kubectl get deploy -n cloudns
```

```
NAME           READY   UP-TO-DATE   AVAILABLE   AGE
external-app   2/2     2            2           4m6s
fastapi        3/5     5            3           4m1s
```

```bash
# get pods
$ kubectl get pods -n cloudns
```

```bash
# get services
$ kubectl get svc -n cloudns
```

```
NAME                     TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)   AGE
actix-internal-service   ClusterIP   10.107.104.64   <none>        80/TCP    4m45s
fastapi-app              ClusterIP   10.109.210.63   <none>        80/TCP    4m40s
```

```bash
# get list of pods
$ kubectl logs <pod-name> -n cloudns
# go inside docker container of Pod
$ kubectl exec --stdin --tty <pod-name> -n cloudns -- /bin/sh

$ kubectl describe deploy fastapi -n cloudns

$ kubectl describe replicaset <ReplicaSet-name> -n cloudns
```

### Configure Ingress

As Ingress Controller we will use K8s [Ingress-Nginx](https://kubernetes.github.io/ingress-nginx/deploy/) Controller. Let's run ingress controller.

Ingormation about Ingress-Nginx controller:

```bash
$ kubectl get pods -n ingress-nginx -o wide
```

Setup ingress rules:

```bash
# check
$ kubectl get ingress -n cloudns
```

Let's make domain name valid (link ip adress with some domain name). We want to access to Actix app by `https://actix-app.cloud.ru`

```bash
$ make write
```

This command will add following line to /etc/hosts file:

```
127.0.0.1   actix-app.cloud.ru
```

Delete ingress-nginx controller:

```bash
# find 'ingress-nginx' namespace
$ kubectl get ns

$ kubectl delete all --all -n ingress-nginx
```
