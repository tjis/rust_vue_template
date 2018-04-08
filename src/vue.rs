use stdweb;
use stdweb::unstable::{TryFrom,TryInto};

pub struct VueModel {
    js: stdweb::Value
}

impl VueModel {
    pub fn new(js: stdweb::Value) -> VueModel {
        VueModel { js: js}
    }

    pub fn get<T>(&self, prop: &str) -> T where T: TryFrom<stdweb::Value>+Clone{
        let result : T = js! {
            return @{&self.js}[@{prop}];
        }.try_into().map_err(|_|()).unwrap();

        // clone the result here, cause it glitches otherwise
        result.clone()
    }

   pub fn set<'a,T:?Sized>(&self, prop: &str, value: &'a T) where &'a T: Into<stdweb::Value> {
        let jsval: stdweb::Value = value.into();
        js! {
            @{&self.js}[@{prop}] = @{jsval};
        }
    }
}

impl TryFrom<stdweb::Value> for VueModel {
    type Error = ();
    fn try_from(value: stdweb::Value) -> Result<VueModel,()> {
        Ok(VueModel { js: value })
    }
}

impl TryInto<stdweb::Value> for VueModel {
    type Error = ();
    fn try_into(self) -> Result<stdweb::Value,()> {
        Ok(self.js)
    }
}
