.PHONY: phpmyadmin
phpmyadmin:
	docker build --build-arg=UNAGI_PASSWORD=$${UNAGI_PASSWORD} \
		-t asia.gcr.io/icfpc-primary/phpmyadmin -f phpmyadmin.Dockerfile .
