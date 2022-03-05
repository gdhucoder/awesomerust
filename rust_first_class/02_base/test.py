
from json.tool import main


def show(s):
    print(s)

show('123')

mylist = show
mylist('456')
mylist = [1,2,3]
mylist.append(4)
print(mylist)
# 若类型语言，例如python，将函数指针解引用成一个列表，是可以插入新元素的