# Rough outline + TODO
- Blue version is docker container listening on port 8080
- if custom healtchecks fail, docker compose is restarted and the new environment brought up
- Forward all requests on port 8080 to a specific docker container listening on PORT 8085 (Blue)
- make requests to a custom healthcheck for blue
- if it fails, redirect all requests to 8084 (Green)
