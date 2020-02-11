GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
GOTEST=$(GOCMD) test
GOGET=$(GOCMD) get
BINARY_NAME=gilrain
BUILD_DIR=build

.DEFAULT_GOAL := run
.PHONY: all run build clean-build clean
build:
	$(GOBUILD) -o $(BUILD_DIR)/$(BINARY_NAME)

clean:
	$(GOCLEAN)
	rm -f $(BUILD_DIR)/$(BINARY_NAME)

deploy: clean build
	mkdir -p /tmp/gilrain
	cp $(BUILD_DIR)/$(BINARY_NAME) /tmp/gilrain/.

clean-build: clean build

run: build
	./$(BUILD_DIR)/$(BINARY_NAME)