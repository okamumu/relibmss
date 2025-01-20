import relibmss as ms

# Create a binary system (fault tree)
bss = ms.BSS()

# Define events (This version only supports repeated events)
A = bss.defvar('A')
B = bss.defvar('B')
C = bss.defvar('C')

# Make a tree
top = A & B | C # & is AND gate, | is OR gate

# Set probabilities
prob = {
    'A': 0.1,
    'B': 0.2,
    'C': 0.3
}

# Calculate the probability
print(bss.prob(top, prob))

# Set the interval of the probability
probint = {
    'A': (0.1, 0.2),
    'B': (0.2, 0.3),
    'C': (0.3, 0.4)
}

# Calculate the probability
print(bss.prob_interval(top, probint))

# new style
topevent = bss.getbdd(top)
print(topevent.prob(prob))
print(topevent.prob_interval(probint))

# An example of the direct use of BddNode
bdd = ms.BDD()

# Define events (This version only supports repeated events)
A = bdd.defvar('A')
B = bdd.defvar('B')
C = bdd.defvar('C')

# Make a tree
top = A & B | C # & is AND gate, | is OR gate

print(top.prob(prob))
print(top.prob_interval(probint))

