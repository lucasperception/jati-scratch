from Bio import AlignIO
import pathlib
import sys

path = sys.argv[1]
input_file = pathlib.Path(path)
output_file = sys.argv[2]
fasta = AlignIO.read(input_file, "fasta")
AlignIO.write(fasta, output_file, "phylip-relaxed")


