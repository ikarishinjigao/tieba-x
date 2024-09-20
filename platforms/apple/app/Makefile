# Output
infos = "\033[39m$(1)\033[0m"
success = "\033[92m✔  $(1)\033[0m"
failure = "\033[31m✖  $(1)\033[0m"

# Alias
MINT := ./scripts/mint.sh
XCODEGEN := $(MINT) run xcodegen
SWIFTFORMAT := $(MINT) run swiftformat
SWIFTLINT := $(MINT) run swiftlint

# Default step
default: help

.PHONY: setup
setup: ## Setup project
	@make install-tools
	@make generate-project
	@echo $(call success,Setup done! Happy coding! 🚀)

.PHONY: install-tools
install-tools: ## Install tools
	@echo $(call infos,Install tools for you...)
	@mint bootstrap
	@echo $(call success,All tools installed)

.PHONY: format-swift-files
format-swift-files: ## Format source files
	@$(SWIFTFORMAT) ./ --config .swift-format

.PHONY: lint-swift-files
lint-swift-files: ## Lint source files
	@$(SWIFTLINT) lint --config .swift-lint

.PHONY: generate-project
generate-project: ## Generate Xcode project
	@$(XCODEGEN) --spec Project.yml
	@echo $(call success,Project generated)

.PHONY: help
help: ## Show all task info
	@echo "$$(tput bold)Available tasks:$$(tput sgr0)"
	@egrep '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-40s\033[0m %s\n", $$1, $$2}'
