VERSION_MAJOR :=0
VERSION_MINOR :=0
VERSION_PATCH :=1
VERSION_BUILD :=$(or ${VERSION_BUILD},$(shell git rev-parse --short HEAD))
VERSION       :=v$(VERSION_MAJOR).$(VERSION_MINOR).$(VERSION_PATCH)+$(VERSION_BUILD)

LDFLAGS       :="-s -w -X 'main.VERSION=$(VERSION)'"

XRAY_DISABLED := 'true'

.PHONY: build lint deps build.linux clean docker.image test fmt vet generate deploy*

build:
	go build -o bin/application -ldflags=$(LDFLAGS) ./cmd/app

fmt:
	go fmt ./...

.PHONY: formatting-check
formatting-check:
	@gofmt -l -d $$(find . -not -path '*/\.*' -type f -name '*.go')  | grep ^ && exit 1 || true > /dev/null 2>&1

lint:
	golint -set_exit_status ./...

vet:
	go vet ./...

test:
	AWS_XRAY_SDK_DISABLED=$(XRAY_DISABLED) go test -race -cover -coverprofile=coverage.out ./...
	go tool cover -html=coverage.out -o coverage.html

build.linux:
	GOARCH=amd64 GOOS=linux go build -o bin/application -ldflags=$(LDFLAGS) ./cmd/app

clean:
	rm -f bin/application

docker.image:
	docker build -t fpjs/api -f Dockerfile .

generate:
	go generate ./...

deploy.staging: build.linux
	eb deploy fpjs-api-staging2 -r us-east-2
