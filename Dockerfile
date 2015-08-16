FROM fedora:22
MAINTAINER Alex Crawford

VOLUME /mnt/project
WORKDIR /mnt/project

RUN dnf install --assumeyes findutils
RUN dnf install --assumeyes make
RUN dnf install --assumeyes gcc
RUN dnf install --assumeyes arm-none-eabi-gcc
RUN dnf install --assumeyes arm-none-eabi-newlib
