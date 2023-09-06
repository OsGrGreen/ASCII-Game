import sys


print("hello world")
x = 12
print(bin(x))

print(sys.getsizeof(bin(x)))
print(sys.getsizeof(x))
print(sys.getsizeof(str(x)))


binary_d = x.to_bytes((x.bit_length() + 7)//8, 'big')

text_d = str(x).encode('utf-8')

bin_size = sys.getsizeof(binary_d)
text_size = sys.getsizeof(text_d)

print(binary_d)
print(text_d)

print(bin_size)
print(text_size)
