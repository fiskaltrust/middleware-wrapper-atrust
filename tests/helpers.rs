#[macro_export]
macro_rules! mock_call {
  ($mock:ident, $url:expr, $function:tt) => {
    |when, then| {
      when.method(GET).path($url);
      then
          .status(200)
          .header("content-type", "application/json; charset=UTF-8")
          .body(serde_json::ser::to_string(&$mock.$function().unwrap()).unwrap());
    }
  };

  ($mock:ident, $url:expr, $function:tt, $with:expr) => {
    |when, then| {
      when.method(POST).path($url);
      then
          .status(200)
          .header("content-type", "application/json; charset=UTF-8")
          .body(serde_json::ser::to_string(&$mock.$function($with).unwrap()).unwrap());
    }
  };
}
