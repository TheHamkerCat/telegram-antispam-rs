## Telegram Antispam API
#### Blazingly fast spam classification API built using [Rocket](https://github.com/SergioBenitez/Rocket) Web Framework.

![Rust](https://forthebadge.com/images/badges/made-with-rust.svg)

#### Notes
- The classifier works in aggressive mode, it can sometimes classify non-spam messages/emails as spam (when the input is too small)
- The dataset provided may contain some NSFW texts or personal info, it's not thoroughly checked.
- I've included a docker-based example, but you can run it without docker as well.
- `profanity` in response is only there to maintain backwards compatibility for ARQ users, it always returns `false`
## Installation:
I would suggest using docker compose for this, but it's upto you!
### With Docker compose

```sh
$ git clone https://github.com/thehamkercat/telegram-antispam-rs
$ cd telegram-antispam-rs
$ docker-compose build
$ docker-compose up
```

### With Cargo

```sh
$ git clone https://github.com/thehamkercat/telegram-antispam-rs
$ cd telegram-antispam-rs
$ cargo run --release
```

## Endpoints:

```http
POST /spam_check HTTP/1.1
Host: localhost:8000
Content-Type: application/json

{
  "text": "subscribe to my youtube channel"
}

HTTP/1.1 200 OK
content-length: 59
content-type: application/json

{
  "spam": 99,
  "ham": 1,
  "is_spam": true,
  "profanity": false,
  "spam_probability": 99
}
```
### A Go port for the same API can be found [here](https://github.com/axrav/AntispamAPI)

## Usage examples:
### Python

```python
import requests

url = "http://localhost:8000/spam_check"
data = {"text": "subscribe to my youtube channel"}

result = requests.post(url, json=data).json()

print("Is spam:", result["is_spam"])
print("Spam probability:", result["spam_probability"])
```

### Go

```go
package main

import (
	"bytes"
	"encoding/json"
	"net/http"
)

func main() {
	url := "http://localhost:8000/spam_check"
	data := map[string]string{"text": "subscribe to my youtube channel"}
	jsonData, err := json.Marshal(data)
	if err != nil {
		panic(err)
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		panic(err)
	}

	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()

	// Do something with the response if needed
}
```

### Rust

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Resp {
    spam: u8,
    ham: u8,
    is_spam: bool,
    spam_probability: u8,
    profanity: bool
}

#[tokio::main]
async fn main(){
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("text", "Hello please subscribe to my youtube channel!");

    let res = client
        .post("http://localhost:8000/spam_check")
        .json(&map)
        .send()
        .await
        .unwrap();

    let text_response = res.text().await.unwrap();

    let json: Result<Resp, _> = serde_json::from_str(text_response.as_str());
    if json.is_ok() {
        println!("{:?}", json.unwrap());
    }
}

// [dependencies]
// reqwest = { version = "0.11", features = ["json"] }
// serde = {version = "1.0.160", features = ["derive"]}
// serde_json = "1.0.96"
// tokio = { version = "1", features = ["full"] }
```