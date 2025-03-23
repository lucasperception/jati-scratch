temp_dir=$(mktemp -d)
cp "$1" "$temp_dir"/
cd "$temp_dir" || exit
if [ "$2" = "nucleotide" ]; then
  echo "Running nucleotide analysis for $1"
  /bin/raxml-ng --msa "$(basename "$1")" --tree random{1} --model GTR+G --opt-branches off
elif [ "$2" = "protein" ]; then
  echo "Running protein analysis for $1"
  /bin/raxml-ng --msa "$(basename "$1")" --tree random{1} --model LG+G --opt-branches off
fi
cd - || exit 1
rm -rf "$temp_dir"
