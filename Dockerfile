FROM docker.finogeeks.club/base/alpine:3.7

COPY target/release/yaml_merge /usr/bin/yaml_merge
ENV PATH /opt/gatling/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
WORKDIR /app
