.PHONY: usage
usage:
	-echo 'Usage: make (test)'

.PHONY: test
test:
	cargo vendor
	cargo build
	cargo test

.PHONY: docker
docker:
	docker build -t server \
		--build-arg=UNAGI_PASSWORD="$${UNAGI_PASSWORD}" \
		-f server.Dockerfile .

.PHONY: run
run: docker
	docker run -p 8080:8080 --name=server -ti --init --rm server

.PHONY: push
push: docker
	docker tag server asia.gcr.io/icfpc-primary/server
	docker push asia.gcr.io/icfpc-primary/server
	gcloud --project=icfpc-primary run services update \
		--region=asia-northeast1 \
		--image=asia.gcr.io/icfpc-primary/server \
		server
