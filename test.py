import pandas as pd
from time import time
s = time()
df = pd.read_csv('test.csv')
print(time()-s)
# print(df.iloc[:,1:].corrwith(df.iloc[:,0]))