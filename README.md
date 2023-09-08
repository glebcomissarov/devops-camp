# devops-camp

<img src="https://img.shields.io/static/v1?style=for-the-badge&label=&labelColor=55ba80&message=DevOps+Cloud.ru+Camp&color=23282f&logo=icloud&logoColor=white" alt="cloud.ru"/>

## Usage

> [!NOTE]  
> There are some additional changes _for K8s and helm parts_ that are not related with main tasks:
>
> - additional Actix app (in Rust) that communicate with FastAPI app
> - ingress controller
>
> If you do not want to see it, you can jump to [this commit](https://github.com/glebcomissarov/devops-camp/tree/1a33551ab0760540f51d82d81a64223c5a24d1e0).

How to use this repo:

1. Clone this repo. For instance with ssh link:

   ```bash
   $ git clone git@github.com:glebcomissarov/devops-camp.git
   ```

2. Move to any folder and follow instructions in corresponding `README` file. For instance:

   ```bash
   $ cd ./playbook

   # next: follow README file in that folder
   ```

## Tasks

### Task 1: Ansible playbook

<img src="https://img.shields.io/badge/Ansible-da2f20?style=for-the-badge&logo=ansible&logoColor=white" alt="ansible"/>

Необходимо написать Ansible playbook, который выполняет на хосте следующие действия:

- [x] создает нового пользователя cloudru с паролем cloudpass
- [x] разрешает на хосте авторизацию через ssh по ключу
- [x] запрещает логин по ssh от пользователя root
- [x] копирует предоставленный публичный ключ (`cloudru_rsa.pub`) для пользователя cloudru

Плейбук должен выполняться относительно чистого дистрибутива ОС Ubuntu Server 22.04.3. Полученный плейбук и команду для его запуска положить в папку `/playbook`

### Task 2: Web приложение на Python

<img src="https://img.shields.io/badge/FastAPI-009688?style=for-the-badge&logo=fastapi&logoColor=white" alt="python"/>

Требуется написать простое веб-приложение на Python, которое слушает входящие соединения на порту `8000` и предоставляет HTTP API, в котором реализовано 3 метода:

- [x] `GET /hostname` - при запросе на этот метод приложение отдает имя хоста, на котором запущено приложение

- [x] `GET /author` - возвращает значение переменной окружения `$AUTHOR`, в которой задано имя или никнейм человека, выполняющего это задание

- [x] `GET /id` - возвращает значение переменной окружения `$UUID`, содержащее любую произвольную строку-идентификатор в формате uuid

<img src="https://img.shields.io/badge/Docker-326CE5?style=for-the-badge&logo=docker&logoColor=white" alt="docker"/>

Dockerfile:

- [x] Необходимо написать Dockerfile для полученного приложения в соответствии с принятыми в сообществе best-practice.

Полученный скрипт и Dockerfile к нему положить в папку `/app-docker`

<img src="https://img.shields.io/badge/Kubernetes-326CE5?style=for-the-badge&logo=kubernetes&logoColor=white" alt="kubernetes"/>

Kubernetes:

- [x] Необходимо написать манифест для запуска приложения в Kubernetes в отдельном неймспейсе в виде Deployment с 3 репликами и сервиса с типом ClusterIP. Реализовать readiness- и liveness- пробы. В переменную UUID должен подставляться уникальный идентификатор пода в кластере, в котором запущено приложение.

Манифест положить в папку `/manifest`

<img src="https://img.shields.io/badge/Helm-0F1689?style=for-the-badge&logo=helm&logoColor=white" alt="helm"/>

Helm:

- [x] Написать Helm чарт, в котором через переменные в файле `values.yaml` можно задать:

  - имя образа, запускаемого в поде
  - количество реплик приложения
  - значение, подставляемое в переменную `$AUTHOR`

Полученный чарт положить в папку `/helm`
