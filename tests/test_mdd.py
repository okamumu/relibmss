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

