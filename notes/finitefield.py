import math
m_x = 0x11b

def myf(a,b):
    #multiply numbers together, 
    result = 0
    if b < a:
        temp = b
        b = a 
        a = temp
    
    # # 
    # while (1 << bit_position) < b:
    #     if #there is a one when shifting x bits:
    #         result ^= b << bit_position
    #     bit_position += 1
    bit_position = 0
    while (1 << bit_position) <= b:
        if( (b >> bit_position) & 1):
            result ^= (a << bit_position)
        bit_position += 1
    result2 = result
    #mod operation


    print(result)


    N  = result2
    while(result2 > 255):
        #left shifts is tells the placement of the largest 1 in the bitstring 
        left_shifts = 0
        #line up leftmost digit
        while (result2 >> (left_shifts + 1)) > m_x:
            left_shifts += 1
        #subtraction is xor in a finite field
        result2 ^= (m_x <<  left_shifts)

    print(bin(result2))
    return result2

def xtime(a):
    return (a << 1) ^ 0x1b

a = myf(128,2)

print(a)