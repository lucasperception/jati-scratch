from pypythia.prediction import predict_difficulty
import pathlib
import sys

path = sys.argv[1]
msa = pathlib.Path(path)
difficulty = predict_difficulty(msa)
print(difficulty)
