// p145 명령줄에서 사용할 수 있는 도구 제작
// rustc 03_sum_args.rs 로 실행 파일 생성
// ./03_sum_args 100 55 3 으로 실행파일 실행
// 158


fn main() {
    // CLI 파일 실행 이후 입력된 내용을 whitespace 로 split 하여 반복자로 탐색 가능한 <String> type 형태로 입력됨
    let args = std::env::args();  

//    let _vec_arg: Vec<String> = args.collect(); // vec type 으로 변환

    let mut total = 0.0;

    for (i, s) in args.enumerate() {
        if i == 0 { continue; }
        
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };

        total += num;
    }

    println!("{}", total);
}