
#[derive(Debug)]
pub struct Attribute {
  pub name: String,
  pub value: String
}

#[derive(Debug)]
pub struct Element {
  pub tag_name: String,
  pub attributes: Vec<Attribute>
}


#[derive(Debug)]
pub struct Fragment {
  pub children: Vec<Node>
}

#[derive(Debug)]
pub struct Text {
  pub value: String
}

#[derive(Debug)]
pub enum Node {
  Element(Element),
  Fragment(Fragment),
  Text(Text)
}
