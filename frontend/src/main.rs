use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew::virtual_dom::AttrValue;
use gloo_console as console;

//mod MainCont;
//use crate::MainCont::MainCont;
mod tracks;
use tracks::*;


#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeBtn, catch)]
    pub async fn btn(id::u8) -> Result<JsValue, JsValue>;
}



#[derive(Clone, PartialEq, Properties)]
pub struct TrackProps {
    name: String,
    id:u8,
}
pub enum TrackMsg{
    Clicked,
}
pub struct TrackComponent{
    name: String,
    id:u8,
    started:bool,
}

impl Component for TrackComponent{
    type Message = TrackMsg;
    type Properties = TrackProps;

    fn create(_ctx: &Context<Self>) -> Self {
        console::log!(format!("{}",_ctx.props(1).name));
        Self{ name: _ctx.props().name.to_string(), id: _ctx.props().id, started: false}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.name = "succes".to_string();
        console::log!("passt");
        true
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <p>
                <button onclick={ctx.link().callback(|_| TrackMsg::Clicked)}>
                {format!("Id für Debug: {},{}",self.name.to_string(), self.id.to_string())}
                </button>
            </p>
        )
    }
}


//Root Component
pub enum Msg{
    Start,
    Stop,
    Debug,
}
pub struct Model {
    Tracks: Vec<Track>,
}
/*
#[derive(Properties, PartialEq)]
pub struct ModelProps{
    #[prop_or_default]
    pub children: ChildrenWithProps<TrackComponent>,
}
*/
impl Component for Model{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{ Tracks: testvec()}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
            let vec = testvec().iter().map(|track| html!{
            <TrackComponent name={track.name.to_string()} id={track.id}/>
        }).collect::<Html>();
            html!(
            <div>
                { vec }
            </div>
        )
    }
}

fn main(){
    yew::start_app::<Model>();
}