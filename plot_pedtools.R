suppressPackageStartupMessages(library(pedtools))

args <- commandArgs(trailingOnly = TRUE)
if (length(args) < 2) {
    stop("Uso: Rscript plot_pedtools.R <input.csv> <output.svg> [plot_args_r_expression]")
}

input_csv     <- args[1]
output_svg    <- args[2]
raw_plot_args <- if (length(args) >= 3) args[3] else "list()"

df <- read.csv(input_csv, stringsAsFactors = FALSE, strip.white = TRUE)

col_lower <- tolower(names(df))
names(df) <- col_lower

if ("pai" %in% names(df) && !"fid" %in% names(df)) df$fid <- df$pai
if ("mae" %in% names(df) && !"mid" %in% names(df)) df$mid <- df$mae
if ("dadid" %in% names(df) && !"fid" %in% names(df)) df$fid <- df$dadid
if ("momid" %in% names(df) && !"mid" %in% names(df)) df$mid <- df$momid
if ("sexo" %in% names(df) && !"sex" %in% names(df)) df$sex <- df$sexo
if ("falecido" %in% names(df) && !"status" %in% names(df)) df$status <- df$falecido
if ("probando" %in% names(df) && !"proband" %in% names(df)) df$proband <- df$probando

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

if ("sex" %in% names(df)) {
    s <- toupper(trimws(as.character(df$sex)))
    df$sex <- ifelse(s %in% c("M", "MALE", "MAS", "1"), 1,
              ifelse(s %in% c("F", "FEMALE", "FEM", "2"), 2, 0))
} else {
    df$sex <- rep(0, nrow(df))
}

aff_ids     <- c()
carrier_ids <- c()
suspect_ids <- c()

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
        } else if (v %in% c("?", "suspeita", "suspect", "duvida")) {
            suspect_ids <- c(suspect_ids, ind_id)
        }
    }
}

deceased_ids <- c()
if ("status" %in% names(df)) {
    st <- tolower(trimws(as.character(df$status)))
    for (i in seq_len(nrow(df))) {
        if (st[i] %in% c("1", "falecido", "dead", "deceased", "sim", "s", "true")) {
            deceased_ids <- c(deceased_ids, df$id[i])
        }
    }
}

proband_ids <- c()
if ("proband" %in% names(df)) {
    pb <- tolower(trimws(as.character(df$proband)))
    for (i in seq_len(nrow(df))) {
        if (pb[i] %in% c("1", "sim", "true", "s", "proband", "probanda", "probando", "p")) {
            proband_ids <- c(proband_ids, df$id[i])
        }
    }
}

ped_df <- data.frame(
    id = df$id,
    fid = df$fid,
    mid = df$mid,
    sex = df$sex,
    stringsAsFactors = FALSE
)
ped <- as.ped(ped_df, validate = FALSE)

plot_user_args <- eval(parse(text = raw_plot_args))

user_legend_labels <- plot_user_args$legend
user_legend_types  <- plot_user_args$legend.type
plot_user_args$legend      <- NULL
plot_user_args$legend.type <- NULL

# Quebra de linha inteligente nos rótulos:
# Ex: "II-1 [Azul bb]" -> "II-1\n[Azul bb]"
if ("label" %in% names(df) && is.null(plot_user_args$labs)) {
    formatted_labels <- sapply(df$label, function(lb) {
        if (grepl("\\[", lb)) {
            sub("\\s*\\[", "\n[", lb)
        } else {
            lb
        }
    }, USE.NAMES = FALSE)
    plot_user_args$labs <- setNames(df$id, formatted_labels)
}

if (length(aff_ids) > 0 && is.null(plot_user_args$aff)) {
    plot_user_args$aff <- aff_ids
}
plot_user_args$proband <- NULL
if (is.null(plot_user_args$foldLabs)) {
    plot_user_args$foldLabs <- FALSE
}

if (length(deceased_ids) > 0 && is.null(plot_user_args$deceased)) {
    plot_user_args$deceased <- deceased_ids
}

k2ped <- as_kinship2_pedigree(ped)
plist <- kinship2::align.pedigree(k2ped)
maxlev <- as.integer(nrow(plist$pos))
xrange <- range(plist$pos[plist$nid > 0])
n_cols <- as.numeric(diff(xrange)) + 1

calc_w <- max(6.5, min(8.5, n_cols * 1.05 + 0.8))
# Altura proporcional ao número de gerações:
# Compacta a distância vertical entre gerações evitando linhas excessivamente longas
tree_h <- max(2.0, (maxlev - 1) * 1.10 + 0.90)

has_legend <- !is.null(user_legend_labels) && length(user_legend_labels) > 0
mar_bottom <- if (has_legend) 9.0 else 2.2

calc_h <- tree_h + (if (has_legend) 1.95 else 0.60)

svg_w <- if (!is.null(plot_user_args$plot_width)) as.numeric(plot_user_args$plot_width) else calc_w
svg_h <- if (!is.null(plot_user_args$plot_height)) as.numeric(plot_user_args$plot_height) else calc_h
plot_user_args$plot_width  <- NULL
plot_user_args$plot_height <- NULL

if (is.null(plot_user_args$cex)) plot_user_args$cex <- 0.85
if (is.null(plot_user_args$symbolsize)) plot_user_args$symbolsize <- 1.3
plot_user_args$margins  <- c(mar_bottom, 0.8, 0.8, 0.8)
plot_user_args$addSpace <- c(0, 0.40, 0, 0.40)

svg(output_svg, width = svg_w, height = svg_h, bg = "transparent", family = "Liberation Sans")

final_plot_args <- c(list(x = ped, keep.par = TRUE), plot_user_args)
p_out <- do.call(plot, final_plot_args)

boxw <- p_out$scaling$boxw
boxh <- p_out$scaling$boxh
aln  <- p_out$alignment

# 1. Desenho de Heterozigotos Divididos ao Meio (half-fill)
if (length(carrier_ids) > 0) {
    for (c_id in carrier_ids) {
        idx <- which(aln$ped$ID == c_id)
        if (length(idx) > 0) {
            k <- which(aln$plotord == idx)
            if (length(k) > 0) {
                px <- aln$xall[k]
                py <- aln$yall[k]
                sx <- aln$sex[idx]
                if (sx == 1) { # Quadrado
                    rect(px - boxw/2, py, px, py + boxh, col = "black", border = "black")
                    rect(px, py, px + boxw/2, py + boxh, col = "white", border = "black")
                } else if (sx == 2) { # Círculo
                    theta_l <- seq(pi/2, 3*pi/2, length = 30)
                    theta_r <- seq(-pi/2, pi/2, length = 30)
                    dx <- boxw/2; dy <- boxh/2; cy <- py + boxh/2
                    polygon(c(px, px + dx * cos(theta_l), px), c(cy + dy, cy + dy * sin(theta_l), cy - dy), col = "black", border = "black")
                    polygon(c(px, px + dx * cos(theta_r), px), c(cy + dy, cy + dy * sin(theta_r), cy - dy), col = "white", border = "black")
                    segments(px, cy - dy, px, cy + dy, col = "black")
                }
            }
        }
    }
}

# 2. Desenho de Interrogação (?) para Suspeitos
if (length(suspect_ids) > 0) {
    for (s_id in suspect_ids) {
        idx <- which(aln$ped$ID == s_id)
        if (length(idx) > 0) {
            k <- which(aln$plotord == idx)
            if (length(k) > 0) {
                px <- aln$xall[k]
                py <- aln$yall[k] + boxh / 2
                text(px, py, "?", font = 2, cex = 1.35, col = "black")
            }
        }
    }
}

# 3. Desenho Padronizado da Seta de Probanda (NSGC)
# Apontando estritamente para o canto inferior esquerdo do símbolo do indivíduo (px - boxw/2, py + boxh)
if (length(proband_ids) > 0) {
    for (p_id in proband_ids) {
        idx <- which(aln$ped$ID == p_id)
        if (length(idx) > 0) {
            k <- which(aln$plotord == idx)
            if (length(k) > 0) {
                px <- aln$xall[k]
                py <- aln$yall[k]
                # Alvo da seta: apontando para o canto inferior esquerdo com um respiro elegante
                target_x <- px - 0.58 * boxw
                target_y <- py + 1.05 * boxh
                
                # Origem: diagonal externa mais afastada para baixo e esquerda
                orig_x <- target_x - 0.45 * boxw
                orig_y <- target_y + 0.48 * boxh
                
                arrows(x0 = orig_x, y0 = orig_y,
                       x1 = target_x, y1 = target_y,
                       length = 0.08, lwd = 2.0, col = "black")
                # Letra P com respiro afastada da base da seta (lado inferior esquerdo)
                text(orig_x - 0.18 * boxw, orig_y + 0.20 * boxh, "P", cex = 0.90, font = 2)
            }
        }
    }
}

# 4. Card de Legenda Vetorial Integrado
if (has_legend) {
    par(xpd = NA)
    n_items <- length(user_legend_labels)
    types   <- if (!is.null(user_legend_types)) user_legend_types else rep("sq_empty", n_items)
    
    usr <- par("usr")
    pin <- par("pin")
    scale_x <- (usr[2] - usr[1]) / pin[1]
    scale_y <- abs(usr[3] - usr[4]) / pin[2]
    
    # Coordenadas em polegadas calibradas para estabilidade independente do número de gerações
    y_top    <- usr[3] + 0.18 * scale_y
    y_bottom <- y_top  + 1.35 * scale_y
    x_left   <- usr[1] + 0.15 * scale_x
    x_right  <- usr[2] - 0.15 * scale_x
    
    rect(x_left, y_top, x_right, y_bottom, col = "#fcfcfc", border = "#bbbbbb", lwd = 1.0)
    text((x_left + x_right)/2, y_top + 0.18 * scale_y, "LEGENDA", cex = 0.85, font = 2, col = "#333333")
    segments(x_left + 0.25 * scale_x, y_top + 0.30 * scale_y, x_right - 0.25 * scale_x, y_top + 0.30 * scale_y, col = "#d8d8d8", lwd = 0.8)
    
    sym_size_in <- 0.26
    dx <- 0.5 * sym_size_in * scale_x
    dy <- 0.5 * sym_size_in * scale_y
    theta <- seq(0, 2 * pi, length = 60)
    
    y_sym  <- y_top + 0.65 * scale_y
    y_text <- y_top + 1.05 * scale_y
    
    # Distribuição uniforme dos itens ao longo da largura útil do card em polegadas
    xs_in <- seq(from = 0.6, to = pin[1] - 0.6, length.out = n_items)
    xs    <- usr[1] + xs_in * scale_x
              
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
            # Seta apontando para o canto inferior esquerdo com respiro adequado e P espaçado
            arr_target_x <- xi - dx * 1.15
            arr_target_y <- yi + dy * 1.15
            arr_orig_x   <- arr_target_x - dx * 0.70
            arr_orig_y   <- arr_target_y + dy * 0.70
            arrows(x0 = arr_orig_x, y0 = arr_orig_y, x1 = arr_target_x, y1 = arr_target_y, length = 0.06, lwd = 1.8, col = "black")
            text(arr_orig_x - dx * 0.35, arr_orig_y + dy * 0.30, "P", font = 2, cex = 0.85)
        }
        cex_lbl <- if (n_items >= 6) 0.66 else 0.76; text(xi, y_text, lb_i, cex = cex_lbl, adj = c(0.5, 0.5))
    }
}

dev.off()
