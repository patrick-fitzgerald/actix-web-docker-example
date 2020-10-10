# actix-web-docker-example

![CI](https://github.com/patrick-fitzgerald/actix-web-docker-example/workflows/CI/badge.svg)

![Deploy](https://github.com/patrick-fitzgerald/actix-web-docker-example/workflows/Deploy/badge.svg?branch=develop)

## About 

An example of how to package an actix-web project into a Docker container.

The Docker image is built using Github Actions.

## Usage

```sh
docker build -t actix-web-docker-example .
docker run -p 8080:8080 actix-web-docker-example
```

## Dependencies

* [Actix Web](https://actix.rs/) - A powerful, pragmatic, and extremely fast web framework for Rust

