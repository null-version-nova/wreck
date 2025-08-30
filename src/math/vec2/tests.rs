use super::*;

static V1: Vector2<i32> = Vector2 { x: 10, y: 20 };
static V1_1: Vector2<i32> = Vector2 { x: 10, y: 20 };
static V2: Vector2<i32> = Vector2 { x: 5, y: 4 };
static V3: Vector2<i32> = Vector2 { x: 15, y: 4 };

#[test]
fn add() {
    let result = V1 + V2;
    assert_eq!(result.x, 15);
    assert_eq!(result.y, 24);
}

#[test]
fn sub() {
    let result = V1 - V2;
    assert_eq!(result.x, 5);
    assert_eq!(result.y, 16);
}

#[test]
fn copy() {
    let copy = V1;
    assert_eq!(&V1, &copy);
}

#[test]
fn ordering() {
    assert!(V2 < V3);
    assert_eq!(V1.partial_cmp(&V3),None);
    assert!(V1 > V2);
}

#[test]
fn equality() {
    assert_eq!(V1,V1_1);
    assert_eq!(V1,V1);
    assert_ne!(V1,V2);
}
