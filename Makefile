include .env

build-linux-x86:
	@echo "Build and cross compile release for linux x86 with glibc"
	RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc' cargo build --target x86_64-unknown-linux-gnu --release;

	@echo "Push to Artifactory"
	@curl -u $(ARTIFACTORY_USER):$(ARTIFACTORY_PASS) -X PUT https://room303.jfrog.io/artifactory/rust-binary/cli-cross-x86_64-1.1.3 -T /app/target/x86_64-unknown-linux-gnu/release/cli-cross-x86
