temp_dir=$(mktemp -d)
cp "$1" "$temp_dir"/
cd "$temp_dir" || exit
if [ "$2" = "nucleotide" ]; then
  echo "Running nucleotide analysis for $1"
  /bin/phyml -i "$(basename "$1")" -d nt -m GTR -f e -c 2 -a 1 -o tl
elif [ "$2" = "protein" ]; then
  echo "Running protein analysis for $1"
  /bin/phyml -i "$(basename "$1")" -d aa -m LG -f e -c 2 -a 1 -o tl
fi
cd - || exit
rm -rf "$temp_dir"
