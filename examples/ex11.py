import relibmss as ms

mdd = ms.MDD()

c = 10
link1 = mdd.defvar("link1", c)
link2 = mdd.defvar("link2", c)
link3 = mdd.defvar("link3", c)
link4 = mdd.defvar("link4", c)
link5 = mdd.defvar("link5", c)

p1 = mdd.Max([link1, link2])
p2 = mdd.Max([link1, link3, link5])
p3 = mdd.Max([link4, link5])
p4 = mdd.Max([link4, link3, link2])
ss = mdd.Max([p1, p2, p3, p4])

print(ss.size())
print(ss.count(values=[0, 1, 2, 3, 4, 5, 6, 7, 8]))

pv = {"link1": [0.1 for _ in range(10)],
    "link2": [0.1 for _ in range(10)],
    "link3": [0.1 for _ in range(10)],
    "link4": [0.1 for _ in range(10)],
    "link5": [0.1 for _ in range(10)]}

print(ss.prob(probability=pv, values=[0, 1, 2, 3, 4, 5, 6, 7, 8]))
