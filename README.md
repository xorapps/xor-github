### XOR-Github
Data structures to parse the Github API


###### Example 
```rs
    use xor_github::CommitInfo;

    let foo = minreq::get("https://api.github.com/repos/xorapps/xor-image-handler/commits/aae04b57467309cfacbe8cbc534f946dc6f69cee")
    .with_header("Authorization", "token <GITHUB_TOKEN_HERE>")
    .with_header("Accept", "application/vnd.github+json")
    .with_header("User-Agent" , "minreq")
    .send()
    .unwrap();

    let foo = serde_json::from_str::<CommitInfo>(foo.as_str().unwrap()).unwrap();

    dbg!(&foo);
```