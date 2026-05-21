INSTALL_DIR := $(HOME)/.local/bin

.PHONY: compile test install

compile:
	@true

test:
	@true

install:
	@mkdir -p $(INSTALL_DIR)
	@for f in bin/*; do \
		src=$$(realpath "$$f"); dst=$$(realpath "$(INSTALL_DIR)/$$(basename $$f)" 2>/dev/null); \
		if [ "$$src" = "$$dst" ]; then \
			echo "$$(basename $$f): same file — skipping"; \
		else \
			install -m 755 "$$f" $(INSTALL_DIR)/; \
		fi; \
	done
