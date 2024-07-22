# Disk Analyzer

This service is responsible for analysing the disk usage of the system and providing the information on a web ui.

## Configuration

Add disks to be analysed as volumes in the `docker-compose.yml` file and bind them to the container.

```yaml
volumes:
  - /:/disk
  - /disk2:/disk2
```

Add all the volume paths to the `PATHS` env variable in the `docker-compose.yml` file.

```yaml
environment:
  - PATHS=/disk,/disk2
```

## How to run

```bash
docker-compose up
```

## How to access

Open your browser and go to `http://localhost:5609`
