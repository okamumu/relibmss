import relibmss as ms

# Create a binary system
bss = ms.BSS()

# Define events (This version only supports repeated events)
A = bss.defvar('A')
B = bss.defvar('B')
C = bss.defvar('C')

# Make a system
top = bss.kofn(2, [A, B, C]) # k-of-n gate

# Obtain the minimal path vectors
s = bss.mpvs(top) # s is a set of minimal path vectors (ZDD representation)

# Convert the ZDD representation to a list of sets
min_path = s.extract()
print('The number of minimal path vectors:', len(min_path))
for x in min_path:
    print(x)
