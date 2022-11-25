# Caesar_Cipher

![시저암호](/img/Caesar_circle.png)

- **카이사르 암호**(Caesar cipher) 또는 시저 암호는 암호학에서 다루는 간단한 치환암호의 일종이다.
- 율리우스 시저(Julius Caesar)에 의해 만들어졌다.
- 군사작전에 쓰였던 시저 암호는 정말로 간단하다.
- 예:3글자씩 밀어내는 카이사르 암호로 'COME TO ROME'을 암호화하면 'FRPH WR URPH'가 된다.

## 알파벳 개수보다 키 값이 커질떄

```rs
    *letters[&(shift % lang_arr.len())]
```

- key값에 나머지를 저장함으로 키 값이 27이면 결국 1만큼이동한것과 같기 때문에 26으로 나머지를 구해준다.

## 키 값을 더했을 떄 z보다 커질떄
