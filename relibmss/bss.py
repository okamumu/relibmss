import warnings
import relibmss as ms

def _to_rpn(expr):
    stack = [expr]
    rpn = []
    cache = set([])
    while len(stack) > 0:
        node = stack.pop()
        idnum = str(id(node))
        if isinstance(node, tuple) and node[0] == "save" and len(stack) > 0:
            idnum = node[1]
            rpn.append('save({})'.format(idnum))
            cache.add(idnum)
        elif isinstance(node, tuple) and node[0] == "save":
            pass
        elif idnum in cache:
            rpn.append('load({})'.format(idnum))
        elif isinstance(node.value, tuple):
            stack.append(("save", idnum))
            for i in range(len(node.value) - 1, -1, -1):
                stack.append(node.value[i])
        else:
            rpn.append(str(node.value))
    return ' '.join(rpn)

class _Expression:
    def __init__(self, value):
        self.value = value

    def __and__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('&')))
    
    def __or__(self, other):
        if not isinstance(other, _Expression):
            other = _Expression(other)
        return _Expression((self, other, _Expression('|')))

    def __str__(self):
        if isinstance(self.value, tuple):
            return _to_rpn(self)
        return str(self.value)

    def _to_rpn(self):
        if isinstance(self.value, tuple):
            return _to_rpn(self)
        return str(self.value)

class Context:
    def __init__(self, vars=[]):
        self.vars = set([])
        self.bdd = ms.BDD()
        for varname in vars:
            self.vars.add(varname)
            self.bdd.defvar(varname)

    def defvar(self, name):
        self.vars.add(name)
        return _Expression(name)
    
    def get_varorder(self):
        return self.bdd.get_varorder()

    # def set_varorder(self, x: list):
    #     for varname in x:
    #         self.bdd.defvar(varname)

    def __str__(self):
        return str(self.vars)
    
    def getbdd(self, arg: _Expression):
        if not isinstance(arg, _Expression):
            arg = _Expression(arg)
        rpn = arg._to_rpn()
        return self.bdd.rpn(rpn)
    
    def const(self, value):
        return _Expression(value)

    def And(self, args: list):
        assert len(args) > 0
        if not isinstance(args[0], _Expression):
            args[0] = _Expression(args[0])
        if len(args) == 1:
            return args[0]
        x = args[0]
        for y in args[1:]:
            if not isinstance(y, _Expression):
                y = _Expression(y)
            x = _Expression((x, y, _Expression('&')))
        return x

    def Or(self, args: list):
        assert len(args) > 0
        if not isinstance(args[0], _Expression):
            args[0] = _Expression(args[0])
        if len(args) == 1:
            return args[0]
        x = args[0]
        for y in args[1:]:
            if not isinstance(y, _Expression):
                y = _Expression(y)
            x = _Expression((x, y, _Expression('|')))
        return x

    def Not(self, arg: _Expression):
        if not isinstance(arg, _Expression):
            arg = _Expression(arg)
        return _Expression((arg, _Expression('!')))

    def ifelse(self, condition: _Expression, then_expr: _Expression, else_expr: _Expression):
        if not isinstance(condition, _Expression):
            condition = _Expression(condition)
        if not isinstance(then_expr, _Expression):
            then_expr = _Expression(then_expr)
        if not isinstance(else_expr, _Expression):
            else_expr = _Expression(else_expr)
        return _Expression((condition, then_expr, else_expr, _Expression('?')))

    def kofn(self, k: int, args: list):
        assert k <= len(args)
        if k == 1:
            return self.Or(args)
        elif k == len(args):
            return self.And(args)
        else:
            return self.ifelse(args[0], self.kofn(k-1, args[1:]), self.kofn(k, args[1:]))
    
    def prob(self, arg: _Expression, values: dict, ss = [True]):
        warnings.warn("This function is obsolete. Use the method of BddNode directly.", category=DeprecationWarning)
        top = self.getbdd(arg)
        return top.prob(values, ss)
    
    def bmeas(self, arg: _Expression, values: dict, ss = [True]):
        warnings.warn("This function is obsolete. Use the method of BddNode directly.", category=DeprecationWarning)
        top = self.getbdd(arg)
        return top.bmeas(values, ss)

    def prob_interval(self, arg: _Expression, values: dict, ss = [True]):
        warnings.warn("This function is obsolete. Use the method of BddNode directly.", category=DeprecationWarning)
        top = self.getbdd(arg)
        return top.prob_interval(values, ss)

    def bmeas_interval(self, arg: _Expression, values: dict, ss = [True]):
        warnings.warn("This function is obsolete. Use the method of BddNode directly.", category=DeprecationWarning)
        top = self.getbdd(arg)
        return top.bmeas_interval(values, ss)

    def minpath(self, arg: _Expression):
        warnings.warn("This function is obsolete. Use the method of BddNode directly.", category=DeprecationWarning)
        top = self.getbdd(arg)
        return top.minpath()

