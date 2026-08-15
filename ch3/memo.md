## Rust가 불변성(immutable)을 권장하는 이유 
만약 코드의 어떤 기능이 어떤 값이 절대 변하지 않는다는 가정하에 동작는데, 다른 부분에서 그 값을 변경한다면 첫 번째 코드가 의도한 대로 작동하지 않을 가능성이 있음. 이러한 유형의 버그는 사후에 추적하기 어려울 수 있으며, 특히 값을 변경하는 코드가 값을 때때로만 변경하는 경우에는 더욱 어려움.
Rust 컴파일러는 값이 변경되지 않는다고 명시하면 실제로 값이 변경되지 않도록 보장하므로, 직접 값을 추적할 필요가 없음. 따라서 코드를 더 쉽게 이해할 수 있음.

```rs
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // error!
    println!("The value of x is: {x}");
}
```

## 그럼에도 불구하고 가변성(mutable)을 지원하는 이유
가변성은 매우 유용하며 코드를 더 편리하게 작성할 수 있도록 해줌.
따라서 Rust의 변수는 기본적으로 불변이지만, 변수 이름 앞에 `mut`를 붙이면 가변적으로 만들 수 있도록 함.
`mut`를 붙이는 것은 또한 코드의 다른 부분에서 해당 변수의 값을 변경할 것임을 코드를 읽는 사람에게 알려주는 역할.

```rs
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## Rust 상수
### 불변 변수와 상수의 차이

1. `mut` 사용 불가: 기본값이 불변인게 아니라 상수는 항상 불변임.
2. `let` 키워드 대신 `const` 키워드로 선언함: 선언할 때는 타입을 항상 명시해줘야함.
3. 상수는 모든 scope에서 선언 가능함.
4. 상수는 상수 표현식으로만 설정할 수 있음: 런타임에만 계산할 수 있는 값의 결과로는 설정할 수 없음.

### 특징
Rust 상수 선언에는 대문자와 `_`를 사용함.
상수는 프로그램이 실행되는 동안, 선언된 범위 내에서 유효함.
이러한 특성 덕분에 상수는 프로그램의 여러 부분에서 알아야 할 값, 예를 들어 게임 플레이어의 최대 획득 점수나 빛의 속도와 같은 값을 저장하는 데 유용함.

## Data Type 
중요: Rust는 정적 타입 언어이므로 컴파일 시점에 모든 변수의 타입을 알아야 함.

### Scalar Type
스칼라 타입은 단일 값을 나타냄.
주요 scalar type: integer, floating-point number, boolean, character

#### integer

|         Length         | Signed  | Unsigned |
|------------------------|---------|----------|
| 8-bit                  | `i8`    | `u8`     |
| 16-bit                 | `i16`   | `u16`    |
| 32-bit                 | `i32`   | `u32`    |
| 64-bit                 | `i64`   | `u64`    |
| 128-bit                | `i128`  | `u128`   |
| Architecture-dependent | `isize` | `usize`  |

표현 가능한 정수 형태: Decimal, Hex(`0x`), Octal(`0o`), Binary(`0b`), Byte(`b'A'`, `u8` only)

debug mode로 코드를 컴파일할 경우, Rust는 정수 오버플로우에 대한 검사를 수행함. 오버플로우 검출 시 프로그램은 런타임에 panic에 빠지고 에러 메시지와 함께 종료됨.

release mode로 컴파일할 때는 정수 오버플로우 검사를 수행하지 않음.
대신 저장할 수 있는 최대값보다 큰 값은 해당 유형이 저장할 수 있는 최소값으로 wrapping 함.
예를 들어 `u8`의 경우, 256은 0이 되고, 257은 1이 됨.
프로그램은 패닉에 빠지지 않지만, 변수에는 예상했던 값과는 다른 값이 저장될 수 있음. 
정수 오버플로의 이러한 wrapping을 이용하는 것은 오류로 간주됨.

#### floating-point
Rust는 소수점이 있는 부동 소수점 숫자를 위한 두 가지 기본 데이터 유형(`f32`, `f64`)을 제공함.
기본값: `f64` (최신 CPU에서 속도는 동일한데 정밀도는 더 높기 때문)

```rs
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

#### boolean
1byte 크기의 true/false를 나타내는 타입.

```rs
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

#### character
기본적인 알파벳 타입. 
4바이트 크기를 가지고 유니코드 스칼라 값을 나타내므로 ASCII 문자뿐만 아니라 훨씬 더 많은 문자(한글, 일본어, 중국어, 이모지 등)를 표현할 수 있음.

Unicode Scalar Range : U+0000..U+D7FE, U+E000..U+10FFFF.

### Compound Type 
컴파운드 타입은 여러 값을 하나의 타입으로 묶을 수 있음.
기본 Compound Type: tuple, array

#### tuple 
다양한 유형의 값을 하나의 복합 유형으로 묶는 일반적인 방법.
길이 고정 ->  선언 후 크기 변경 불가.

`선언 예시`
```rs
fn main() {
	// tuple_name: (type1, type2, type3) = (value1, value2, value3);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
역으로 튜플에서 요소를 분해하는 것도 가능함.

```rs
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

요소 직접 접근에는 점(`.`)과 인덱스를 사용함.

```rs
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // 500

    let six_point_four = x.1; // 6.4

    let one = x.2; // 1
}
```

값이 없는 튜플은 unit이라고 부름. 
unit과 그에 해당하는 타입은 모두 ()로 표기되며, 빈 값 또는 빈 반환 타입을 나타냄.
표현식이 다른 값을 반환하지 않으면 암묵적으로 unit 값을 반환함.

#### array
같은 요소를 모아놓은 고정 길이 배열.

`선언 예시`
```rs
fn main() {
    let a = [1, 2, 3, 4, 5];

	// array_name: [type; size] = [..];
	let b: [i32; 5] = [1, 2, 3, 4, 5];

}
```
데이터를 힙이 아닌 스택에 저장하고싶을 때 유용함.

## Function
선언에는 `fn` 키워드를 사용하며 함수와 변수 이름에는 snake case를 사용함.
main 함수가 러스트 프로그램의 엔트리포인트임.

Rust는 함수의 정의 위치는 신경 쓰지 않고 호출자가 접근할 수 있는 범위 내에 정의되어 있는지 여부만 중요하게 생각함.
따라서 main 함수 아래에 정의된 함수도 호출 가능함.

#### parameters

```rs
fn main() {
    another_function(5);
	print_labeled_measurement(5, 'h');
}

// name: type
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

#### Statements and Expressions 
함수 본문은 선택적으로 expression으로 끝나는 일련의 statement들로 구성됨.

* Statement: 어떤 동작을 수행하지만 값을 반환하지 않는 명령어
	
	* `e.g.` 변수 선언 및 할당, 함수 정의
* Expression: 결과값(resultant)으로 평가(evaluate)함
	
	* `e.g.` 수 연산, 함수/매크로 호출, 변수에 할당되는 값, 새로운 스코프를 만드는 중괄호

Rust에서 할당 연산자는 Statement이므로(반환 값이 없으므로), C와 다르게 연속 할당이 불가능함.

```rs
fn main() {
	// x = y = 6; <- C에서는 가능함.
	// C의 `=`는 할당받는 값을 반환하기 때문.
    let x = (let y = 6);
}
```

#### Return Values
함수는 호출하는 코드에 값을 반환할 수 있음. `->` 뒤에 타입을 명시해야 함.
Rust에서 함수의 반환 값은 함수 본문 블록의 마지막 expression 값과 동일함.

`e.g.`
```rs
fn five() -> i32 {
    5 // 마지막 expression, 주의: 세미콜론 없음
}

fn main() {
    let x = five(); // five()는 5를 반환한다.

    println!("The value of x is: {x}");
}
```

마지막 expression에 세미콜론이 추가되면 오류가 발생함.
세미콜론 추가 -> expression이 statement가됨.
`e.g.`
```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1; // statement는 값을 반환하지 않는다 -> 리턴값 타입 불일치
}
```

## Control Flow
### if-else
Rust의 if는 expression임 -> right value로 사용 가능

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

### loop
stop이 명시될 때 까지 무한히 반복하는 코드 블록.

```rs
fn main() {
    loop {
        println!("again!");
    }
}
```
반복문의 용도 중 하나는 실패할 가능성이 있는 작업을 재시도하는 것임. 
또한, 반복문에서 나온 결과를 코드의 나머지 부분으로 전달해야 할 경우 break 문 뒤에 반환할 값을 추가하면 됨.

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // counter == 20
        }
    };

    println!("The result is {result}");
}
```
#### loop labels
반복문 안에 또 다른 반복문이 있는 경우, 가장 안쪽 반복문에서 break 및 continue를 사용하면 가장 안쪽 반복문에 적용됨.
Rust는 중첩된 루프에 loop label을 지정할 수 있으며, 이 레이블을 break 또는 continue 키워드와 함께 사용하여 가장 안쪽 루프가 아닌 레이블이 지정된 루프에 적용하도록 지정할 수 있음.
루프 레이블은 작은따옴표(')로 시작해야 함.

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```
```bash
~/loop_ex$ cargo run
   Compiling loop_ex v0.1.0 (/home/kmwook/Rust-Exercise/ch3/loop_ex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/loop_ex`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```
#### loop with while
loop에 조건 검사를 추가한 것.

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

#### for
for문을 사용하면 인덱스 오버 없이 순회 가능

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
for문을 특정 횟수만큼 반복할 경우.
```rs
fn main() {
    for number in (1..4).rev() { // .rev() -> 역순으로
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
