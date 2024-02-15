run-tutor-nodb-srv:
	cargo run -p tutor-nodb

run-tutor-db-srv:
	cargo run --bin iter1 -p tutordb

run-server:
	cargo run -p tutor-nodb --bin basic-server

start-db:
	docker-compose up -d

stop-db:
	docker-compose down