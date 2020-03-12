$xelatex = 'xelatex -shell-escape -synctex=1 -interaction=nonstopmode %O %S';
$pdf_mode = 5;

push @generated_exts, 'nav';
push @generated_exts, 'snm';
push @generated_exts, 'xdv';
push @generated_exts, 'lol';
push @generated_exts, 'locode';
push @generated_exts, 'run.xml';

$bibtex_use = 2;

@default_files = (
  'initial-presentation/initial-presentation.tex',
  'final-presentation/final-presentation.tex',
  'thesis/thesis.tex'
);
