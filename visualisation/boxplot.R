source("common.R")

# datasets config

card <- 100000
mult <- 100
precisions <- c(4, 8, 12, 16)
iters <- 100

# algorithms to analyse

algorithms <- c("HyperLogLog", "GumbelGeo", "GumbelLazyGeo", "GumbelHar", "GumbelLazyHar")
no.algorithms <- length(algorithms)

# create the plots

for (prec in precisions) {

	estimates <- list()

	# read all estimates

	for (i in 1:no.algorithms) {
		alg.estimates <- read.data(algorithms[i], prec, card, mult)
		estimates <- c(estimates, list(alg.estimates))
	}

	# create a comparison boxplot

	png(paste0("boxplot_", prec, ".png"), width = 1920, height = 1080)

	boxplot(estimates,
		main = paste("Boxplots for cardinality estimators (cardinality = ", card, ")"),
		xlab = "Algorithm",
		ylab = "Estimations",
		names = algorithms,
		col = rainbow(no.algorithms)
	)

	# mark the means

	means <- lapply(estimates, mean)
	points(unlist(means), pch = 3, cex = 2, lwd = 2)

	# mark the cardinality with a line

	abline(h = card, lwd = 2, col = 'green')

	# add a legend

	legend("topright", legend = algorithms, fill = rainbow(no.algorithms), bty = "n", cex = 1.5)

	dev.off()
}
