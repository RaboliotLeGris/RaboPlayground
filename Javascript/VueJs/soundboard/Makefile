.DEFAULT_GOAL := build
.PHONY: build clean build-front build-back

build: build-front build-back

build-front:
	cd Front
	npm i
	npm run build

build-back:
	cd ../Backend
	cargo build --release

clean:
	rm -rf Front/dist
	rm -rf Front/node_modules
	rm -rf Backend/target

docker: clean build
	docker build .

