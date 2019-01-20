
default: clean staticcheck test build

.PHONY:
checkstyle:
	@echo ">> checking code style"
	@fmtRes=$$(gofmt -l $$(find . -iname '*.go' -print)); \
	if [ -n "$${fmtRes}" ]; then \
		echo "gofmt checking failed!"; echo "$${fmtRes}"; echo; \
		echo "Please ensure you are using $$(go version) for formatting code."; \
		exit 1; \
	fi
	@echo "Code Style successfully checked"

.PHONY: lint
lint:
	@echo ">> try to lint"
	@lintRes=$$(golint ./...); \
	if [ -n "$${lintRes}" ]; then \
		echo "golint checking failed!"; echo "$${lintRes}"; echo; \
		exit 1; \
	fi
	@echo "Linting was successful"

.PHONY: vet
vet:
	@echo ">> try to vet"
	@vetRes=$$(go vet ./...); \
	if [ -n "$${vetRes}" ]; then \
		echo "go vet checking failed!"; echo "$${vetRes}"; echo; \
		exit 1; \
	fi
	@echo "Vet was successful"

.PHONY: staticcheck
staticcheck: checkstyle lint vet

.PHONY: test
test:
	@echo ">> running all unit tests"
	go test ./...

.PHONY: build
build:
	@echo ">> building binaries"
	go build -o out/tmlr ./cmd/tmlr-client/main.go

.PHONY: run
run: build
	./out/tmlr

.PHONY: clean
clean:
	rm -rf out/
	go clean ./cmd/tmlr-client

.PHONY: help
help:
	@echo "default		clean staticcheck test build"
	@echo "------"
	@echo "clean		cleans the output folder (out/)"
	@echo "checkstyle	executes checkstyle on all go files"
	@echo "lint		lints all go files"
	@echo "vet		executes vet on all go files"
	@echo "staticcheck	combines checkstyle, lint, vet"
	@echo "test		executes all unit tests"
	@echo "build		builds the project"
	@echo "run		builds and runs the project"