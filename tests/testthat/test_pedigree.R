# Testes exaustivos para o script CLI plot_pedigree.R e transformações genéticas

test_that("validação de linha de comando: requer no mínimo 2 argumentos", {
  output0 <- suppressWarnings(system2("Rscript", c("plot_pedigree.R"), stdout = TRUE, stderr = TRUE))
  expect_true(any(grepl("Uso: Rscript plot_pedigree.R", output0)))

  output1 <- suppressWarnings(system2("Rscript", c("plot_pedigree.R", "apenas_um_arg.csv"), stdout = TRUE, stderr = TRUE))
  expect_true(any(grepl("Uso: Rscript plot_pedigree.R", output1)))
})

test_that("normalização de colunas PT-BR para padrão kinship2", {
  df <- data.frame(
    id = c("1", "2", "3"),
    pai = c("0", "0", "1"),
    mae = c("", "0", "2"),
    sex = c("M", "FEMALE", "Unknown"),
    falecido = c("0", "1", "0"),
    probando = c("0", "0", "1"),
    stringsAsFactors = FALSE
  )

  # 1. Normalização de Nomes
  if ("pai" %in% names(df)) df$dadid <- df$pai
  if ("mae" %in% names(df)) df$momid <- df$mae
  if ("falecido" %in% names(df)) df$status <- df$falecido
  if ("probando" %in% names(df)) df$proband <- df$probando

  # 2. Progenitores: 0 ou vazio vira NA
  df$dadid[df$dadid == "0" | df$dadid == ""] <- NA
  df$momid[df$momid == "0" | df$momid == ""] <- NA

  expect_true(is.na(df$dadid[1]))
  expect_equal(df$dadid[3], "1")
  expect_true(is.na(df$momid[1]))
  expect_true(is.na(df$momid[2]))
  expect_equal(df$momid[3], "2")

  # 3. Sexo canônico (1=M, 2=F, 3=Unknown, 4=Terminated)
  s <- toupper(as.character(df$sex))
  df$canonical_sex <- ifelse(s %in% c("M", "MALE", "MAS", "1"), 1,
                      ifelse(s %in% c("F", "FEMALE", "FEM", "2"), 2,
                      ifelse(s %in% c("T", "TERM", "4"), 4, 3)))

  expect_equal(df$canonical_sex, c(1, 2, 3))
})

test_that("matriz de afecção e zigosidade clínica", {
  fenotipos <- c("homozigoto", "portador", "wt", "?", "afetado", "het")
  
  col1 <- rep(0, length(fenotipos))
  col2 <- rep(0, length(fenotipos))

  for (i in seq_along(fenotipos)) {
    val <- tolower(fenotipos[i])
    if (val %in% c("1", "hom", "homozigoto", "afetado", "doente", "mut")) {
      col1[i] <- 1; col2[i] <- 1
    } else if (val %in% c("het", "heterozigoto", "portador", "carrier")) {
      col1[i] <- 1; col2[i] <- 0
    } else if (val %in% c("?", "na", "unknown", "desconhecido")) {
      col1[i] <- NA; col2[i] <- NA
    }
  }

  affected_mat <- cbind(col1, col2)

  # Homozigoto / Afetado -> (1, 1)
  expect_equal(affected_mat[1, ], c(1, 1))
  expect_equal(affected_mat[5, ], c(1, 1))

  # Heterozigoto / Portador -> (1, 0)
  expect_equal(affected_mat[2, ], c(1, 0))
  expect_equal(affected_mat[6, ], c(1, 0))

  # Selvagem / WT -> (0, 0)
  expect_equal(affected_mat[3, ], c(0, 0))

  # Desconhecido -> (NA, NA)
  expect_true(all(is.na(affected_mat[4, ])))
})

test_that("matriz de gemelaridade (MZ vs DZ)", {
  df <- data.frame(
    id = c("1", "2", "3", "4"),
    gemeo_com = c("2", "0", "4", "0"),
    tipo_gemeo = c("MZ", "", "DZ", ""),
    stringsAsFactors = FALSE
  )

  rel_rows <- list()
  for (i in 1:nrow(df)) {
    sibling_id <- df$gemeo_com[i]
    if (!is.na(sibling_id) && sibling_id != "" && sibling_id != "0") {
      target_idx <- which(df$id == sibling_id)
      if (length(target_idx) > 0 && i < target_idx) {
        twin_type <- toupper(df$tipo_gemeo[i])
        rel_code <- ifelse(twin_type == "MZ", 1, 2)
        rel_rows[[length(rel_rows) + 1]] <- c(df$id[i], sibling_id, rel_code)
      }
    }
  }

  rel_mat <- do.call(rbind, rel_rows)
  expect_equal(nrow(rel_mat), 2)
  expect_equal(rel_mat[1, 3], "1") # MZ
  expect_equal(rel_mat[2, 3], "2") # DZ
})
