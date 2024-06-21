dev:
	docker-compose -f docker-compose.dev.yml up -d --build
	docker exec -it my_blog bash

release:
	docker-compose -f docker-compose.build.yml up -d --build

clear:
	docker-compose -f docker-compose.dev.yml down
	docker-compose -f docker-compose.build.yml down
	docker system prune -a -f
	docker volume prune -a -f

deploy_img:
	docker-compose -f docker-compose.build.yml up -d --build
	docker-compose -f docker-compose.build.yml push