import random
import pandas as pd
from fifo import fifoPageFaults
from lfu import LFUpageFaults
from lru import LRUpageFaults
from optimal import optimalPage
from secondChance import secondChance

def generateReferenceSequence(amount, variety):
    arr = []
    for i in range(0, amount):
        arr.append(random.randint(0, variety))
    return arr

if __name__ == '__main__':
    data = [[], [], [], [], []]
    memorySize = 20
    for i in range(10000):
        seq = generateReferenceSequence(1000, 100)
        data[0].append(optimalPage(seq, len(seq), memorySize))
        data[1].append(fifoPageFaults(seq, len(seq), memorySize))
        data[2].append(LFUpageFaults(len(seq), memorySize, seq))
        data[3].append(LRUpageFaults(seq, len(seq), memorySize))
        data[4].append(secondChance(' '.join(map(str, seq)), memorySize))
    df = pd.DataFrame(data).T
    df.to_excel(excel_writer="./result.xlsx")

