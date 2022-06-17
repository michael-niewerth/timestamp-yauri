extern crate core;

use web_sys::window;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use gloo_console as console;
use wasm_bindgen_futures::spawn_local;

//mod MainCont;
//use crate::MainCont::MainCont;
mod tracks;
use tracks::*;


#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeBtn, catch)]
    pub async fn btn(id:u8,activeId:u8) -> Result<JsValue, JsValue>;
}
fn sendActionToBackend(new_id:u8,activeId:u8) {
    spawn_local(async move {
            match btn(new_id,activeId).await{
                Ok(message) => console::log!("passt{}",message),
                Err(e) => {
                    let window = window().unwrap();
                    window
                        .alert_with_message(&format!("Error: {:?}", e))
                        .unwrap();
                }
            }
        }
    )
}

//Root Component
pub enum Msg{
    ChildClicked { child_id: u8 }
}
pub struct Model {
    Tracks: Vec<Track>,
    activeId: u8
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self
    {
        Self { Tracks: testvec(), activeId: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::ChildClicked {child_id} => {
                sendActionToBackend(child_id,self.activeId);
                if self.activeId == child_id{
                    self.activeId= 0;
                }else{
                self.activeId = child_id;
                }
             }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let vec = testvec();
        let vector = vec.into_iter().map(|children| html!{
            <button class={"button"} name={children.name.to_string()} onclick={ctx.link().callback(move |_| Msg::ChildClicked { child_id: children.id.clone() })}>{children.name.to_string()}</button>
        }).collect::<Html>();

        html!{
            <>
                <h2>{"Timestamp"}</h2>
                <p> {"ID of Active Track: "}{self.activeId}</p>
                {vector}
            </>
        }
     }
}

fn main(){

    yew::start_app::<Model>();
}