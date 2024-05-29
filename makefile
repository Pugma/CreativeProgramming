setup:
	docker compose -f dockerfiles/compose.yaml up -d --build
clean:
	docker compose -f dockerfiles/compose.yaml down
clean-all:
	docker compose -f dockerfiles/compose.yaml down --rmi all --volumes --remove-orphans