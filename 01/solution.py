from pathlib import Path

text = Path('input1').read_text()

elves = text.split('\n\n')

print(len(elves))

cals = [sum(map(int, x.split())) for x in elves]

print(max(cals))