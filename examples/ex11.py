# 必要なライブラリのインポート
import relibmss as ms

mss = ms.MDD()
x = mss.defvar("x", 3)
y = mss.defvar("y", 3)
z = mss.defvar("z", 3)

z1 = x + y 
z2 = z - x

print(z1 != z2)
try:
    print(z1 > z2)
except Exception as e:
    print(e)

tmp = z1.eq(z2)
print(tmp.dot())

tmp = z1.le(z2)
print(tmp.dot())



