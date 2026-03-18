#! /bin/bash

set -euo pipefail

declare -A files

files["ENCFF350GCX"]="p300_mESC"
files["ENCFF360VIS"]="H3K27ac_mESC"
files["ENCFF558YNC"]="H3K27me3_mESC"
files["ENCFF625YHS"]="p300_Heart"
files["ENCFF814SUK"]="H3K27ac_Heart"
files["ENCFF110HRW"]="H3K27me3_Heart"

mkdir -p data

for f in "${!files[@]}"; do
  name="${files[$f]}"
  if [ ! -f "data/${name}.bed" ]; then
    curl -L -o "data/"${name}".bed.gz" "https://www.encodeproject.org/files/${f}/@@download/${f}.bed.gz"
    gunzip "data/"${name}".bed.gz"
  fi
done
