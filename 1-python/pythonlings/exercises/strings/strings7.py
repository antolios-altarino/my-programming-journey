# Exercise: Strings 7
#
# Use .split() to break `csv_line` on commas and assign the resulting list
# to `items`. Then use ", ".join() to reassemble `items` back into a single
# string and assign it to `rejoined`.

csv_line = "apple,banana,cherry"
items = csv_line.split(",")
print(items)
rejoined = ", ".join(items)
print(rejoined)