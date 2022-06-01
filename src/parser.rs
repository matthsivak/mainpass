const DEBUG: bool = false;

pub enum Data {
  Section(String, Vec<Data>),
  Item(String, String),
}

/*
toml
key = data

[key]
key = data
key = data
key = data


rust
  [
    ["key", "data"]
    ["key",
      [
        ["key", "data"],
        ["key", "data"],
        ["key", "data"],
      ]
    ]
  ]

*/
