#!/usr/bin/env python3
print("id, year, month, year_month, scotland, temperature, spawn, eat, left, died, done")

f = open("fish_log.txt", "r+")

x = ""
for i in range(160):
    x += "%d" % ((i // 10)%10)
# print(x)
x = ""
for i in range(160):
    x += "%d" % (i % 10)
# print(x)

for x in f.readlines():
    # print(x)
    __x = x[80:87]
    if __x.strip() == "no":
        __x = "0"
    print("%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s" % (x[1:4], x[21:25], x[26:28], x[21:28], x[29:36], x[54:61], x[65:71], __x, x[99:106], x[131:138], x[152:158]))
    