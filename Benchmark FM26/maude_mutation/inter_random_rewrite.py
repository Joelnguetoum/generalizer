import maude
import sys
import random
import os

cwd = os.getcwd()
path = str(cwd) + '/maude_mutation/interactions.maude'
maude.init()

maude.load(path)

m = maude.getCurrentModule()

t = m.parseTerm(sys.argv[1])
nb_rewrites = int(sys.argv[2])

rules = ['R1','R2','R3','R4','R5','R6']

for n in range(1,nb_rewrites+1):
     t1 = t
     while t == t1:
     	r = random.choice(rules)
     	strat = m.parseStrategy(r)
     	solutions = list(t1.srewrite(strat,False))
     	if len(solutions) != 0:
     		t1 = (random.choice(solutions))[0]

     t = t1

print(t)