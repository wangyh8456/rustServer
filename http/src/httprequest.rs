use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method{
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version{
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource{
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resource,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest{
    fn from(req:String)->Self{
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_resource = Resource::Path(String::from(""));
        let mut parsed_version=Version::V1_1;
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines(){
            if line.contains("HTTP"){
                let (method,resource,version)=process_req_line(line);
                parsed_method=method;
                parsed_resource=resource;
                parsed_version=version;
            }else if line.contains(":"){
                let (key,value)=process_header_line(line);
                parsed_headers.insert(key,value);
            }else if line.len()==0{
            }else{
                parsed_msg_body=line;
            }
        }

        HttpRequest{
            method:parsed_method,
            resource:parsed_resource,
            version:parsed_version,
            headers:parsed_headers,
            msg_body:parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s:&str)->(Method,Resource,Version){
    let mut words=s.split_whitespace();
    let method=words.next().unwrap();
    let resource=words.next().unwrap();
    let version=words.next().unwrap();

    (method.into(),Resource::Path(resource.to_string()),version.into())
}

fn process_header_line(s:&str)->(String,String){
    let mut words=s.split(":");
    let mut key=String::from("");
    let mut value=String::from("");
    if let Some(k)=words.next(){
        key=k.to_string();
    }
    if let Some(v)=words.next(){
        value=v.to_string();
    }
    (key,value)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_method_into(){
        let m:Method = "GET".into();
        assert_eq!(Method::GET,m);
    }
    
    #[test]
    fn test_version_into(){
        let v:Version = "HTTP/1.1".into();
        assert_eq!(Version::V1_1,v);
    }

    #[test]
    fn test_httprequest(){
        let s=String::from("GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:curl/7.71.0\r\nAccept:*/*\r\n\r\n");
        let mut header_expected=HashMap::new();
        header_expected.insert("Host".into(),"localhost".into());
        header_expected.insert("User-Agent".into(),"curl/7.71.0".into());
        header_expected.insert("Accept".into(),"*/*".into());
        let req:HttpRequest=s.into();
        println!("{:?}",req);
        assert_eq!(Method::GET,req.method);
        assert_eq!(Resource::Path("/greeting".to_string()),req.resource);
        assert_eq!(Version::V1_1,req.version);
        assert_eq!(header_expected,req.headers);
    }
}