# 必要なライブラリのインポート
import numpy as np
import relibmss as ms
from functools import reduce

# def for a gate with switch-case structure
def max_gate(mss, x, y):
  return mss.switch([
        mss.case(cond= x < y, then=y),
        mss.case(then=x) # default
    ])

def min_gate(mss, x, y):
  return mss.switch([
        mss.case(cond= x < y, then=x),
        mss.case(then=y) # default
    ])

def var2(mss, x):
  return mss.switch([
        mss.case(cond= x == 0, then=0),
        mss.case(then=x-1) # default
    ])

def maxs(mss, x):
  return reduce(lambda a, b: max_gate(mss, a, b), x, mss.const(0))

def mins(mss, x):
  return reduce(lambda a, b: min_gate(mss, a, b), x, mss.const(2))

# 基地局を表すクラスの定義

class Base:
  def __init__(self, name, p, r1, r2):
    # name: ラベル，p(x, y); 座標，r1: 範囲, r2 (r2 > r1)
    self.name = name
    self.p = p
    self.r1 = r1
    self.r2 = r2

  def __repr__(self) -> str:
    return f"Base({self.name}, {self.p}, {self.r1}, {self.r2})"

  def check_dist1(self, p):
    # 与えられた座標p(x,y)が範囲内かどうかを判定する
    # pはnumpy array
    return np.linalg.norm(self.p - p) <= self.r1

  def check_dist2(self, p):
    # 与えられた座標p(x,y)が範囲内かどうかを判定する
    # pはnumpy array
    return self.r1 < np.linalg.norm(self.p - p) <= self.r2

def get_bases(base_list, p):
  # 与えられた座標p(x,y)をカバーする基地局のリストを返す
  bases1 = [b for b in base_list if b.check_dist1(p)]
  bases2 = [b for b in base_list if b.check_dist2(p)]
  return (bases1, bases2)

def get_bases2(base_list, ps):
  # 与えられた座標集合ps={p1, p2, ...}が作るエリアをカバーする基地局のリストを返す
  #    全部の点をカバーしていればエリアをカバーすることになる
  bases1 = [b for b in base_list if all([b.check_dist1(p) for p in ps])]
  bases2 = [b for b in base_list if all([b.check_dist1(p) or b.check_dist2(p) for p in ps])]
  bases2 = [b for b in bases2 if b not in bases1]
  return (bases1, bases2)

np.random.seed(12247) # 乱数シードの設定
r1 = 0.5 # カバーする半径（この場合は全部の基地が同じ半径）
r2 = 1.0 # カバーする半径（この場合は全部の基地が同じ半径）
n = 10 # 基地数
base_list = [Base('base'+str(i), np.random.rand(2), r1, r2) for i in range(n)] # ランダムに[0,1] x [0,1]エリアに基地をn個配置

orders = [(x[1][0], 3) for x in enumerate(sorted([(b.name, np.linalg.norm(b.p)) for b in base_list], key=lambda x: x[1]))]
print(orders)

mss = ms.MSS(vars=orders)
vars = {b.name: mss.defvar(b.name, 3) for b in base_list} # 基地に対応する変数の作成
prob = {b.name: [0.01, 0.3, 0.69] for b in base_list} # 各基地が故障しない確率

gn = 5 # グリッド数 gn x gn 個の点を均等に配置
grid_x = np.linspace(0, 1, gn)
grid_y = np.linspace(0, 1, gn)
# 点座標毎 [[0,0], [0,0.1], ...] のようなデータ
grid = [np.array([x, y]) for x in grid_x for y in grid_y]
# エリア毎の座標集合のデータ [[[0,0], [0,0.1], [0.1, 0]], [[0,0.1], [0.1,0], [0.1,0.1]], ...]
grid2 = [np.array([[grid_x[xi], grid_y[yi]], [grid_x[xi], grid_y[yi-1]], [grid_x[xi-1], grid_y[yi]], [grid_x[xi-1], grid_y[yi-1]]]) for xi in range(1, gn) for yi in range(1, gn)]

def make_tree(mss, g, base_list):
  bases1 = [vars[b.name] for b in get_bases(base_list, g)[0]]
  bases2 = [vars[b.name] for b in get_bases(base_list, g)[1]]
  return max_gate(mss, maxs(mss, bases1), var2(mss, maxs(mss, bases2)))

xs1 = [make_tree(mss, g, base_list) for g in grid]
sy1 = mins(mss, xs1)

def make_tree2(mss, gs, base_list):
  bases1 = [vars[b.name] for b in get_bases2(base_list, gs)[0]]
  bases2 = [vars[b.name] for b in get_bases2(base_list, gs)[1]]
  return max_gate(mss, maxs(mss, bases1), var2(mss, maxs(mss, bases2)))

xs2 = [make_tree2(mss, gs, base_list) for gs in grid2]
sy2 = mins(mss, xs2)

# print(xs1[0].to_rpn())

# print(sy1.to_rpn())

mdd = mss.getmdd(sy1)

print(mdd.size())
# print(mdd.dot())

## An example of the direct use of MDD

np.random.seed(12247) # 乱数シードの設定
r1 = 0.5 # カバーする半径（この場合は全部の基地が同じ半径）
r2 = 1.0 # カバーする半径（この場合は全部の基地が同じ半径）
n = 10 # 基地数
base_list = [Base('base'+str(i), np.random.rand(2), r1, r2) for i in range(n)] # ランダムに[0,1] x [0,1]エリアに基地をn個配置
orders = [(x[1][0], 3) for x in enumerate(sorted([(b.name, np.linalg.norm(b.p)) for b in base_list], key=lambda x: x[1]))]

mdd = ms.MDD(vars=orders)
vars = {b.name: mdd.defvar(b.name, 3) for b in base_list} # 基地に対応する変数の作成
prob = {b.name: [0.01, 0.3, 0.69] for b in base_list} # 各基地が故障しない確率

gn = 5 # グリッド数 gn x gn 個の点を均等に配置
grid_x = np.linspace(0, 1, gn)
grid_y = np.linspace(0, 1, gn)
# 点座標毎 [[0,0], [0,0.1], ...] のようなデータ
grid = [np.array([x, y]) for x in grid_x for y in grid_y]
# エリア毎の座標集合のデータ [[[0,0], [0,0.1], [0.1, 0]], [[0,0.1], [0.1,0], [0.1,0.1]], ...]
grid2 = [np.array([[grid_x[xi], grid_y[yi]], [grid_x[xi], grid_y[yi-1]], [grid_x[xi-1], grid_y[yi]], [grid_x[xi-1], grid_y[yi-1]]]) for xi in range(1, gn) for yi in range(1, gn)]

def make_tree(mdd, g, base_list):
  bases1 = [vars[b.name] for b in get_bases(base_list, g)[0]]
  bases2 = [vars[b.name] for b in get_bases(base_list, g)[1]]
  return max_gate(mdd, maxs(mdd, bases1), var2(mdd, maxs(mdd, bases2)))

xs1 = [make_tree(mdd, g, base_list) for g in grid]
sy1 = mins(mdd, xs1)

def make_tree2(mdd, gs, base_list):
  bases1 = [vars[b.name] for b in get_bases2(base_list, gs)[0]]
  bases2 = [vars[b.name] for b in get_bases2(base_list, gs)[1]]
  return max_gate(mdd, maxs(mdd, bases1), var2(mdd, maxs(mdd, bases2)))

xs2 = [make_tree2(mdd, gs, base_list) for gs in grid2]
sy2 = mins(mdd, xs2)

print(sy1.size())
# print(sy1.dot())
