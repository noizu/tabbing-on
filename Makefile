INSTALL_DIR := $(HOME)/.local/bin
SHARE_DIR   := $(HOME)/.local/share/tabbing-on

.PHONY: compile test install uninstall

compile:
	@true

test:
	@true

install:
	@mkdir -p $(INSTALL_DIR) $(SHARE_DIR)/lib $(SHARE_DIR)/shell
	@for f in bin/*; do \
		src=$$(realpath "$$f"); dst=$$(realpath "$(INSTALL_DIR)/$$(basename $$f)" 2>/dev/null); \
		if [ "$$src" = "$$dst" ]; then \
			echo "$$(basename $$f): same file — skipping"; \
		else \
			install -m 755 "$$f" $(INSTALL_DIR)/; \
		fi; \
	done
	@install -m 644 lib/*.sh $(SHARE_DIR)/lib/
	@install -m 644 shell/tabbing.bash shell/tabbing.zsh $(SHARE_DIR)/shell/
	@echo "Installed: bin → $(INSTALL_DIR), lib+shell → $(SHARE_DIR)"

uninstall:
	@for f in bin/*; do rm -f "$(INSTALL_DIR)/$$(basename $$f)"; done
	@rm -rf $(SHARE_DIR)
	@echo "Removed tabbing-on from $(INSTALL_DIR) and $(SHARE_DIR)"
