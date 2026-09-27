# Testes unitários para o script CLI plot_pedigree.R

test_that("plot_pedigree.R valida quantidade mínima de argumentos", {
  output <- suppressWarnings(system2("Rscript", c("plot_pedigree.R"), stdout = TRUE, stderr = TRUE))
  expect_true(any(grepl("Uso: Rscript plot_pedigree.R", output)))
})

test_that("plot_pedigree.R exige argumentos de entrada e saída", {
  output <- suppressWarnings(system2("Rscript", c("plot_pedigree.R", "in.csv"), stdout = TRUE, stderr = TRUE))
  expect_true(any(grepl("Uso: Rscript plot_pedigree.R", output)))
})
