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

# top = 1-(1-pa*pb)*(1-pc) = pa*pb+pc-pa*pb*pc
# top / pa = pb - pb*pc = 0.2 - 0.2*0.3 = 0.14
# top / pb = pa - pa*pc = 0.1 - 0.1*0.3 = 0.07
# top / pc = 1 - pa*pb = 1 - 0.1*0.2 = 0.98

print(bss.bmeas(top, prob))

# Set the interval of the probability
prob = {
    'A': (0.1, 0.2),
    'B': (0.2, 0.3),
    'C': (0.3, 0.4)
}

# Calculate the probability
print(bss.prob_interval(top, prob))

print(bss.bmeas_interval(top, prob))

###

prob = {
    'A': 0.5,
    'B': 0.5,
    'C': 0.5
}

# structure importance measure
print(bss.bmeas(top, prob))
