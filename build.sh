cd "$(dirname "$0")"

mkdir -p "$(pwd)/www/public/"

CSS_FILE="$(pwd)/www/public/app.css"
OUTPUT_CSS=$CSS_FILE wasm-pack build
