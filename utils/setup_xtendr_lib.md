PKG=thingdoer

## in R interactive:
usethis::create_package('${PKG}', fields = list(), rstudio = FALSE, roxygen = TRUE, check_name = FALSE, open = FALSE);


Rscript -e "setwd('${PKG}'); rextendr::use_extendr('${PKG}', crate_name = '${PKG}', lib_name = '${PKG}', quiet = TRUE, overwrite = TRUE); rextendr::document()"


## test if works

Rscript -e "devtools::load_all('${PKG}'); hello_world()"
