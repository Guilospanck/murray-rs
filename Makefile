.PHONY: clippy
clippy:
	cargo clippy --all-targets -- -D warnings

.PHONY: check
check:
	cargo check --all-targets

.PHONY: test
test:
	cargo test --tests

##! Tag and push it. Example: ❯ make tag-and-push new_tag=v0.0.2
.PHONY: tag-and-push
tag-and-push:
	git tag -a ${new_tag} && git push origin ${new_tag}

##! Generate new test coverage
.PHONY: coverage
coverage:
	./generate_coverage.sh

.PHONY: open-coverage
open-coverage:
	open -a Google\ Chrome.app target/debug/coverage/index.html

.PHONY: coverage-and-open
coverage-and-open: coverage open-coverage

##! Generate docs
.PHONY: generate-docs
generate-docs:
	cargo doc --open --no-deps --package murray-rs