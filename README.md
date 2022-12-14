# 카이사르 암호

![시저암호](/img/Caesar_circle.png)

- **카이사르 암호**(Caesar cipher) 또는 시저 암호는 암호학에서 다루는 간단한 치환암호의 일종이다.
- **율리우스 시저**(Julius Caesar)에 의해 만들어졌다.
- 군사작전에 쓰였던 시저 암호는 정말로 간단하다.
- 예:3글자씩 밀어내는 카이사르 암호로 'COME TO ROME'을 암호화하면 'FRPH WR URPH'가 된다.
- **카이사르 암호**는 단순하고 간단하여 일반인도 쉽게 사용할수 있지만 철자의 빈도와 자주 사용되는 단어와 형태를 이용하면 쉽게 풀수 있다는 단점이 있다.

| 평문   | a   | b   | c   | d   | e   | f   | g   | h   | i   | j   | k   | l   | m   | n   | o   | p   | q   | r   | s   | t   | u   | v   | w   | x   | y   | z   |
| ------ | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 암호문 | D   | E   | F   | G   | H   | I   | J   | K   | L   | M   | N   | O   | P   | Q   | R   | S   | T   | U   | V   | W   | X   | Y   | Z   | A   | B   | C   |

![카이사르2](https://user-images.githubusercontent.com/88940298/205244684-5d6acbd3-556e-49fc-b92f-b667b1093bd2.svg)

## 키 값 만큼 이동

```rs
 let shift = numbers[&c] + shifts as usize;
```

- 3일경우

```rs
 let shift = numbers[&c] + 3 as usize;
```

## 알파벳 개수보다 키 값이 커질떄

```rs
    *letters[&(shift % lang_arr.len())]
```

- key값에 나머지를 저장함으로 키 값이 27이면 결국 1만큼이동한것과 같기 때문에 26으로 나머지를 구해준다.

## 전체코드

- [전체코드](https://github.com/kyunghyunHan/Caesar_Cipher/blob/main/README.md)
