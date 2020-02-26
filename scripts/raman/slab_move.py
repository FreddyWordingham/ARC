import json
import sys
import numpy as np
from collections import OrderedDict

with open("input/raman/surfaces/slab.json","r") as jsonFile:
    data = json.load(jsonFile, object_pairs_hook=OrderedDict)

outerTrans = data["trans"]
innerTrans = outerTrans["trans"]
#print(innerTrans)
innerTrans[0] = np.float(sys.argv[1])
#print(innerTrans)
#print(outerTrans)
#print("data: ", data)
with open("input/raman/surfaces/slab.json", "w") as jsonFile:
    json.dump(data, jsonFile)
