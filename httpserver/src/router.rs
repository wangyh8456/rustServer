use super::handler::{Handler, StaticPageHandler, PageNotFoundHandler,WebServiceHandler};
use http::{httprequest,httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router{
    pub fn router(req:HttpRequest,stream:&mut impl Write){
        match req.method{
            httprequest::Method::GET=>match &req.resource{
                httprequest::Resource::Path(s)=>{
                    let router:Vec<&str>=s.split("/").collect();
                    //router[0]是host，router[1]是api或者其他
                    match router[1]{
                        "api"=>{
                            let resp:HttpResponse=WebServiceHandler::handle(&req);
                            let _=resp.send_response(stream);
                        }
                        _=>{
                            let resp:HttpResponse=StaticPageHandler::handle(&req);
                            let _=resp.send_response(stream);
                        }
                    }
                }
            }
            _=>{
                let resp:HttpResponse=PageNotFoundHandler::handle(&req);
                let _=resp.send_response(stream);
            }
        }
    }
}