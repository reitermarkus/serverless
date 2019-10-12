$xelatex = 'xelatex -synctex=1 -interaction=nonstopmode %O %S';
$pdf_mode = 5;

push @generated_exts, 'nav';
push @generated_exts, 'snm';
push @generated_exts, 'xdv';

$bibtex_use = 2;

@default_files = ('presentation/presentation.tex', 'thesis/thesis.tex');
