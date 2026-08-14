## Handling Potential Failure 
```rs
io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
```

`read_line` 함수는 사용자 입력을 전달받은 문자열에 그대로 넣어주고 `Result` 값도 반환함.
`Result`는 열거형(enum)으로, 여러 가지 가능한 상태(variant) 중 하나를 가질 수 있는 타입임.
`Result`의 목적은 오류 처리 정보를 인코딩하는 것이며 `Ok`와 `Err` 두 가지 유형이 있음.

* `Ok`: 작업이 성공적으로 완료되었음을 의미. 성공적으로 생성된 값을 포함함. 
* `Err`: 작업이 실패했음을 의미.실패 원인에 대한 정보를 포함함.

Result 인스턴스가 Err 값일 때 expect 메서드를 호출하면 프로그램이 종료되고 expect 메서드의 인수로 전달된 오류 메시지가 표시됨. Ok라면 Ok 메소드가 반환하는 값(사용자 입력 바이트 수)을 가져와서 해당 값만 반환.
expect 호출 없이 `read_line`을 사용할 경우 컴파일은 되지만 warning 출력됨.

-> Rust는 프로그램이 발생 가능한 에러에 대한 처리를 하지 않을 경우 경고를 보낸다.

