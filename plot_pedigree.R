suppressPackageStartupMessages(library(kinship2))

args <- commandArgs(trailingOnly = TRUE)
if (length(args) < 2) {
    stop("Uso: Rscript plot_pedigree.R <input.csv> <output.svg> [plot_args_r_expression]")
}

input_csv  <- args[1]
output_svg <- args[2]
# Argumentos de plot passados como string pelo Node (ex: "list(cex=0.8)")
raw_plot_args <- if (length(args) >= 3) args[3] else "list()"

# Lê a tabela do Markdown
df <- read.csv(input_csv, stringsAsFactors = FALSE, strip.white = TRUE)

# --- Camada de Inteligência Clínico-Genética (Tradução e Normalização) ---

# 1. Normalização de Nomes de Coluna (Suporte a PT-BR amigável)
if ("pai" %in% names(df)) df$dadid <- df$pai
if ("mae" %in% names(df)) df$momid <- df$mae
if ("falecido" %in% names(df)) df$status <- df$falecido
if ("probando" %in% names(df)) df$proband <- df$probando

# 2. Progenitores: 0 ou vazio vira NA para não quebrar a raiz
if ("dadid" %in% names(df)) df$dadid[df$dadid == "0" | df$dadid == ""] <- NA
if ("momid" %in% names(df)) df$momid[df$momid == "0" | df$momid == ""] <- NA

# 3. Sexo Canônico (1=M, 2=F, 3=Unknown, 4=Terminated/Diamond)
if ("sex" %in% names(df)) {
    s <- toupper(as.character(df$sex))
    df$sex <- ifelse(s %in% c("M", "MALE", "MAS", "1"), 1,
              ifelse(s %in% c("F", "FEMALE", "FEM", "2"), 2,
              ifelse(s %in% c("T", "TERM", "4"), 4, 3)))
}

# 4. TRATAMENTO COMPLEXO: Zigosidade e Afecção (affected matrix para portadores)
target_aff_col <- if ("fenotipo" %in% names(df)) "fenotipo" else if ("afetado" %in% names(df)) "afetado" else if ("affected" %in% names(df)) "affected" else NULL

if (!is.null(target_aff_col)) {
    aff_vals <- tolower(as.character(df[[target_aff_col]]))
    
    col1 <- rep(0, nrow(df))
    col2 <- rep(0, nrow(df))
    
    for (i in 1:nrow(df)) {
        val <- aff_vals[i]
        # Homo mutado / Afetado Sólido -> (1, 1)
        if (val %in% c("1", "hom", "homozigoto", "afetado", "doente", "mut")) {
            col1[i] <- 1; col2[i] <- 1
        } 
        # Heterozigoto / Portador / Meio-Preenchido Vertical -> (1, 0)
        else if (val %in% c("het", "heterozigoto", "portador", "carrier")) {
            col1[i] <- 1; col2[i] <- 0
        }
        # Desconhecido -> (NA, NA) -> Gera interrogação dentro do símbolo
        else if (val %in% c("?", "na", "unknown", "desconhecido")) {
            col1[i] <- NA; col2[i] <- NA
        }
        # Hígido / Selvagem / WT -> (0, 0)
    }
    df$affected <- cbind(col1, col2)
}

# 5. TRATAMENTO COMPLEXO: Relações de Gêmeos (relation matrix)
if ("gemeo_com" %in% names(df) && any(!is.na(df$gemeo_com) & df$gemeo_com != "" & df$gemeo_com != "0")) {
    rel_rows <- list()
    for (i in 1:nrow(df)) {
        sibling_id <- df$gemeo_com[i]
        if (!is.na(sibling_id) && sibling_id != "" && sibling_id != "0") {
            target_idx <- which(df$id == sibling_id)
            if (length(target_idx) > 0 && i < target_idx) {
                # Padrão: DZ - Dizigótico
                twin_type <- if ("tipo_gemeo" %in% names(df)) toupper(df$tipo_gemeo[i]) else "DZ"
                # Códigos kinship2: 1=MZ (barra unindo), 2=DZ (sem barra)
                rel_code <- ifelse(twin_type == "MZ", 1, 2)
                rel_rows[[length(rel_rows) + 1]] <- c(df$id[i], sibling_id, rel_code)
            }
        }
    }
    if (length(rel_rows) > 0) {
        rel_mat <- do.call(rbind, rel_rows)
        # Converte IDs textuais em índices numéricos da tabela
        df$relation <- matrix(c(
            match(rel_mat[, 1], df$id),
            match(rel_mat[, 2], df$id),
            as.numeric(rel_mat[, 3])
        ), ncol = 3)
    }
}

# --- Instanciação do Pedigree e Plotagem Vetorial ---

# Prepara argumentos para pedigree(...) usando os nomes canônicos
ped_args <- list(
    id = df$id,
    dadid = df$dadid,
    momid = df$momid,
    sex = df$sex
)

# Adiciona as specs complexas calculadas acima
if (!is.null(df$affected)) ped_args$affected <- df$affected
if (!is.null(df$status))   ped_args$status   <- as.numeric(df$status)
if (!is.null(df$relation)) ped_args$relation <- df$relation
if ("famid" %in% names(df)) ped_args$famid   <- df$famid

# Cria o objeto pedigree nativo do R
ped <- do.call(pedigree, ped_args)

# Avalia a string de argumentos visuais (cex, packed, etc) passados pelo Node
plot_user_args <- eval(parse(text = raw_plot_args))

# Se o usuário passou uma coluna 'label' no CSV, usa como rótulo (id do plot)
if ("label" %in% names(df) && is.null(plot_user_args$id)) {
    plot_user_args$id <- df$label
}

# --- Auto-fit de dimensões e proporções compactas didáticas ---
plist <- align.pedigree(ped)
maxlev <- as.integer(nrow(plist$pos))
xrange <- range(plist$pos[plist$nid > 0])
n_cols <- as.numeric(diff(xrange)) + 1

# Altura dinâmica proporcional às gerações (evita esticar linhas desnecessariamente)
calc_h <- max(1.8, min(8.0, maxlev * 0.95 + 0.4))
# Largura proporcional ao número de colunas
calc_w <- max(4.0, min(10.0, n_cols * 1.3 + 1.0))

svg_w <- if (!is.null(plot_user_args$plot_width)) as.numeric(plot_user_args$plot_width) else calc_w
svg_h <- if (!is.null(plot_user_args$plot_height)) as.numeric(plot_user_args$plot_height) else calc_h

plot_user_args$plot_width <- NULL
plot_user_args$plot_height <- NULL

# Padrões ergonômicos e didáticos para símbolos proeminentes e linhas curtas
if (is.null(plot_user_args$symbolsize)) plot_user_args$symbolsize <- 2.2
if (is.null(plot_user_args$cex))        plot_user_args$cex <- 0.85
if (is.null(plot_user_args$branch))     plot_user_args$branch <- 0.4

# Configura o renderizador SVG Cairo com margens mínimas
svg(output_svg, width = svg_w, height = svg_h, bg = "transparent")
par(mar = c(0.3, 0.3, 0.3, 0.3))

# Chamada dinâmica final combinando x=ped com todas as specs visuais declaradas
final_plot_args <- c(list(x = ped), plot_user_args)
do.call(plot, final_plot_args)

dev.off()