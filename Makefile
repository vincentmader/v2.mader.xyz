up:
	docker-compose up -d --build
down:
	docker-compose stop
logs:
	docker-compose logs -f -t
