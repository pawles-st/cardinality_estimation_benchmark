# datasets config

#cardinalities <- c(1000, 10000, 100000, 1000000)
cardinalities <- c(100000)
data.size.multiplies <- c(1, 100)
precisions <- c(4, 8, 12, 16)
iters <- 100
no.algorithms <- 2

# create the cartesian product without the last element

datasets <- expand.grid(card = cardinalities, mult = data.size.multiplies)
datasets <- datasets[-nrow(datasets), ]

# create the plots

prec = 12
for (card in cardinalities) {

	# get HyperLogLog estimates

	estimates.hll <- lapply(data.size.multiplies, function(mult) {
		data.text <- paste(prec, format(card, scientific = FALSE), format(card * mult, scientific = FALSE), sep = '_')
		filename <- paste0("../results/HyperLogLog_", data.text, ".txt")
		scan(filename, what = double(), nmax = iters)
	})

	# get Gumbel estimates

	estimates.gumbel <- lapply(data.size.multiplies, function(mult) {
		data.text <- paste(prec, format(card, scientific = FALSE), format(card * mult, scientific = FALSE), sep = '_')
		filename <- paste0("../results/Gumbel_", data.text, ".txt")
		scan(filename, what = double(), nmax = iters)
	})

	# get Gumbel Lazy estimates

	#estimates.gumbel.lazy <- lapply(data.size.multiplies, function(mult) {
		#data.text <- paste(prec, format(card, scientific = FALSE), format(card * mult, scientific = FALSE), sep = '_')
		#filename <- paste0("../results/GumbelLazy_", data.text, ".txt")
		#scan(filename, what = double(), nmax = iters)
	#})

	# combine the estimates

	estimates.combined <- vector("list", length(data.size.multiplies) * no.algorithms)
	estimates.combined[seq(1, length(estimates.combined), by = no.algorithms)] <- estimates.hll
	estimates.combined[seq(2, length(estimates.combined), by = no.algorithms)] <- estimates.gumbel
	#estimates.combined[seq(3, length(estimates.combined), by = no.algorithms)] <- estimates.gumbel.lazy

	png("boxplot.png", width = 1920, height = 1080)

	# create a comparison boxplot

	boxplot(estimates.combined,
		main = "Boxplots for cardinality estimators",
		xlab = "Dataset size",
		ylab = "Estimations",
		names = format(card * rep(data.size.multiplies, each = no.algorithms), scientific = FALSE),
		col = rainbow(no.algorithms))

	# mark the means

	means <- lapply(estimates, function(l) {
		mean(l)
	})
	points(unlist(means), pch = 3, cex = 1.5)

	# mark the cardinality with a line

	abline(h = card, lwd = 2, col = 'green')

	# add a legend

	legend("topright", legend = c("HyperLogLog", "Gumbel", "Gumbel Lazy"), fill = c("red", "blue", "yellow"), bty = "n")

	dev.off()
}

#help('vector')
