enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// O enum acima equivale a todos os structs abaixo
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// porém, seria muito mais difícil escrever uma
// função que recebesse tudo isso ao invés de receber
// apenas o enum

// um enum pode ter metodos implementados exatamente como uma struct
impl Message {
    fn call(&self) {
    //     method body
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}