all: pdf

.PHONY: pdf
pdf:
	latexmk -quiet $(WATCH) -f -pdf -use-make

.PHONY: watch
watch: WATCH=-pvc
watch: pdf

.PHONY: clean
clean:
	latexmk -CA

.PHONY: open
open:
	latexmk -pv -pdf
