import relibmss as ms
import numpy as np

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    
    def dist(self, p):
        return np.sqrt((self.x - p.x)**2 + (self.y - p.y)**2)

np.random.seed(1234)

m = 50
n = 100
ps = {'p'+str(i): Point(np.random.rand(), np.random.rand()) for i in range(m)}
# sort ps based on the distance from (0,0)
sortedps = [k for (k,v) in sorted(ps.items(), key=lambda x: x[1].dist(Point(0,0)))]
print(sortedps)

bss = ms.BSS(vars=sortedps) # create an instance with the variable order
vars = {'p'+str(i): bss.defvar('p'+str(i)) for i in range(m)}

grid = {'grid_{}_{}'.format(i,j): Point(x,y) for (i,x) in enumerate(np.linspace(0,1,n)) for (j,y) in enumerate(np.linspace(0,1,n))}

r = 0.5
result = []
for (k,v) in grid.items():
    nm = [vars[pn] for (pn, pv) in ps.items() if pv.dist(v) <= r]
    result.append(bss.Or(nm))
expr = bss.And(result)

# print(str(expr))

bdd = bss.getbdd(expr)
print(bdd.size())

paths = bdd.minpath()
print(paths.count())

prob = {'p'+str(i): 0.9 for i in range(m)}

print(bdd.prob(prob))

# An example of the direct use of BDD

np.random.seed(1234)

ps = {'p'+str(i): Point(np.random.rand(), np.random.rand()) for i in range(m)}
sortedps = [k for (k,v) in sorted(ps.items(), key=lambda x: x[1].dist(Point(0,0)))]
print(sortedps)

bdd = ms.BDD(vars=sortedps) # create an instance with the variable order
vars = {'p'+str(i): bdd.defvar('p'+str(i)) for i in range(m)}

grid = {'grid_{}_{}'.format(i,j): Point(x,y) for (i,x) in enumerate(np.linspace(0,1,n)) for (j,y) in enumerate(np.linspace(0,1,n))}

r = 0.5
result = []
for (k,v) in grid.items():
    nm = [vars[pn] for (pn, pv) in ps.items() if pv.dist(v) <= r]
    result.append(bdd.Or(nm))
expr = bdd.And(result)

print(expr.size())

paths = expr.minpath()
print(paths.count())

prob = {'p'+str(i): 0.9 for i in range(m)}

print(expr.prob(prob))
