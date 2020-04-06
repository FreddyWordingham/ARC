import json
import sys
import numpy as np
from collections import OrderedDict

predicted_laser = 2740.0/4.0
predicted_raman = 2360.0/4.0

with open("input/raman/materials/intralipid.json","r") as jsonFile:
    data = json.load(jsonFile, object_pairs_hook=OrderedDict)

optics = data["optics"]
scat = optics["scat_coeff"]
bifur = scat["Bifurcation"]
#print(bifur)
ratio = np.float(sys.argv[1])/predicted_laser
bifur["a"] = np.float(sys.argv[1])
bifur["b"] = ratio*predicted_raman
#print(bifur)
with open("input/raman/materials/intralipid.json", "w") as jsonFile:
   json.dump(data, jsonFile)
