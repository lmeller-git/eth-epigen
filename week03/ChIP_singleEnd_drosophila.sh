#!/bin/bash

set -euo pipefail

ref=./data/BDGP6_genome.fasta.gz
adapters=./data/TruSeq3-PE.fa

# rm -rf data/trimmed data/aligned data/tracks data/peaks

mkdir -p data/trimmed
mkdir -p data/aligned
mkdir -p data/tracks
mkdir -p data/peaks

f=./data/ENCFF127RRR.fastq.gz

base=`basename $f .fastq.gz`
bam=data/aligned/"$base".bam
echo $base

# TRIMMING

trimdir=data/trimmed

if [ -f "$trimdir/$base.fastq.gz" ]; then
  echo $trimdir/$base".fastq.gz found; skipping"
else
trimmomatic SE -threads 6 -summary "$trimdir/$base.stats" -phred33 $f \
  "$trimdir/$base.fastq.gz" ILLUMINACLIP:$adapters:2:15:4:4:true \
  LEADING:20 TRAILING:20 SLIDINGWINDOW:4:15 MINLEN:25
fi

# ALIGNMENT

aligned=data/aligned

if ![ -f "data/bowtie2*" ]; then
  bowtie2-build --threads 4 $ref data/bowtie2
fi

if [ -f "$bam" ]; then
    echo "$bam found; skipping"
else
(bowtie2 -p 4 -x data/bowtie2 -U "$trimdir/$base.fastq.gz") 2> "$aligned/$base.bowtie2" |\
  samtools view -bS - | samtools sort -@4 -m 2G - > $bam
samtools index $bam
fi

# PEAKS

peaks=data/peaks

if [ -f "peaks/${base}_peaks.narrowPeak" ]; then
    echo "Peaks found; skipping"
else
    macs3 callpeak -t $bam -f BAM -n $base -g dm --outdir $peaks
fi
# macs3 callpeak --outdir peaks -n ttk --gsize dm -t aligned/ttk.bam -c aligned/input.bam

# COVERAGE TRACKS

tracks=data/tracks

if [ -f "tracks/$base.bw" ]; then
    echo "tracks/$base.bw found; skipping"
else
  bamCoverage -p 6 --ignoreDuplicates --effectiveGenomeSize 142573017 --normalizeUsing CPM \
    -b "$aligned/$base.bam" -o "$tracks/$base.bw"
fi

