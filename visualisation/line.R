source("common.R")

# datasets config

cardinalities <- seq(from = 10000, to = 1000000, by = 10000)
mult <- 100
precisions <- c(4, 8, 12, 16)
iters <- 100

# algorithms to analyse

algorithms <- c("HLL", "GHLLGeo", "GHLLHar", "GHLLPlus")
no.algorithms <- length(algorithms)

# create the plots

for (prec in precisions) {

	means <- c()

	for (card in cardinalities) {

		# read all estimates

		for (i in 1:no.algorithms) {
			alg.estimates <- read.data(algorithms[i], prec, card, mult)
			means <- c(means, mean(alg.estimates) / card)
		}
	}

	# create a comparison lineplot

	png(paste0("line_", prec, ".png"), width = 1920, height = 1080)

	plot(x = cardinalities,
		y = rep(1, length(cardinalities)),
		main = "Line plot for cardinality estimators",
		xlab = "Cardinality",
		ylab = "Estimation / Cardinality",
		ylim = c(0.9, 1.1),
		type = "n",
	)

	# mark the means

	cols <- rainbow(no.algorithms)
	for (i in 1:no.algorithms)
		lines(x = cardinalities,
			y = means[seq(i, length(means), by = no.algorithms)],
			main = "Line plot for cardinality estimators",
			xlab = "Cardinality",
			ylab = "Estimation / Cardinality",
			col = cols[i],
			lwd = 1.5,
	)

	# mark the ideal result with a line

	abline(h = 1, lwd = 2, col = 'green')

	# add a legend

	legend("topright", legend = algorithms, fill = rainbow(no.algorithms), bty = "n", cex = 1.5)

	dev.off()
}
