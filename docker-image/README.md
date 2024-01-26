In order to run the Jupyter notebooks on Docker, execute the following:

```shell
docker compose --profile prod up --build
```

When the container is ready, follow the link from its output in the form of `http://127.0.0.1:8888/?token=...`.
