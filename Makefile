default: format

format:
	cargo fmt

test:
	cargo test

docker-build:
	docker build . -t rust-rest-api-exploration

docker-run:
	docker run -p 8080:8080 rust-rest-api-exploration
