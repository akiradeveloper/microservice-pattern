#[micro_service_pattern::service]
trait MyService {
    fn a(x: &str) -> u32;
    async fn b(x: u32) -> Option<u32>;
}

struct Impl;
#[micro_service_pattern::service_impl]
impl MyService for Impl {
    fn a(&self, x: &str) -> u32 {
        0
    }
    async fn b(&self, x: u32) -> Option<u32> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test() {
        let cli = MyServiceServer::new(Impl);
        assert_eq!(cli.a("hello"), 0);
        assert_eq!(cli.b(1).await, None);

        let mut mock = MockMyService::new();
        mock.expect_a().return_const(1u32);
        assert_eq!(mock.a("hello"), 1);
        mock.expect_b().return_const(None);
        assert_eq!(mock.b(1).await, None);
    }
}

fn main() {}
