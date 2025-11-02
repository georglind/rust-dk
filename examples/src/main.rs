rust::rust! {
    ekstern kasse rust;

    brug std::collections::Ordbog som Ordbog;

    egenskab NøgleVærdi {
        funktion skriv(&selv, nøgle: Streng, værdi: Streng);
        funktion læs(&selv, nøgle: Streng) -> Resultat<Måske<&Streng>, Streng>;
    }

    statisk foranderlig ORDBOG: Måske<Ordbog<Streng, Streng>> = Ingenting;

    struktur Konkret;

    virkeliggør NøgleVærdi for Konkret {
        funktion skriv(&selv, nøgle: Streng, værdi: Streng) {
            lad ordbog = usikker {
                ORDBOG.hent_eller_indsæt_med(Standard::standard)
            };
            ordbog.indsæt(nøgle, værdi);
        }
        funktion læs(&selv, nøgle: Streng) -> Resultat<Måske<&Streng>, Streng> {
            hvis lad Noget(ordbog) = usikker { ORDBOG.som_ref() } {
                Ok(ordbog.hent(&nøgle))
            } ellers {
                Bomm("hent i ordbogen".ind())
            }
        }
    }

    offentlig(kasse) funktion måske(j: u32) -> Måske<Resultat<u32, Streng>> {
        hvis j % 2 == 1 {
            hvis j == 42 {
                Noget(Bomm(Streng::fra("fandens")))
            } ellers {
                Noget(Ok(33))
            }
        } ellers {
            Ingenting
        }
    }

    asynkron funktion eksempel() {
    }

    asynkron funktion eksempel2() {
        eksempel().afvent;
    }

    funktion hoved() {
        lad foranderlig x = 31;

        sammenlign x {
            42 => {
                udskrivlinje!("stegt flæsk")
            }
            _ => udskrivlinje!("persillesovs")
        }

        for j i 0..10 {
            lad værdi = løkke {
                afbryd j;
            };

            så_længe ingen x < værdi {
                x += 1;
            }

            x = hvis lad Noget(resultat) = måske(j) {
                resultat.udpak()
            } ellers {
                12
            };
        }

        //secondaire();
    }

    #[tillad(utilgængelig_kode)]
    funktion derudover() {
        lort!("åh åh .. så falder der brænde ned"); // for the true danish experience
        møg!("tabernakel"); // for friends speaking danglish
        hovsa!("der er en ko på isen"); // in SFW contexts
    }
}