suppressPackageStartupMessages(library(kinship2))

args <- commandArgs(trailingOnly = TRUE)
if (length(args) < 2) {
    stop("Uso: Rscript plot_pedigree.R <input.csv> <output.svg> [plot_args_r_expression]")
}

input_csv  <- args[1]
output_svg <- args[2]
raw_plot_args <- if (length(args) >= 3) args[3] else "list()"

df <- read.csv(input_csv, stringsAsFactors = FALSE, strip.white = TRUE)

# 1. Normalização de Nomes de Coluna
if ("pai" %in% names(df)) df$dadid <- df$pai
if ("mae" %in% names(df)) df$momid <- df$mae
if ("sexo" %in% names(df)) df$sex <- df$sexo
if ("falecido" %in% names(df)) df$status <- df$falecido
if ("probando" %in% names(df)) df$proband <- df$probando

# 2. Progenitores
if ("dadid" %in% names(df)) df$dadid[df$dadid == "0" | df$dadid == ""] <- NA
if ("momid" %in% names(df)) df$momid[df$momid == "0" | df$momid == ""] <- NA

# 3. Sexo Canônico
if ("sex" %in% names(df)) {
    s <- toupper(as.character(df$sex))
    df$sex <- ifelse(s %in% c("M", "MALE", "MAS", "1"), 1,
              ifelse(s %in% c("F", "FEMALE", "FEM", "2"), 2,
              ifelse(s %in% c("T", "TERM", "4"), 4, 3)))
} else {
    df$sex <- rep(3, nrow(df))
}

# 4. Zigosidade, Afecção e Suspeita (?)
target_aff_col <- if ("fenotipo" %in% names(df)) "fenotipo" else if ("afetado" %in% names(df)) "afetado" else if ("affected" %in% names(df)) "affected" else NULL
suspect_ids <- c()

if (!is.null(target_aff_col)) {
    aff_vals <- tolower(trimws(as.character(df[[target_aff_col]])))
    has_carrier <- any(aff_vals %in% c("het", "heterozigoto", "portador", "carrier"))

    for (i in 1:nrow(df)) {
        if (aff_vals[i] %in% c("?", "suspeito", "suspeita", "investigacao", "investigação")) {
            suspect_ids <- c(suspect_ids, as.character(df$id[i]))
        }
    }

    if (has_carrier) {
        col1 <- rep(0, nrow(df))
        col2 <- rep(0, nrow(df))
        for (i in 1:nrow(df)) {
            val <- aff_vals[i]
            if (val %in% c("1", "hom", "homozigoto", "afetado", "doente", "mut", "sim", "s", "a")) {
                col1[i] <- 1; col2[i] <- 1
            } else if (val %in% c("het", "heterozigoto", "portador", "carrier")) {
                col1[i] <- 1; col2[i] <- 0
            } else if (val %in% c("na", "unknown", "desconhecido")) {
                col1[i] <- NA; col2[i] <- NA
            }
        }
        df$affected <- cbind(col1, col2)
    } else {
        aff_vec <- rep(0, nrow(df))
        for (i in 1:nrow(df)) {
            val <- aff_vals[i]
            if (val %in% c("1", "hom", "homozigoto", "afetado", "doente", "mut", "sim", "s", "a")) {
                aff_vec[i] <- 1
            } else if (val %in% c("na", "unknown", "desconhecido")) {
                aff_vec[i] <- NA
            }
        }
        df$affected <- aff_vec
    }
}

# 5. Relações de Gêmeos
if ("gemeo_com" %in% names(df) && any(!is.na(df$gemeo_com) & df$gemeo_com != "" & df$gemeo_com != "0")) {
    rel_rows <- list()
    for (i in 1:nrow(df)) {
        sibling_id <- df$gemeo_com[i]
        if (!is.na(sibling_id) && sibling_id != "" && sibling_id != "0") {
            target_idx <- which(df$id == sibling_id)
            if (length(target_idx) > 0 && i < target_idx) {
                twin_type <- if ("tipo_gemeo" %in% names(df)) toupper(df$tipo_gemeo[i]) else "DZ"
                rel_code <- ifelse(twin_type == "MZ", 1, 2)
                rel_rows[[length(rel_rows) + 1]] <- c(df$id[i], sibling_id, rel_code)
            }
        }
    }
    if (length(rel_rows) > 0) {
        rel_mat <- do.call(rbind, rel_rows)
        df$relation <- matrix(c(
            match(rel_mat[, 1], df$id),
            match(rel_mat[, 2], df$id),
            as.numeric(rel_mat[, 3])
        ), ncol = 3)
    }
}

# Instanciação do Pedigree
ped_args <- list(
    id = df$id,
    dadid = df$dadid,
    momid = df$momid,
    sex = df$sex
)
if (!is.null(df$affected)) ped_args$affected <- df$affected
if (!is.null(df$status))   ped_args$status   <- as.numeric(df$status)
if (!is.null(df$relation)) ped_args$relation <- df$relation
if ("famid" %in% names(df)) ped_args$famid   <- df$famid

ped <- do.call(pedigree, ped_args)

plot_user_args <- eval(parse(text = raw_plot_args))

# Extração de parâmetros da legenda customizada
user_legend_labels <- plot_user_args$legend
user_legend_types  <- plot_user_args$legend.type
plot_user_args$legend      <- NULL
plot_user_args$legend.type <- NULL

if ("label" %in% names(df) && is.null(plot_user_args$id)) {
    plot_user_args$id <- df$label
}

# Auto-fit de dimensões
plist <- align.pedigree(ped)
maxlev <- as.integer(nrow(plist$pos))
xrange <- range(plist$pos[plist$nid > 0])
n_cols <- as.numeric(diff(xrange)) + 1

# Proporções ajustadas ao tamanho de página A4:
# Menor largura em polegadas evita redução extrema pelo Typst (que diminuía símbolos e texto)
calc_h <- max(6.5, min(10.0, maxlev * 1.8 + 3.2))
calc_w <- max(6.5, min(8.5, n_cols * 1.05 + 0.8))

svg_w <- if (!is.null(plot_user_args$plot_width)) as.numeric(plot_user_args$plot_width) else calc_w
svg_h <- if (!is.null(plot_user_args$plot_height)) as.numeric(plot_user_args$plot_height) else calc_h

plot_user_args$plot_width  <- NULL
plot_user_args$plot_height <- NULL

# Padrões visuais destacados: símbolos grandes e texto bem nítido
if (is.null(plot_user_args$symbolsize)) plot_user_args$symbolsize <- 3.2
if (is.null(plot_user_args$cex))        plot_user_args$cex <- 0.92
if (is.null(plot_user_args$branch))     plot_user_args$branch <- 0.35
if (is.null(plot_user_args$density))    plot_user_args$density <- c(-1, -1)
if (is.null(plot_user_args$angle))      plot_user_args$angle <- c(0, 0)

svg(output_svg, width = svg_w, height = svg_h, bg = "transparent", family = "Liberation Sans")
# Margem inferior generosa para acomodar o card de legenda sem sobrepor nenhum indivíduo
par(mar = c(12.5, 0.6, 0.4, 0.6), xpd = TRUE)

# Patch kinship2:::plot.pedigree para evitar linha divisória em homozigotos vazados / não afetados
tryCatch({
    fn_lines <- deparse(kinship2:::plot.pedigree)
    idx_db <- grep("drawbox <- function", fn_lines)
    if (length(idx_db) > 0) {
        replacement_db <- c(
          "    polylist_full <- list(square = list(list(x = c(-1, -1, 1, 1)/2, y = c(0, 1, 1, 0))), circle = list(list(x = 0.5 * cos(seq(0, 2 * pi, length = 50)), y = 0.5 * sin(seq(0, 2 * pi, length = 50)) + 0.5)), diamond = list(list(x = c(0, -0.5, 0, 0.5), y = c(0, 0.5, 1, 0.5))), triangle = list(list(x = c(0, -0.56, 0.56), y = c(0, 1, 1))))",
          "    drawbox <- function(x, y, sex, affected, status, col, polylist, density, angle, boxw, boxh) {",
          "        if (all(affected == 0)) {",
          "            polygon(x + polylist_full[[sex]][[1]]$x * boxw, y + polylist_full[[sex]][[1]]$y * boxh, col = \"white\", border = col)",
          "        } else if (all(affected == 1) && length(affected) > 1 && all(density == -1)) {",
          "            polygon(x + polylist_full[[sex]][[1]]$x * boxw, y + polylist_full[[sex]][[1]]$y * boxh, col = col, border = col)",
          "        } else {"
        )
        idx_status <- grep("if (status == 1)", fn_lines, fixed=TRUE)
        new_lines <- c(
          fn_lines[1:(idx_db-1)],
          replacement_db,
          fn_lines[(idx_db+2):(idx_status-1)],
          "        }",
          fn_lines[idx_status:length(fn_lines)]
        )
        patched_fn <- eval(parse(text = paste(new_lines, collapse="\n")), envir = environment(kinship2:::plot.pedigree))
        utils::assignInNamespace("plot.pedigree", patched_fn, ns = "kinship2")
    }
}, error = function(e) {})

final_plot_args <- c(list(x = ped), plot_user_args)
p_out <- do.call(plot, final_plot_args)

boxh <- p_out$boxh
boxw <- p_out$boxw

# Desenho de interrogação (?) para suspeitos
if (length(suspect_ids) > 0) {
    for (s_id in suspect_ids) {
        target_idx <- which(ped$id == s_id)
        if (length(target_idx) > 0) {
            for (r in 1:nrow(plist$nid)) {
                for (c in 1:ncol(plist$nid)) {
                    if (plist$nid[r, c] == target_idx) {
                        px <- plist$pos[r, c]
                        py <- r + boxh / 2
                        text(px, py, "?", font = 2, cex = 1.35, col = "black")
                    }
                }
            }
        }
    }
}

# Desenho da seta de Probanda (NSGC)
proband_ids <- c()
if ("proband" %in% names(df)) {
    pb <- tolower(trimws(as.character(df$proband)))
    for (i in seq_len(nrow(df))) {
        if (pb[i] %in% c("1", "sim", "true", "s", "proband", "probanda", "p")) {
            proband_ids <- c(proband_ids, as.character(df$id[i]))
        }
    }
}

if (length(proband_ids) > 0) {
    for (p_id in proband_ids) {
        target_idx <- which(ped$id == p_id)
        if (length(target_idx) > 0) {
            for (r in 1:nrow(plist$nid)) {
                for (c in 1:ncol(plist$nid)) {
                    if (plist$nid[r, c] == target_idx) {
                        px <- plist$pos[r, c]
                        py <- r
                        arrows(x0 = px - 0.86 * boxw, y0 = py + 1.30 * boxh,
                               x1 = px - 0.54 * boxw, y1 = py + 1.02 * boxh,
                               length = 0.08, lwd = 2.0, col = "black")
                        text(px - 0.94 * boxw, py + 1.38 * boxh, "P", cex = 0.90, font = 2)
                    }
                }
            }
        }
    }
}

# --- Card Elegante de Legenda Integrada ao Gráfico ---
if (!is.null(user_legend_labels) && length(user_legend_labels) > 0) {
    par(xpd = NA)
    n_items <- length(user_legend_labels)
    types   <- if (!is.null(user_legend_types)) user_legend_types else rep("sq_empty", n_items)
    
    usr <- par("usr")
    pin <- par("pin")
    scale_x <- (usr[2] - usr[1]) / pin[1]
    scale_y <- abs(usr[3] - usr[4]) / pin[2]
    
    # Posicionamento estritamente na margem inferior, abaixo dos limites do gráfico
    y_plot_bottom   <- usr[3]
    y_device_bottom <- grconvertY(0.012, from = "ndc", to = "user")
    
    y_top    <- y_plot_bottom + 0.16
    y_bottom <- y_device_bottom - 0.03
    x_left   <- usr[1] + 0.08
    x_right  <- usr[2] - 0.08
    
    # Fundo e Borda do Card de Legenda
    rect(x_left, y_top, x_right, y_bottom, col = "#fcfcfc", border = "#bbbbbb", lwd = 1.0)
    
    # Cabeçalho da Legenda
    text((x_left + x_right)/2, y_top + 0.11, "LEGENDA", cex = 0.85, font = 2, col = "#333333")
    
    # Linha divisória sutil
    segments(x_left + 0.3, y_top + 0.20, x_right - 0.3, y_top + 0.20, col = "#d8d8d8", lwd = 0.8)
    
    # Símbolos com dimensões físicas uniformes em polegadas (evita distorção de proporção)
    sym_size_in <- 0.28
    dx <- 0.5 * sym_size_in * scale_x
    dy <- 0.5 * sym_size_in * scale_y
    theta <- seq(0, 2 * pi, length = 60)
    
    y_sym  <- y_top + 0.40
    y_text <- y_top + 0.63
    
    xs <- seq(from = x_left + 0.8,
              to   = x_right - 0.8,
              length.out = n_items)
              
    for (i in seq_len(n_items)) {
        xi   <- xs[i]
        yi   <- y_sym
        t_i  <- types[i]
        lb_i <- user_legend_labels[i]
        
        if (t_i %in% c("sq_empty", "empty_m")) {
            rect(xi - dx, yi - dy, xi + dx, yi + dy, border = "black", col = "white", lwd = 1.3)
        } else if (t_i %in% c("circ_empty", "empty_f")) {
            polygon(xi + dx * cos(theta), yi + dy * sin(theta), border = "black", col = "white", lwd = 1.3)
        } else if (t_i %in% c("half", "sq_half")) {
            rect(xi - dx, yi - dy, xi, yi + dy, border = "black", col = "black", lwd = 1.3)
            rect(xi, yi - dy, xi + dx, yi + dy, border = "black", col = "white", lwd = 1.3)
        } else if (t_i == "circ_half") {
            theta_l <- seq(pi/2, 3*pi/2, length = 30)
            theta_r <- seq(-pi/2, pi/2, length = 30)
            polygon(c(xi, xi + dx * cos(theta_l), xi), c(yi + dy, yi + dy * sin(theta_l), yi - dy), col = "black", border = "black", lwd = 1.3)
            polygon(c(xi, xi + dx * cos(theta_r), xi), c(yi + dy, yi + dy * sin(theta_r), yi - dy), col = "white", border = "black", lwd = 1.3)
            segments(xi, yi - dy, xi, yi + dy, col = "black", lwd = 1.3)
        } else if (t_i %in% c("solid", "sq_solid")) {
            rect(xi - dx, yi - dy, xi + dx, yi + dy, border = "black", col = "black", lwd = 1.3)
        } else if (t_i == "circ_solid") {
            polygon(xi + dx * cos(theta), yi + dy * sin(theta), border = "black", col = "black", lwd = 1.3)
        } else if (t_i %in% c("suspect", "circ_suspect")) {
            polygon(xi + dx * cos(theta), yi + dy * sin(theta), border = "black", col = "white", lwd = 1.3)
            text(xi, yi, "?", font = 2, cex = 1.25, col = "black")
        } else if (t_i == "sq_suspect") {
            rect(xi - dx, yi - dy, xi + dx, yi + dy, border = "black", col = "white", lwd = 1.3)
            text(xi, yi, "?", font = 2, cex = 1.25, col = "black")
        } else if (t_i == "proband") {
            rect(xi - dx, yi - dy, xi + dx, yi + dy, border = "black", col = "white", lwd = 1.3)
            arrows(x0 = xi - dx * 1.5, y0 = yi + dy * 1.35, x1 = xi - dx * 0.95, y1 = yi + dy * 0.75, length = 0.05, lwd = 1.8, col = "black")
            text(xi - dx * 1.85, yi + dy * 1.5, "P", font = 2, cex = 0.85)
        }
        cex_lbl <- if (n_items >= 6) 0.66 else 0.76; text(xi, y_text, lb_i, cex = cex_lbl, adj = c(0.5, 0.5))
    }
}

dev.off()
