FROM docker.io/rust:latest

# Change USER_ID and USER_GID to match your current user
ENV USER=dev USER_ID=1000 USER_GID=1000

RUN apt-get update
RUN apt-get install -y sudo nano
RUN apt-get install -y qtbase5-dev qtbase5-dev-tools qtquickcontrols2-5-dev qtdeclarative5-dev qml-module-qtquick2 qml-module-qtquick-window2

# Passwordless sudo
### INSECURE, DO NOT USE IN PRODUCTION ###
RUN echo "%sudo ALL = (ALL) NOPASSWD: ALL" >> /etc/sudoers

# now creating user
RUN groupadd --gid "${USER_GID}" "${USER}" && \
    useradd \
      --uid ${USER_ID} \
      --gid ${USER_GID} \
      --create-home \
      --shell /bin/bash \
      ${USER} && \
    gpasswd -a "${USER}" sudo