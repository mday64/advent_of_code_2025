#!python3
import sys
import os

# Convert the input to the form "dayNN", where "NN" is a two-digit
# number with leading zero.
day = int(sys.argv[1])
day_name = f"day{day:02}"
os.mkdir(day_name)

# Copy the _template directory tree, editing files by replacing
# "dayNN" with the name of the new directory
src_path = "_template"
for (src_dir, dir_names, file_names) in os.walk(src_path):
    dest_dir = src_dir.replace("_template", day_name, 1)
    for dir_name in dir_names:
        os.mkdir(os.path.join(dest_dir, dir_name))
    for file_name in file_names:
        contents = open(os.path.join(src_dir, file_name)).read()
        contents = contents.replace("dayNN", day_name)
        open(os.path.join(dest_dir, file_name), mode="x").write(contents)
