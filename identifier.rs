//^
//^ HEAD
//^

//> HEAD -> CORE
use core::any::TypeId;


//^
//^ IDENTIFIER
//^

//> IDENTIFIER -> WRAPPER
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Identifier(pub TypeId);

//> IDENTIFIER -> DEFAULT
const impl Default for Identifier {
    fn default() -> Self {
        struct Random;
        return Identifier(TypeId::of::<Random>());
    }
}