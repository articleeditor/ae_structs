# Shopify Structs

This Rust module contains two structs that can be useful for working with data related to products and images.

## Structs

### Image

The `Image` struct represents an image and has the following field:

- `url: String`: The URL of the image.

### Product

The `Product` struct represents a product and has the following fields:

- `id: String`: The unique ID of the product.
- `title: String`: The title of the product.
- `handle: String`: The handle of the product.
- `createdAt: String`: The date and time when the product was created.
- `description: String`: The description of the product.
- `descriptionHtml: String`: The description of the product in HTML format.
- `featuredImage: Option<Image>`: An optional field that contains the featured image of the product, if it exists.

## Example

```rust
use rust_structs::{Image, Product};

fn main() {
    let image = Image {
        url: "https://example.com/image.jpg".to_string(),
    };

    let product = Product {
        id: "123456".to_string(),
        title: "Product Title".to_string(),
        handle: "product-handle".to_string(),
        createdAt: "2022-01-01T00:00:00Z".to_string(),
        description: "Product description".to_string(),
        descriptionHtml: "<p>Product description</p>".to_string(),
        featuredImage: Some(image),
    };
}
