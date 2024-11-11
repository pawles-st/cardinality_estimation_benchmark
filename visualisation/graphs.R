# datasets config

#cardinalities <- c(1000, 10000, 100000, 1000000)
cardinalities <- c(1000)
data.size.multiplies <- c(1, 10, 100, 1000, 10000)
iters <- 100

# create the cartesian product without the last element

datasets <- expand.grid(card = cardinalities, mult = data.size.multiplies)
datasets <- datasets[-nrow(datasets), ]

# create the plots

for (card in cardinalities) {

	# get HyperLogLog estimates

	estimates.hll <- lapply(data.size.multiplies, function(mult) {
		data.text <- paste(4, format(card, scientific = FALSE), format(card * mult, scientific = FALSE), sep = '_')
		filename.hll <- paste0("../benchmark/target/accuracy/HyperLogLog_", data.text, ".txt")
		estimates.hll <- list(scan(filename.hll, what = double(), nmax = iters))
	})

	# get Gumbel estimates

	estimates.gumbel <- lapply(data.size.multiplies, function(mult) {
		data.text <- paste(4, format(card, scientific = FALSE), format(card * mult, scientific = FALSE), sep = '_')
		filename.gumbel <- paste0("../benchmark/target/accuracy/Gumbel_", data.text, ".txt")
		estimates.gumbel <- list(scan(filename.gumbel, what = double(), nmax = iters))
	})

	# combine the estimates

	estimates <- list()
	for (i in 1:length(data.size.multiplies)) {
		estimates <- c(estimates, estimates.hll[[i]], estimates.gumbel[[i]])
	}

	png("boxplot.png", width = 1920, height = 1080)

	# create a comparison boxplot

	boxplot(estimates,
		main = "Boxplots for cardinality estimators",
		xlab = "Dataset size",
		ylab = "Estimations",
		names = format(card * rep(data.size.multiplies, each = 2), scientific = FALSE),
		col = c("red", "blue"))

	# mark the means

	means <- lapply(estimates, function(l) {
		mean(l)
	})
	points(unlist(means), pch = 3, cex = 1.5)

	# mark the cardinality with a line

	abline(a = card, b = 0, lwd = 2, col = 'green')

	# add a legend

	legend("topright", legend = c("HyperLogLog", "Gumbel"), fill = c("red", "blue"), bty = "n")

	dev.off()
}

#help('points')
