\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{DPSbachelor}[2007/09/19 DPS bachelor theses document class]
\newif\if@mainmatter\@mainmattertrue
\LoadClass[a4paper,12pt,abstracton,parskip,twoside]{scrreprt}
\raggedbottom
\RequirePackage[english]{babel}
\RequirePackage{a4}
\RequirePackage{graphicx}
\RequirePackage{listings}
\RequirePackage{setspace}
\RequirePackage{scrpage2}
\RequirePackage{amsmath}
\RequirePackage{amssymb}
\RequirePackage{amsthm}
\RequirePackage{floatflt}
\RequirePackage[%
  bookmarks,%
  plainpages=false,%
  pdfpagelabels,%
  colorlinks=false,%
  pdfborder={0 0 0},%
]{hyperref}
\DeclareOption{english}{\AtBeginDocument{\selectlanguage{english}}}
\ProcessOptions\relax

\RequirePackage{caption}
\DeclareCaptionType{code}[Code Listing][List of Code Listings]

\AtBeginDocument{%
  \global\newbox\absbox%
  \renewcommand\abstractname{Abstract}
  \newcommand\ackname{Acknowledgments}
  \definetoday
}

\newcommand\definetoday{%
  \renewcommand*\today{
      \number\day\space \ifcase\month\or%
        January\or%
        February\or%
        March\or%
        April\or%
        May\or%
        June\or%
        July\or%
        August\or%
        September\or%
        October\or%
        November\or%
        December\or%
      \fi\space \number\year
  }
}

\newcommand\@bibfile{biblio}
\newcommand\bibfile[1]{\renewcommand\@bibfile{#1}}
\newcommand*\@mailaddress\@empty
\global\let\@matriculationnumber\@empty
\newcommand*\mailaddress[1]{\gdef\@mailaddress{#1}}
\newcommand*\matriculationnumber[1]{\gdef\@matriculationnumber{#1}}
\newcommand*\@plaintitle\@empty
\newcommand*\@plainauthor\@empty
\newcommand*\plaintitle[1]{\gdef\@plaintitle{#1}}
\newcommand*\plainauthor[1]{\gdef\@plainauthor{#1}}

\let\@OLDappendix\appendix
\renewcommand\appendix{%
  \backmatter
  \phantomsection{\addcontentsline{toc}{chapter}{\bibname}}
  % \bibliographystyle{plain}
  % \bibliography{\@bibfile}
  \@OLDappendix
}

\newcommand\kindname{Bachelor~Thesis}
\newcommand\supervisorname{Supervisor}
\newcommand\university{Leopold--Franzens--University \\ Innsbruck \\ }
\newcommand\institute{Institute of Computer Science \\ Research Group DPS \\ (Distributed and Parallel Systems) \\}

\subject{\kindname}

\newcommand{\supervisor}[1]{%
  \def\@supervisor{#1}
  \publishers{\textbf{\supervisorname:} #1}
}

\renewcommand\abstract[1]{%
  \global\setbox\absbox=\hbox{#1}%
}

\global\let\@acknowledgments\@empty

\newcommand\acknowledgments[1]{\def\@acknowledgments{#1}}

\newcommand*\frontmatter{%
  \if@twoside\cleardoublepage\else\clearpage\fi
  \@mainmatterfalse\pagenumbering{roman}%
}

\newcommand*\mainmatter{%
  \if@twoside\cleardoublepage\else\clearpage\fi
  \@mainmattertrue\pagenumbering{arabic}%
}

\newcommand*\backmatter{%
\if@openright\cleardoublepage\else\clearpage\fi
  \@mainmatterfalse%
}

\renewcommand*\maketitle[1][-1]{{
  \frontmatter
  \thispagestyle{empty}
  \let\footnotesize\small
  \let\footnoterule\relax
  \let\footnote\thanks
  \renewcommand*\thefootnote{\@fnsymbol\c@footnote}
  \let\@oldmakefnmark\@makefnmark
  \renewcommand*{\@makefnmark}{\rlap\@oldmakefnmark}

  \begin{floatingfigure}[l]{35mm}
    \includegraphics[width=10em]{uibk-logo.pdf}
  \end{floatingfigure}
  \ \vspace{-2.4em}

  \begin{flushright}
    {\normalfont \sffamily
      {\LARGE \university}
      \vspace{\stretch{1}}
      {\textbf {\large \institute \ }}
      \vspace{\stretch{2}}
      {\LARGE \\ \textbf{\@title} \\ \ }
      \vspace{\stretch{1}}
      {\textbf {\large \\ \kindname \\ \ }}
      {\large \\ \@publishers \\ \ }
      \vspace{\stretch{1}}
      {\bfseries
        {\large \\ \@author \\
          \ifx\@matriculationnumber\@empty{}\else{\@matriculationnumber}\fi
        \ }
      }
      \vspace{\stretch{1}}
      {\large \\ Innsbruck \\ \today{} \\ \ }
    }
  \end{flushright}

  \setcounter{footnote}{0}%
  \global\let\thanks\relax
  \global\let\maketitle\relax
  \global\let\@thanks\@empty
  \global\let\@author\@empty
  \global\let\@date\@empty
  \global\let\@title\@empty
  \global\let\@extratitle\@empty
  \global\let\@titlehead\@empty
  \global\let\@subject\@empty
  \global\let\@publishers\@empty
  \global\let\@uppertitleback\@empty
  \global\let\@lowertitleback\@empty
  \global\let\@dedication\@empty
  \global\let\author\relax
  \global\let\title\relax
  \global\let\extratitle\relax
  \global\let\titlehead\relax
  \global\let\subject\relax
  \global\let\publishers\relax
  \global\let\uppertitleback\relax
  \global\let\lowertitleback\relax
  \global\let\dedication\relax
  \global\let\date\relax
  \global\let\and\relax
}}

\let\@OLDtableofcontents\tableofcontents
\renewcommand\tableofcontents{%
  \pagestyle{empty}
  \cleardoublepage
  \begin{center}
  \normalfont\sectfont\nobreak\abstractname
  \@endparpenalty\@M
  \end{center}
  \unhbox\absbox
  \par\vfil\null
  \ifx\@acknowledgments\@empty{}\else
    \cleardoublepage
    \chapter*{\ackname}
    \@acknowledgments
    \cleardoublepage
  \fi
  \@OLDtableofcontents
  \listoffigures
  \listoftables
  \listofcodes
  \cleardoublepage
  \cleardoublepage
  \input{preface.tex}
  \mainmatter
  \pagestyle{scrheadings}
  \setheadsepline{0.5pt}
  \automark[section]{chapter}
  \onehalfspacing
}
\endinput
