INSTALL_DIR := $(HOME)/.local/bin

.PHONY: compile test install

compile:
	@true

test:
	@true

install:
	@mkdir -p $(INSTALL_DIR)
	cp bin/* $(INSTALL_DIR)/
	chmod +x $(INSTALL_DIR)/tabbing-*
	chmod +x $(INSTALL_DIR)/_tabbing-*
	chmod +x $(INSTALL_DIR)/demo-runner
