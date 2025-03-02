

flag = "BKISC{G00d_j0b_H4rdw4res_4re_v3ry_easy!}"

def string_to_bits(s):
    return ' '.join(format(ord(char), '08b') for char in s)

#V_ref = 5V

arr_bits = string_to_bits(flag).split(' ')

print(arr_bits)

V_out = []
for i in arr_bits:
    V_out.append(((5 * int(i,2)) / 256)*(-1))

print(V_out)

# V_out = [-1.2890625, -1.46484375, -1.42578125, -1.62109375, -1.30859375, -2.40234375, -1.38671875, -0.9375, -0.9375, -1.953125, -1.85546875, -2.0703125, -0.9375, -1.9140625, -1.85546875, -1.40625, -1.015625, -2.2265625, -1.953125, -2.32421875, -1.015625, -2.2265625, -1.97265625, -2.24609375, -1.85546875, -1.015625, -2.2265625, -1.97265625, -1.85546875, -2.3046875, -0.99609375, -2.2265625, -2.36328125, -1.85546875, -1.97265625, -1.89453125, -2.24609375, -2.36328125, -0.64453125, -2.44140625]