FROM docker.finogeeks.club/base/alpine:3.7

COPY target/release/yaml_merge /usr/bin/yaml_merge
WORKDIR /app
