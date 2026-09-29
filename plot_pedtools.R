suppressPackageStartupMessages(library(pedtools))

args <- commandArgs(trailingOnly = TRUE)
if (length(args) < 2) {
    stop("Uso: Rscript plot_pedtools.R <input.csv> <output.svg> [plot_args_r_expression]")
}

input_csv  <- args[1]
output_svg <- args[2]
raw_plot_args <- if (length(args) >= 3) args[3] else "list()"

# Lê a tabela do Markdown
df <- read.csv(input_csv, stringsAsFactors = FALSE, strip.white = TRUE)

# --- Camada de Inteligência Clínico-Genética (Tradução e Normalização) ---

# 1. Normalização de Nomes de Coluna (Suporte a PT-BR e EN)
col_lower <- tolower(names(df))
names(df) <- col_lower

if ("pai" %in% names(df) && !"fid" %in% names(df)) df$fid <- df$pai
if ("mae" %in% names(df) && !"mid" %in% names(df)) df$mid <- df$mae
if ("dadid" %in% names(df) && !"fid" %in% names(df)) df$fid <- df$dadid
if ("momid" %in% names(df) && !"mid" %in% names(df)) df$mid <- df$momid
if ("sexo" %in% names(df) && !"sex" %in% names(df)) df$sex <- df$sexo
if ("falecido" %in% names(df) && !"status" %in% names(df)) df$status <- df$falecido
if ("probando" %in% names(df) && !"proband" %in% names(df)) df$proband <- df$probando

# 2. IDs e Progenitores: 0 ou vazio vira NA ou 0 (pedtools usa 0 para fundadores)
df$id <- as.character(df$id)
if ("fid" %in% names(df)) {
    df$fid <- as.character(df$fid)
    df$fid[is.na(df$fid) | df$fid == "0" | df$fid == ""] <- "0"
} else {
    df$fid <- rep("0", nrow(df))
}
if ("mid" %in% names(df)) {
    df$mid <- as.character(df$mid)
    df$mid[is.na(df$mid) | df$mid == "0" | df$mid == ""] <- "0"
} else {
    df$mid <- rep("0", nrow(df))
}

# 3. Sexo Canônico (1=M, 2=F, 0=Unknown/Diamond)
if ("sex" %in% names(df)) {
    s <- toupper(trimws(as.character(df$sex)))
    df$sex <- ifelse(s %in% c("M", "MALE", "MAS", "1"), 1,
              ifelse(s %in% c("F", "FEMALE", "FEM", "2"), 2, 0))
} else {
    df$sex <- rep(0, nrow(df))
}

# 4. Afecção (aff) e Portador (carrier)
aff_ids <- c()
carrier_ids <- c()

target_aff_col <- if ("fenotipo" %in% names(df)) {
  "fenotipo"
} else if ("afetado" %in% names(df)) {
  "afetado"
} else if ("affected" %in% names(df)) {
  "affected"
} else {
  NULL
}

if (!is.null(target_aff_col)) {
    vals <- tolower(trimws(as.character(df[[target_aff_col]])))
    for (i in seq_len(nrow(df))) {
        v <- vals[i]
        ind_id <- df$id[i]
        if (v %in% c("1", "hom", "homozigoto", "afetado", "doente", "mut", "affected", "yes", "sim", "s")) {
            aff_ids <- c(aff_ids, ind_id)
        } else if (v %in% c("het", "heterozigoto", "portador", "carrier")) {
            carrier_ids <- c(carrier_ids, ind_id)
        }
    }
}

# 5. Falecidos (deceased)
deceased_ids <- c()
if ("status" %in% names(df)) {
    st <- tolower(trimws(as.character(df$status)))
    for (i in seq_len(nrow(df))) {
        if (st[i] %in% c("1", "falecido", "dead", "deceased", "sim", "s", "true")) {
            deceased_ids <- c(deceased_ids, df$id[i])
        }
    }
}

# 6. Probando (proband)
proband_ids <- c()
if ("proband" %in% names(df)) {
    pb <- tolower(trimws(as.character(df$proband)))
    for (i in seq_len(nrow(df))) {
        if (pb[i] %in% c("1", "sim", "true", "s", "proband", "probando", "p")) {
            proband_ids <- c(proband_ids, df$id[i])
        }
    }
}

# 7. Gêmeos (twins)
twins_spec <- NULL
twin_col <- if ("gemeo_com" %in% names(df)) "gemeo_com" else if ("twin" %in% names(df)) "twin" else NULL
if (!is.null(twin_col)) {
    t_rows <- list()
    for (i in seq_len(nrow(df))) {
        sib <- trimws(as.character(df[[twin_col]][i]))
        if (!is.na(sib) && sib != "" && sib != "0") {
            sib_idx <- which(df$id == sib)
            if (length(sib_idx) > 0 && i < sib_idx[1]) {
                twin_type <- if ("tipo_gemeo" %in% names(df)) toupper(trimws(df$tipo_gemeo[i])) else "DZ"
                # pedtools twins: code 1 = MZ, 2 = DZ
                t_code <- if (twin_type == "MZ") 1 else 2
                t_rows[[length(t_rows) + 1]] <- c(df$id[i], sib, t_code)
            }
        }
    }
    if (length(t_rows) > 0) {
        twins_spec <- do.call(rbind, t_rows)
    }
}

# 8. Cria o objeto pedigree nativo do pedtools
ped_df <- data.frame(
    id = df$id,
    fid = df$fid,
    mid = df$mid,
    sex = df$sex,
    stringsAsFactors = FALSE
)

ped <- as.ped(ped_df, validate = TRUE)

# 9. Avalia argumentos visuais declarativos
plot_user_args <- eval(parse(text = raw_plot_args))

# Rótulos (labs) customizados (ex: label, nome, idade)
if ("label" %in% names(df) && is.null(plot_user_args$labs)) {
    plot_user_args$labs <- df$label
}

# Anotações clínicas nos argumentos do plot
if (length(aff_ids) > 0 && is.null(plot_user_args$aff)) {
    plot_user_args$aff <- aff_ids
}
if (length(carrier_ids) > 0 && is.null(plot_user_args$carrier)) {
    plot_user_args$carrier <- carrier_ids
}
if (length(deceased_ids) > 0 && is.null(plot_user_args$deceased)) {
    plot_user_args$deceased <- deceased_ids
}
if (length(proband_ids) > 0 && is.null(plot_user_args$proband)) {
    plot_user_args$proband <- proband_ids
}
if (!is.null(twins_spec) && is.null(plot_user_args$twins)) {
    plot_user_args$twins <- twins_spec
}

# Auto-dimensionamento proporcional
n_ind <- pedsize(ped)
n_gen <- length(generations(ped))

calc_w <- max(4.0, min(12.0, n_ind * 0.9 + 1.5))
calc_h <- max(2.5, min(8.0, n_gen * 1.3 + 0.6))

svg_w <- if (!is.null(plot_user_args$plot_width)) as.numeric(plot_user_args$plot_width) else calc_w
svg_h <- if (!is.null(plot_user_args$plot_height)) as.numeric(plot_user_args$plot_height) else calc_h

plot_user_args$plot_width <- NULL
plot_user_args$plot_height <- NULL

# Padrões visuais
if (is.null(plot_user_args$cex)) plot_user_args$cex <- 0.9
if (is.null(plot_user_args$symbolsize)) plot_user_args$symbolsize <- 1.1

# Renderiza em SVG transparente
svg(output_svg, width = svg_w, height = svg_h, bg = "transparent")
par(mar = c(0.4, 0.4, 0.4, 0.4))

final_plot_args <- c(list(x = ped), plot_user_args)
do.call(plot, final_plot_args)

dev.off()
