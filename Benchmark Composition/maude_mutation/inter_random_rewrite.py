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

strat_str = 'R1 | R2'
strat = m.parseStrategy(strat_str)

solutions = None

for n in range(1,nb_rewrites+1):
    solutions = t.srewrite(strat,False)
    (t,temp) = random.choice(list(solutions))


print(t)