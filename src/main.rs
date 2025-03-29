#[allow(dead_code)]
fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[allow(dead_code)]
fn is_odd(num: i32) -> bool {
    !is_even(num)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        // 偶数のテスト
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(4));
        assert!(is_even(100));
        assert!(is_even(-2));
        assert!(is_even(-100));

        // 奇数のテスト（偽になることを確認）
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(99));
        assert!(!is_even(-1));
        assert!(!is_even(-99));
    }

    #[test]
    fn test_is_odd() {
        // 奇数のテスト
        assert!(is_odd(1));
        assert!(is_odd(3));
        assert!(is_odd(99));
        assert!(is_odd(-1));
        assert!(is_odd(-99));

        // 偶数のテスト（偽になることを確認）
        assert!(!is_odd(0));
        assert!(!is_odd(2));
        assert!(!is_odd(4));
        assert!(!is_odd(100));
        assert!(!is_odd(-2));
        assert!(!is_odd(-100));
    }

    #[test]
    fn test_even_odd_relationship() {
        // 同じ数値に対して is_even と is_odd は逆の結果になることを確認
        for num in -10..=10 {
            assert_eq!(is_even(num), !is_odd(num));
            assert_eq!(is_odd(num), !is_even(num));
        }
    }
}

fn main() {
    println!("テストを実行するには `cargo test` を使用してください");
}
