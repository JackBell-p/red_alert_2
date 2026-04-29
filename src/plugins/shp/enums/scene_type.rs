pub struct SceneInfo {
    pub cn_desc: &'static str,
    pub pal_prefix: &'static str,
    pub scene_mark: &'static str,
}

pub enum SceneType {
    Urban,
    Snow,
    Tem,
    Anim,
}

impl SceneType {
    pub fn info(&self) -> SceneInfo {
        match self {
            SceneType::Urban => SceneInfo { cn_desc: "城市", pal_prefix: "uniturb", scene_mark: "u" },
            SceneType::Snow  => SceneInfo { cn_desc: "雪地", pal_prefix: "unitsno", scene_mark: "a" },
            SceneType::Tem   => SceneInfo { cn_desc: "野外", pal_prefix: "unittem", scene_mark: "t" },
            SceneType::Anim  => SceneInfo { cn_desc: "动画", pal_prefix: "anim",   scene_mark: ""  },
        }
    }
}
