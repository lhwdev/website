use std::ops::{Deref, DerefMut};


struct Privilege;

#[derive(PartialEq)]
pub struct Privileges(Vec<Privilege>);
impl Deref for Privileges { type Target = Vec < Privilege > ; fn deref (& self) -> & Self :: Target { & self . 0 } } impl DerefMut for Privileges { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl serde :: Serialize for Privileges { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer { self . 0 . serialize (serializer) } } impl < 'de > serde :: Deserialize < 'de > for Privileges { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > { Ok (Privileges (Vec < Privilege > :: deserialize (deserializer) ?)) } }
