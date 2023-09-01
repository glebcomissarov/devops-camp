# devops-camp

<img src="https://img.shields.io/static/v1?style=for-the-badge&label=&labelColor=55ba80&message=DevOps+Cloud.ru+Camp&color=23282f&logo=icloud&logoColor=white" alt="cloud.ru"/>

## Ansible playbook

<img src="https://img.shields.io/badge/Ansible-da2f20?style=for-the-badge&logo=ansible&logoColor=white" alt="ansible"/>

Необходимо написать Ansible playbook, который выполняет на хосте следующие действия:

- [x] создает нового пользователя cloudru с паролем cloudpass
- [x] разрешает на хосте авторизацию через ssh по ключу
- [x] запрещает логин по ssh от пользователя root
- [x] копирует предоставленный публичный ключ (`cloudru_rsa.pub`) для пользователя cloudru

Плейбук должен выполняться относительно чистого дистрибутива ОС Ubuntu Server 22.04.3. Полученный плейбук и команду для его запуска положить в папку `/playbook`

## Web приложение на Python

<img src="https://img.shields.io/badge/FastAPI-009688?style=for-the-badge&logo=fastapi&logoColor=white" alt="python"/>

Требуется написать простое веб-приложение на Python, которое слушает входящие соединения на порту `8000` и предоставляет HTTP API, в котором реализовано 3 метода:

- [ ] `GET /hostname` - при запросе на этот метод приложение отдает имя хоста, на котором запущено приложение

- [ ] `GET /author` - возвращает значение переменной окружения `$AUTHOR`, в которой задано имя или никнейм человека, выполняющего это задание

- [ ] `GET /id` - возвращает значение переменной окружения `$UUID`, содержащее любую произвольную строку-идентификатор в формате uuid

<img src="https://img.shields.io/badge/Docker-326CE5?style=for-the-badge&logo=docker&logoColor=white" alt="docker"/>

Dockerfile:

- [ ] Необходимо написать Dockerfile для полученного приложения в соответствии с принятыми в сообществе best-practice.

Полученный скрипт и Dockerfile к нему положить в папку `/app`

<img src="https://img.shields.io/badge/Kubernetes-326CE5?style=for-the-badge&logo=kubernetes&logoColor=white" alt="kubernetes"/>

Kubernetes:

- [ ] Необходимо написать манифест для запуска приложения в Kubernetes в отдельном неймспейсе в виде Deployment с 3 репликами и сервиса с типом ClusterIP. Реализовать readiness- и liveness- пробы. В переменную UUID должен подставляться уникальный идентификатор пода в кластере, в котором запущено приложение.

Манифест положить в папку `/manifest`

<img src="https://img.shields.io/badge/Helm-0F1689?style=for-the-badge&logo=helm&logoColor=white" alt="helm"/>

Helm:

- [ ] Написать Helm чарт, в котором через переменные в файле `values.yaml` можно задать:

  - имя образа, запускаемого в поде
  - количество реплик приложения
  - значение, подставляемое в переменную `$AUTHOR`

Полученный чарт положить в папку `/helm`
