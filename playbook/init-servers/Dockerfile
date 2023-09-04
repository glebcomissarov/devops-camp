FROM ubuntu:20.04

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install --no-install-recommends -y \
        sudo \
        openssh-server \
        python3

RUN mkdir /var/run/sshd

# set password to root user and append to sudo group
RUN echo 'root:rootpass' | chpasswd && adduser root sudo
# allow root user login with ssh
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

USER root

# copy ssh public key to server
COPY ./root_rsa.pub /root/.ssh/root_rsa.pub
RUN cat /root/.ssh/root_rsa.pub >> /root/.ssh/authorized_keys

RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 22

# ENTRYPOINT service ssh restart && bash

# run sshd
CMD ["/usr/sbin/sshd", "-D", "-e"]