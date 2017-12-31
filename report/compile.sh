rm *.bbl *.blg *.log *.aux
latex OS_report.tex
bibtex OS_report.aux
latex OS_report.tex
latex OS_report.tex
rm *.bbl *.blg *.log *.aux
dvips OS_report.dvi
ps2pdf OS_report.ps
rm *.dvi *.ps
mv OS_report.pdf ../
xdg-open ../OS_report.pdf
clear
