use whistle_ast::IdentType;

pub trait Type {
  fn is_eq(type1: IdentType, type2: IdentType) -> bool;
}

impl Type for IdentType {
  fn is_eq(type1: IdentType, type2: IdentType) -> bool {
    if type1 == type2 {
      if type1 == IdentType::Error {
        return false
      }
      return true
    }
    if let IdentType::Union(list1) = type1.clone() {
      if list1.contains(&type2) {
        return true
      }
    }
    if let IdentType::Union(list2) = type1.clone() {
      if list2.contains(&type1) {
        return true
      }
    }
    false
  }
}
