#%% imports
import csv
import os
from traceback import print_tb

#%% Path to the data file
data_path = os.path.join(os.path.dirname(__file__), 'optimized_games.csv')

#%% Reading the data file
with open(data_path) as data_file:
    data = data_file.read()

#%% Splitting the data into lines and then into sets of numbers
sets = [set(map(int, line.split(','))) for line in data.strip().split('\n')]

#%% Finding the largest intersection length
max_intersection_length = 0

for i in range(len(sets)):
    for j in range(i + 1, len(sets)):
        intersection_length = len(sets[i].intersection(sets[j]))
        if intersection_length > max_intersection_length:
            max_intersection_length = intersection_length


# %%
assert max_intersection_length < 3, f"{max_intersection_length=}"

# %% test that no set contains a number smaller or equal to 31:
failed = False
for i in range(len(sets)):
    if any([num <= 31 for num in sets[i]]):
        print(f"Failure for set {sets[i]}`")
        failed = True
assert not failed, "Failure encontered above"


print(f"Tested games are OK! Finished {__file__} successfully")