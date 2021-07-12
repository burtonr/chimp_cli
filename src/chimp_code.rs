pub trait SCM {
  fn clone(&self) -> String;
  fn commit(&self) -> String;
  fn push(&self) -> String;
  fn pull(&self) -> String;
  fn branch(&self) -> String;
}
