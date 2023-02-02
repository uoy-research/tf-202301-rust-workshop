#!/usr/bin/env bash
# This script piulld down some test FASTA data from Gencode

# Extract the entire genome:
curl --output genome.fasta.gz https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_42/GRCh38.primary_assembly.genome.fa.gz
gunzip genome.fasta.gz

# Pull out a small subset of chr1:
head -100001 genome.fasta > small.fasta
