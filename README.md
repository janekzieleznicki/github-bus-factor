# Simple app to fetch projects from github API and calculate bus factor of main contributor

## Build and run

In order to run application it's necessary to provide API token via environment variable. Example:

```shell
TOKEN=$(cat .token) cargo run -- --language python --count 5000
```

Some tests access github API so a token is needed to run them as well
```shell
TOKEN=$(cat .token) cargo test
```

## Run in docker

1. Build image
    ```shell
    docker build --tag busfactor --file=Dockerfile .
    ```
2. Run application
    ```shell
    docker run -it --env TOKEN=$(cat .token) busfactor
    ```