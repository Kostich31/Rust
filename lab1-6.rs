//Лаба №1
fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        // рекурсия.
        return fib(n - 1) + fib(n - 2);
    }
}

#[test]
fn test_fib() {
    assert_eq!(fib(20), 6765);
}

// Лаба №2
fn collatz_length(mut n: i32) -> u32 {
    let mut i: u32 = 1;
    while n > 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        i += 1;
    }
    return i;
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

// Лаба №3
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut out_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            out_matrix[j][i] = matrix[i][j];
        }
    }
    return out_matrix;
}

#[test]
fn test_transpose() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [[101, 201, 301], [102, 202, 302], [103, 203, 303],]
    );
}

// Лаба №4
fn magnitude(vector: &[f64]) -> f64 {
    let sum_of_squares: f64 = vector.iter().map(|&x| x * x).sum();
    sum_of_squares.sqrt()
}

fn normalize(vector: &mut [f64]) {
    let mag = magnitude(vector);
    for component in vector.iter_mut() {
        *component /= mag;
    }
}

#[test]
fn test_magnitude() {
    assert_eq!(magnitude(&[0.0, 1.0, 0.0]), 1.0);
    assert_eq!(magnitude(&[1.0, 0.0, 0.0]), 1.0);
    assert_eq!(magnitude(&[0.0, 0.0, 1.0]), 1.0);
    assert_eq!(magnitude(&[3.0, 4.0, 0.0]), 5.0);
}

#[test]
fn test_normalize() {
    let mut v = [3.0, 4.0, 0.0];
    normalize(&mut v);
    assert_eq!(magnitude(&v), 1.0);
    let mut v = [1.0, 1.0, 1.0];
    normalize(&mut v);
    assert_eq!(magnitude(&v), 1.0);
    let mut v = [2.0, 0.0, 0.0];
    normalize(&mut v);
    assert_eq!(v, [1.0, 0.0, 0.0]);
}

// Лаба №5
#[derive(Debug)]
/// Событие лифта, на которое должен реагировать контроллер.
enum Event {
    CarArrived(i32),                        // Кабина приехала на заданный этаж
    CarDoorOpened,                          // Двери кабины открыты
    CarDoorClosed,                          // Двери кабины закрыты
    LobbyCallButtonPressed(i32, Direction), // Кнопка вызова лифта нажата на этаже
    CarFloorButtonPressed(i32),             // Кнопка этажа нажата в кабине лифта
}

/// Направление движения лифта.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// Кабина приехала на заданный этаж.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// Двери кабины открыты.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// Двери кабины закрыты.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// Кнопка вызова лифта нажата на заданном этаже.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::LobbyCallButtonPressed(floor, dir)
}

/// Кнопка этажа нажата в кабине лифта.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarFloorButtonPressed(floor)
}

#[test]
fn test_car_arrived() {
    let event = car_arrived(5);
    match event {
        Event::CarArrived(floor) => assert_eq!(floor, 5),
        _ => panic!("Expected CarArrived event"),
    }
}

#[test]
fn test_car_door_opened() {
    let event = car_door_opened();
    match event {
        Event::CarDoorOpened => (),
        _ => panic!("Expected CarDoorOpened event"),
    }
}

#[test]
fn test_car_door_closed() {
    let event = car_door_closed();
    match event {
        Event::CarDoorClosed => (),
        _ => panic!("Expected CarDoorClosed event"),
    }
}

#[test]
fn test_lobby_call_button_pressed() {
    let event = lobby_call_button_pressed(3, Direction::Up);
    match event {
        Event::LobbyCallButtonPressed(floor, dir) => {
            assert_eq!(floor, 3);
            assert!(matches!(dir, Direction::Up));
        }
        _ => panic!("Expected LobbyCallButtonPressed event"),
    }
}

#[test]
fn test_car_floor_button_pressed() {
    let event = car_floor_button_pressed(7);
    match event {
        Event::CarFloorButtonPressed(floor) => assert_eq!(floor, 7),
        _ => panic!("Expected CarFloorButtonPressed event"),
    }
}

#[test]
fn test_event_debug_format() {
    let event = car_arrived(2);
    let event_str = format!("{:?}", event);
    assert_eq!(event_str, "CarArrived(2)");
    let event = car_door_opened();
    let event_str = format!("{:?}", event);
    assert_eq!(event_str, "CarDoorOpened");
    let event = lobby_call_button_pressed(1, Direction::Down);
    let event_str = format!("{:?}", event);
    assert_eq!(event_str, "LobbyCallButtonPressed(1, Down)");
}

// Лаба №6

/// Операция над двумя выражениями.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// Выражение в форме узла дерева.
#[derive(Debug)]
enum Expression {
    /// Операция над двумя дочерними выражениями.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Значение
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    match e {
        Expression::Value(val) => val,
        Expression::Op { op, left, right } => {
            let left_val = eval(*left);
            let right_val = eval(*right);

            match op {
                Operation::Add => left_val + right_val,
                Operation::Sub => left_val - right_val,
                Operation::Mul => left_val * right_val,
                Operation::Div => left_val / right_val,
            }
        }
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}
