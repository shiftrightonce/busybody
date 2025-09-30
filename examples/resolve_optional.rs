#[tokio::main]
async fn main() {
    let container = busybody::ServiceContainerBuilder::new()
        .register(0) // 1. We are storing a counter that will be used to count the number of visitors
        .await
        .resolver(|container| async move {
            let current = container.get_type::<i32>().await.unwrap_or_default() + 1; // increment counter
            container.set_type(current).await; // resave it

            // 2. For every tenth visitor, apply a 25% discount
            //    An example of how we could return none for a particular resource
            if current % 10 == 0 {
                Some(Discount(0.25))
            } else {
                None
            }
        })
        .await
        .build();

    // 3. Loops mocks visitors hitting the site
    for count in 1..=128 {
        let amount = (count as f32).powf(2.0);
        let mut after_discount = amount;
        // 4. When we get back "some" Discount, we know it is time to apply it
        if let Some(Some(discount)) = container.get_type::<Option<Discount>>().await {
            after_discount = discount.apply(amount)
        }

        let flag = if amount != after_discount { ">> " } else { "" };

        println!(
            "{} original amount: ${:.02}, after 25% discount: ${:.02}",
            flag, amount, after_discount
        );
    }
}

#[derive(Debug, Clone)]
struct Discount(f32);

impl Discount {
    fn apply(&self, base: f32) -> f32 {
        base - (base * self.0)
    }
}
