set -euo pipefail

if [ "$(uname -m)" = "arm64" ]; then
    export PATH=$PATH:/opt/homebrew/bin
fi

mint "$@"
