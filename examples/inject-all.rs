#[tokio::main]
async fn main() {
    //1. Using `inject_all` you can inject a tuple that has 1 to 17 fields
    let (a,) = busybody::helpers::inject_all::<(TypeA,)>().await;
    println!("a: {:?}", a);

    // 2. tuple with two fields
    let (a, b) = busybody::helpers::inject_all::<(TypeA, TypeB)>().await;
    println!("a: {:?}, b: {:?}", a, b);

    // 3. tuple with three fields
    let (a, c, b) = busybody::helpers::inject_all::<(TypeA, TypeC, TypeB)>().await;
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);

    // 4. a complex type
    let group: GroupOfABC = busybody::helpers::inject_all().await;
    println!("GroupOfABC instance: {:#?}", group);
}

#[derive(Debug)]
struct TypeA;

#[busybody::async_trait]
impl busybody::Injectable for TypeA {
    async fn inject(_: &busybody::ServiceContainer) -> Self {
        Self
    }
}

#[derive(Debug)]
struct TypeB;

#[busybody::async_trait]
impl busybody::Injectable for TypeB {
    async fn inject(_: &busybody::ServiceContainer) -> Self {
        Self
    }
}

#[derive(Debug)]
struct TypeC;

#[busybody::async_trait]
impl busybody::Injectable for TypeC {
    async fn inject(_: &busybody::ServiceContainer) -> Self {
        Self
    }
}

#[derive(Debug)]
#[allow(dead_code)] // We are not using the struct fields
struct GroupOfABC {
    a: TypeA,
    b: TypeB,
    c: TypeC,
}

#[busybody::async_trait]
impl busybody::Injectable for GroupOfABC {
    async fn inject(container: &busybody::ServiceContainer) -> Self {
        Self {
            a: container.provide().await,
            b: container.provide().await,
            c: container.provide().await,
        }
    }
}
