use serde_json::{Result, Value};

fn main() -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2334567"
            ]
        }
    "#;

    let v: Value = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}