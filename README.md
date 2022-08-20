# FastAPI vs axum
A project to assess the differences between two web frameworks in Python (FastAPI) and Rust (axum). The frameworks will be assessed using three criteria: code complexity, deployment, and speed.

## Code Complexity

Rust's base app logic is more complex than FastAPI's `App` object and decorator pattern. 

_Note: this is a deliberate design choice of `axum` to have a "a macro free API" compared to other Rust frameworks with a similar code pattern/feel as decorators in Python_

<img src="./assets/apps.png"/>
<br>
<br>

In this case, the handlers and data models (pydantic and serde respectively) are virtually the same code footprint:

<img src="./assets/handlers.png" />
<br>
<br>

## Deployment

### Dependencies
Python (FastAPI) requires dependency management at runtime. In this case, `poetry` was used and the `requirements.txt` file is made via `poetry` to ensure pinned dependencies for the runtime environment:

```shell
poetry export --without-hashes > requirements.txt
```

For Rust, `cargo` and the `Cargo.toml` manage all the dependencies. Since Rust is compiled there is no runtime dependencies to pin or manage.

The differences are really a tradeoff, Rust has two Dockerfiles and Python has one Dockerfile, but requires a method to install pinned dependencies (for a consistent environment).

<img src="./assets/dockerfiles.png" />
<br>
<br>

### Image Size

Rust requires a large upfront image for compilation (this could most likely be smaller, but compilation image size is not a concern). Python's image size can vary depending on user configuration, but remains ~2x larger than the Rust server image:

<img src="./assets/images.png" />
<br>
<br>

Overall for deployment, FastAPI takes more configuration: various base images (buster, slim-buster, etc.), two possible workers (gunicorn and uvicorn), and a dependency management solution. Being a compiled language, Rust avoids some of these tasks/configuration.

## Speed

<img src="./assets/docker-settings.png" width=50% />