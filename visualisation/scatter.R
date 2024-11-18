source("common.R")

# datasets config

cardinalities <- seq(from = 10000, to = 600000, by = 10000)
mult <- 100
precisions <- c(4, 8, 12, 16)
iters <- 100

# algorithms to analyse

#algorithms <- c("HyperLogLog", "GumbelGeo", "GumbelLazyGeo", "GumbelLazyHar")
algorithms <- c("GumbelHar")
no.algorithms <- length(algorithms)

# create the plots

for (prec in precisions) {

	estimates <- list()
	means <- c()

	for (card in cardinalities) {

		# read all estimates

		for (i in 1:no.algorithms) {
			alg.estimates <- read.data(algorithms[i], prec, card, mult)
			estimates <- c(estimates, list(alg.estimates / card))
			means <- c(means, mean(alg.estimates) / card)
		}
	}

	# create a comparison boxplot

	png(paste0("scatter_", prec, ".png"), width = 1920, height = 1080)

	#unlist(estimates)
	#plot(x = rep(cardinalities, each = no.algorithms * iters),
		     #y = unlist(estimates),
		#main = "Scatter plot for cardinality estimators",
		#xlab = "Cardinality",
		#ylab = "Estimation / Cardinality",
		#col = rep(rep(rainbow(no.algorithms), each = iters), times = length(cardinalities)),
		#pch = 16,
	#)

	# mark the means

	plot(x = rep(cardinalities, each = no.algorithms),
		y = means,
		main = "Scatter plot for cardinality estimators",
		xlab = "Cardinality",
		ylab = "Estimation / Cardinality",
		col = rep(rainbow(no.algorithms), times = length(cardinalities)),
		pch = 16
	)

	# mark the ideal result with a line

	abline(h = 1, lwd = 2, col = 'green')

	# add a legend

	legend("topright", legend = algorithms, fill = rainbow(no.algorithms), bty = "n", cex = 1.5)

	dev.off()
}
