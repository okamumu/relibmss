import relibmss as ms

# Define gates
def gate1(mss, x, y):
    return mss.switch([
        mss.case(cond=mss.And([x == 0, y == 0]), then=0),
        mss.case(cond=mss.Or([x == 0, y == 0]), then=1),
        mss.case(cond=mss.Or([x == 2, y == 2]), then=3),
        mss.case(then=2) # default
    ])

def gate2(mss, x, y):
    return mss.switch([
        mss.case(cond=x == 0, then=0),
        mss.case(then=y)
    ])

mss = ms.MSS()

A = mss.defvar('A', 2)
B = mss.defvar('B', 3)
C = mss.defvar('C', 3)

sx = gate1(mss, B, C)
ss = gate2(mss, A, sx)

mdd = mss.minpath(ss)
print(mdd.dot())
for path in mdd.extract([0,1,2], type='mdd'):
    print(path)

# An example of the direct use of MDD

mdd = ms.MDD()

A = mdd.defvar('A', 2)
B = mdd.defvar('B', 3)
C = mdd.defvar('C', 3)

sx = gate1(mdd, B, C)
ss = gate2(mdd, A, sx)

mdd = ss.minpath()
print(mdd.dot())
for path in mdd.extract([0,1,2], type='mdd'):
    print(path)

