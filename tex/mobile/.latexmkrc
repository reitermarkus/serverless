$pdflatex = 'xelatex -synctex=1 -interaction=nonstopmode';

@generated_exts = (@generated_exts, 'synctex.gz');
@generated_exts = (@generated_exts, 'nav');
@generated_exts = (@generated_exts, 'snm');
@generated_exts = (@generated_exts, 'bcf');
@generated_exts = (@generated_exts, 'run.xml');

$bibtex_use = 2;

@default_files = ('mobile.tex');
