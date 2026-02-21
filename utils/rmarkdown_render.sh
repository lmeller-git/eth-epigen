#! /bin/bash

set -euo pipefail

OUTDIR="$PWD"

render() {
  local file="$1"
  local outfile="$OUTDIR/$(basename "${file%.*}.html")"
  echo "rendering $file to $outfile..."
  Rscript -e "rmarkdown::render('$file', output_file='$outfile')"
  echo "Done"
}

if [ ! -d "$OUTDIR" ]; then
  mkdir -p "$OUTDIR"
fi

for path in "$@"; do
    if [ -d "$path" ]; then
        find "$path" -type f -name "*.Rmd" | while read -r file; do
            render "$file"
        done
    elif [ -f "$path" ] && [[ "$path" == *.Rmd ]]; then
        render "$path"
    else
        echo "skipping $path"
    fi
done


