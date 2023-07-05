#[microservice_pattern::service]
trait MyService {
    fn a(x: &str) -> u32;
    fn b(x: u32) -> Option<u32>;
}

struct MockImpl;
#[microservice_pattern::service_impl]
impl MyService for MockImpl {
    async fn a(&self, x: &str) -> u32 {
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
        let cli: MyServiceClient = MyServiceServer::new(MockImpl);
        assert_eq!(cli.a("hello").await, 0);
        assert_eq!(cli.b(1).await, None);

        // mockall works
        let mut mock = MockMyService::new();
        mock.expect_a().return_const(1u32);
        assert_eq!(mock.a("hello").await, 1);
        mock.expect_b().return_const(Some(1));
        assert_eq!(mock.b(1).await, Some(1));
    }
}

fn main() {}
