up:
	docker-compose up -d --build
down:
	docker-compose down
logs:
	docker-compose logs -f -t
rocket:
	cd bin && ./build_client --release
	./entrypoint.sh
