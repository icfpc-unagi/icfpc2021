.PHONY: build
build:
	docker build \
		--build-arg=UNAGI_PASSWORD=$$UNAGI_PASSWORD \
		--build-arg=UNAGI_API_KEY=$$UNAGI_API_KEY \
		-t server .

.PHONY: run
run: build
	docker run -p 8080:8080 --name=server -ti --init --rm server

.PHONY: push
push:
	docker tag server asia.gcr.io/icfpc-primary/server
	docker push asia.gcr.io/icfpc-primary/server
	gcloud --project=icfpc-primary run services update \
		--region=asia-northeast1 \
		--image=asia.gcr.io/icfpc-primary/server \
		server
