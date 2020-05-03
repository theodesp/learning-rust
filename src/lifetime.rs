struct Config {
    debug_mode: bool,
}

#[derive(Debug)]
struct Product;

struct ProductService<'a> {
    config: &'a Config,
}
struct BasketService<'a> {
    config: &'a Config,
}

impl<'a> ProductService<'a> {
    fn new(config: &Config) -> ProductService {
        ProductService { config: config }
    }

    fn get_product(&self, id: i32) -> Product {
        if self.config.debug_mode {
            println!("retrieving product for id: {:?}", id);
        }

        Product
    }
}

impl<'a> BasketService<'a> {
    fn new(config: &Config) -> BasketService {
        BasketService { config: config }
    }

    fn add_product(&self, item: Product) {
        if self.config.debug_mode {
            println!("adding product {:?}", item);
        }
    }
}

fn main() {
    let config = Config { debug_mode: true };
    let product_service = ProductService::new(&config);
    let basket_service = BasketService::new(&config);

    let product = product_service.get_product(1);
    basket_service.add_product(product);
}
