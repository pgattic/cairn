
def fact(x):
    if x == 0:
        return 1
    return x * fact(x-1)

def fact_iter(x):
    result = 1
    for i in range(1, x+1):
        result *= i
    return result

print(fact_iter(10000))

#def fib(x):
#    if x == 0:
#        return 0
#    elif x == 1:
#        return 1
#    else:
#        return fib(x-1) + fib(x-2)
#
#print(fib(6))

