import relibmss as ms

def test_test1():
    mdd = ms.MddMgr()
    x = mdd.defvar("x", [0, 1])
    print(x.dot())

def test_test2():
    mdd = ms.MddMgr()
    x = mdd.defvar("x", range(2))
    y = mdd.defvar("y", range(3))
    z = mdd.defvar("z", range(3))
    v = mdd.rpn("x y z * +")
    print(v.dot())

def test_case3():
    context = ms.SymbolicMgr()
    x = context.var("x", range(2))
    y = context.var("y", range(3))
    z = context.var("z", range(3))
    v = x * y + z
    n1 = v.mdd()
    print(n1.dot())
