clean:
	rm ./test-rs-output-ts-input/schema.ts || true
	rm ./test-data/* || true
	@echo CLEANED

install-ts-test-deps: 
	rm -rf ./test-rs-output-ts-input/node_modules || true
	cd test-rs-output-ts-input && npm i

test: clean
	cargo test
	cd test-rs-output-ts-input && npm run test
