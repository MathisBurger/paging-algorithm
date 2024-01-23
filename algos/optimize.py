import pandas as pd
resNr = input("FileNr: ")
df_samples = pd.read_excel("./results/raw/" + resNr + ".xlsx", engine='openpyxl')
entrycount = int(input("Entries: "))
optimized = [[], [], [], [], []]
for i in range(5):
    l = len(df_samples[i].values)
    if l > entrycount:
        l = int(l / entrycount)
        for j in range(entrycount):
            sum = 0
            for x in range(l):
                sum += df_samples[i].values[j*l+x]
            optimized[i].append(sum / l)
    else:
        for j in range(l):
            optimized[i].append(df_samples[i].values[j])
df = pd.DataFrame(optimized).T
df.to_excel(excel_writer="./results/optimized/" + resNr + ".xlsx")