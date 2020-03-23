import json
import sys
import numpy as np
from collections import OrderedDict

with open("input/raman/materials/intralipid.json","r") as jsonFile:
    data = json.load(jsonFile, object_pairs_hook=OrderedDict)

optics = data["optics"]
scat = optics["scat_coeff"]
bifur = scat["Bifurcation"]
#print(bifur)
bifur["a"] = np.float(sys.argv[1])
bifur["b"] = np.float(sys.argv[1]) - 50.0
#print(bifur)
with open("input/raman/materials/intralipid.json", "w") as jsonFile:
   json.dump(data, jsonFile)
