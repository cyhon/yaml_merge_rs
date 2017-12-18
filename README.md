# yaml-merge

### Usage
#### Shell

```shell
yaml_merge -i foo.yaml bar.yml -o merged.yml
```

#### Docker

```
docker run -it --rm -v `pwd`:/app docker.finogeeks.club/tools/yaml-merge yaml_merge -i foo.yml bar.yml -o merge.yml
```

#### Drone

```
build:
  image: docker.finogeeks.club/tools/yaml-merge
  pull: true
  commands:
    - yaml_merge -i foo.yml bar.yml -o merge.yml
  when:
    branch: master
```
