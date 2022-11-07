clean:
	rm ./test-rs-output-ts-input/schema.ts
	@echo CLEANED

test: clean
	cargo test --features full
	cd test-rs-output-ts-input && npm run test
