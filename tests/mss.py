import relibmss as ms

class Context:
    def __init__(self):
        self.vars = {}
        self.mdd = ms.MddMgr()

    def var(self, name, domain):
        self.vars[name] = domain
        return _Expression(name)
    
    def __str__(self):
        return str(self.vars)
    
    def getmdd(self, _Expression):
        rpn = _Expression.to_rpn()
        return self.mdd.rpn(rpn, self.vars)

class _Expression:
    def __init__(self, value):
        self.value = value

    def __add__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('+')))
    
    def __sub__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('-')))
    
    def __mul__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('*')))
    
    def __truediv__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('/')))
    
    def __eq__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('==')))
    
    def __ne__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('!=')))
    
    def __lt__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('<')))
    
    def __le__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('<=')))
    
    def __gt__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('>')))
    
    def __ge__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('>=')))
    
    def __str__(self):
        if isinstance(self.value, tuple):
            return ' '.join([x.to_rpn() for x in self.value])
        return str(self.value)

    def to_rpn(self):
        if isinstance(self.value, tuple):
            return ' '.join([x.to_rpn() for x in self.value])
        return str(self.value)

def And(args: list):
    if len(args) == 1:
        return args[0]
    x = args[0]
    for y in args[1:]:
        x = _Expression((x, y, _Expression('&&')))
    return x

def Or(args: list):
    if len(args) == 1:
        return args[0]
    x = args[0]
    for y in args[1:]:
        x = _Expression((x, y, _Expression('||')))
    return x

def Not(arg: _Expression):
    return _Expression((arg, _Expression('!')))

def IfThenElse(condition: _Expression, then_expr: _Expression, else_expr: _Expression):
    return _Expression((condition, then_expr, else_expr, _Expression('?')))
    

# 使用例
if __name__ == "__main__":
    ctx = Context()
    x = ctx.var("x", range(2))
    y = ctx.var("y", range(3))
    z = ctx.var("z", range(3))
    v = x * y + z
    v = And([x >= 1, y <= 1, z == 0])
    print(v)

    tree = ctx.mdd.rpn("x 1 >= y 1 <= &&", ctx.vars)
    tree2 = ctx.getmdd(v)

    print(tree.dot())
    print(tree2.dot())
