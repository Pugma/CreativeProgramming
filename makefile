setup:
	docker compose -f dockerfiles/compose.yaml up -d --build
clean:
	docker compose down