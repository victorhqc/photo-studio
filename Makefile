bake:
	docker build -t photo-studio-musl:latest .

build:
	./scripts/build.sh photo-api $(version)

build_in_github:
	./scripts/build_in_github.sh
