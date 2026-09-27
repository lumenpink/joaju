# Entry point para suíte de testes em R
if (requireNamespace("testthat", quietly = TRUE)) {
  library(testthat)
  testthat::test_dir("tests/testthat")
}
