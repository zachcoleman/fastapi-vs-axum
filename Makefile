current_dir = $(shell pwd)

### axum

.PHONY: docker-build-axum
docker-build-axum:
	docker build . -f ./Dockerfiles/axum-build -t axum-build
	docker run -v $(current_dir):/app axum-build

.PHONY: docker-run-axum
docker-run-axum:
	docker build . -f ./Dockerfiles/axum-server -t axum-server
	docker run -p 80:80 axum-server

.PHONY: native-run-axum
native-run-axum:
	cargo run -r 

### FastAPI

.PHONY: docker-run-fastapi
docker-run-fastapi:
	docker build . -f ./Dockerfiles/fastapi-server -t fastapi-server
	docker run -p 80:80 fastapi-server

.PHONY: native-run-fastapi
native-run-fastapi:
	gunicorn fastapi-server.main:app --workers 1 --worker-class uvicorn.workers.UvicornWorker --bind 0.0.0.0:80