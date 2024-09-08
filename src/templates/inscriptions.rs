use super::*;

#[derive(Boilerplate)]
pub(crate) struct InscriptionsHtml {
  pub(crate) inscriptions: Vec<InscriptionId>,
  pub(crate) prev: Option<i64>,
  pub(crate) next: Option<i64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InscriptionsJson {
  pub inscriptions: Vec<InscriptionId>,
  pub prev: Option<i64>,
  pub next: Option<i64>,
  pub lowest: Option<i64>,
  pub highest: Option<i64>,
}

impl InscriptionsJson {
  pub fn new(
    inscriptions: Vec<InscriptionId>,
    prev: Option<i64>,
    next: Option<i64>,
    lowest: Option<i64>,
    highest: Option<i64>,
  ) -> Self {
    Self {
      inscriptions,
      prev,
      next,
      lowest,
      highest,
    }
  }
}

impl PageContent for InscriptionsHtml {
  fn title(&self) -> String {
    "Shibescription".into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn without_prev_and_next() {
    assert_regex_match!(
      InscriptionsHtml {
        inscriptions: vec![inscription_id(1), inscription_id(2)],
        prev: None,
        next: None,
      },
      "
        <h1>Shibescription</h1>
        <div class=thumbnails>
          <a href=/shibescription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          <a href=/shibescription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
        </div>
        .*
        prev
        next
        .*
      "
      .unindent()
    );
  }

  #[test]
  fn with_prev_and_next() {
    assert_regex_match!(
      InscriptionsHtml {
        inscriptions: vec![inscription_id(1), inscription_id(2)],
        prev: Some(1),
        next: Some(2),
      },
      "
        <h1>Shibescription</h1>
        <div class=thumbnails>
          <a href=/shibescription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          <a href=/shibescription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
        </div>
        .*
        <a class=prev href=/shibescriptions/1>prev</a>
        <a class=next href=/shibescriptions/2>next</a>
        .*
      "
      .unindent()
    );
  }
}
