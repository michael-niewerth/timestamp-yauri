#[derive(Debug, PartialEq)]
pub struct Track{
    pub name: String,
    pub id:u8,
}

pub fn testvec() ->Vec<Track>{
    vec![
        Track{
            name:"Track 1".to_string(),
            id:1,
        },
        Track{
            name:"Track 2".to_string(),
            id:5,
        }
    ]
}

